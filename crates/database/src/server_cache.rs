use crate::model::lastfm::Lastfm;
use crate::model::prefix::Prefix;
use anyhow::Result;
use common::config::CONFIG;
use md5;
use mini_moka::sync::Cache;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::time::Duration;

// Define traits for cacheable values and keys
trait TCacheV = Send + Sync + Clone + 'static;
trait TCacheK = Hash + Send + Sync + Eq + Clone + 'static;

/// Creates a default cache with a maximum capacity of 10,000 entries and a time-to-idle of 10 minutes.
fn default_cache<K: TCacheK, V: TCacheV>() -> Cache<K, V> {
    Cache::builder()
        .max_capacity(10000)
        .time_to_idle(Duration::from_secs(60 * 10))
        .build()
}

/// Client for interacting with the server cache API.
pub struct ServerCache {
    /// HTTP client for making requests to the cache server.
    client: Client,
    /// Base URL of the cache server.
    base_url: String,
    /// Authentication token for the cache server.
    token: String,
    /// In-memory cache for prefixes
    prefixes_cache: Cache<u64, Prefix>,
    /// In-memory cache for sessions
    sessions_cache: Cache<u64, Lastfm>,
    /// In-memory cache for colors
    colors_cache: Cache<String, Vec<u8>>,
}

impl ServerCache {
    /// Creates a new `ServerCache` instance.
    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: format!("https://{}/store", CONFIG.cache.host),
            token: CONFIG.cache.token.clone(),
            prefixes_cache: default_cache(),
            sessions_cache: default_cache(),
            colors_cache: default_cache(),
        }
    }

    /// Sanitizes a key for use with the cache server.
    ///
    /// The server requires that `/` characters be replaced with `:`.
    fn sanitize_key(key: &str) -> String {
        key.replace('/', ":")
    }

    /// Retrieves a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized value if found, or an error.
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        let sanitized_key = Self::sanitize_key(key);
        let url = format!("{}/{}", self.base_url, sanitized_key);

        let response = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json::<T>().await?;
            Ok(Some(value))
        } else if response.status().as_u16() == 404 {
            Ok(None)
        } else {
            Err(anyhow::anyhow!("Failed to get value from cache: {}", response.status()))
        }
    }

    /// Stores a value in the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to store the value under.
    /// * `value` - The value to store.
    /// * `upsert` - Whether to update the value if it already exists.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, upsert: bool) -> Result<()> {
        let sanitized_key = Self::sanitize_key(key);
        let url = if upsert {
            format!("{}/{}!", self.base_url, sanitized_key)
        } else {
            format!("{}/{}", self.base_url, sanitized_key)
        };

        let response = self.client
            .put(&url)
            .header("Authorization", &self.token)
            .header("Content-Type", "application/json")
            .json(value)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to set value in cache: {}", response.status()))
        }
    }

    /// Deletes a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn delete(&self, key: &str) -> Result<()> {
        let sanitized_key = Self::sanitize_key(key);
        let url = format!("{}/{}", self.base_url, sanitized_key);

        let response = self.client
            .delete(&url)
            .header("Authorization", &self.token)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to delete value from cache: {}", response.status()))
        }
    }

    /// Retrieves a prefix from the cache by guild ID.
    pub async fn get_prefix(&self, guild_id: u64) -> Result<Option<Prefix>> {
        // Check in-memory cache first
        if let Some(prefix) = self.prefixes_cache.get(&guild_id) {
            return Ok(Some(prefix));
        }

        // If not in memory, try to get from the server cache
        match self.get::<Prefix>(&format!("prefix/{}", guild_id)).await {
            Ok(Some(prefix)) => {
                // Store in memory for future requests
                self.prefixes_cache.insert(guild_id, prefix.clone());
                Ok(Some(prefix))
            }
            other => other,
        }
    }

    /// Sets a prefix in the cache for a given guild ID.
    pub async fn set_prefix(&self, guild_id: u64, prefix: &Prefix) -> Result<()> {
        // Update in-memory cache
        self.prefixes_cache.insert(guild_id, prefix.clone());

        // Update server cache
        self.set(&format!("prefix/{}", guild_id), prefix, true).await
    }

    /// Retrieves a lastfm session from the cache by user ID.
    pub async fn get_session(&self, user_id: u64) -> Result<Option<Lastfm>> {
        // Check in-memory cache first
        if let Some(session) = self.sessions_cache.get(&user_id) {
            return Ok(Some(session));
        }

        // If not in memory, try to get from the server cache
        match self.get::<Lastfm>(&format!("session/{}", user_id)).await {
            Ok(Some(session)) => {
                // Store in memory for future requests
                self.sessions_cache.insert(user_id, session.clone());
                Ok(Some(session))
            }
            other => other,
        }
    }

    /// Sets a lastfm session in the cache for a given user ID.
    pub async fn set_session(&self, user_id: u64, session: &Lastfm) -> Result<()> {
        // Update in-memory cache
        self.sessions_cache.insert(user_id, session.clone());

        // Update server cache
        self.set(&format!("session/{}", user_id), session, true).await
    }

    /// Retrieves an image color from the cache by image URL.
    pub async fn get_image_color(&self, image_url: &str) -> Result<Option<Vec<u8>>> {
        // Check in-memory cache first using the full URL as a key
        let url_string = image_url.to_string();
        if let Some(color) = self.colors_cache.get(&url_string) {
            return Ok(Some(color));
        }

        // If not in memory, try to get from the server cache
        let hash = format!("{:x}", md5::compute(image_url));
        match self.get::<Vec<u8>>(&format!("color/{}", hash)).await {
            Ok(Some(color)) => {
                // Store in memory for future requests
                self.colors_cache.insert(url_string, color.clone());
                Ok(Some(color))
            }
            other => other,
        }
    }

    /// Sets an image color in the cache for a given image URL.
    pub async fn set_image_color(&self, image_url: &str, colors: &Vec<u8>) -> Result<()> {
        // Update in-memory cache
        self.colors_cache.insert(image_url.to_string(), colors.clone());

        // Update server cache
        let hash = format!("{:x}", md5::compute(image_url));
        self.set(&format!("color/{}", hash), colors, true).await
    }
}
