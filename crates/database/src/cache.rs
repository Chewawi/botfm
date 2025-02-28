use std::hash::Hash;
use std::mem::size_of;
use std::time::Duration;
use std::u64;
use mini_moka::sync::Cache;

use crate::model::lastfm::Lastfm;
use crate::model::prefix::Prefix;

// Define traits for cacheable values and keys
trait TCacheV = Send + Sync + Clone + 'static;
trait TCacheK = Hash + Send + Sync + Eq + Clone + 'static;

/// Creates a default cache with a maximum capacity of 1000 entries and a time-to-idle of 3 minutes.
///
/// # Type Parameters
///
/// * `K`: The type of the cache key, must implement `TCacheK`.
/// * `V`: The type of the cache value, must implement `TCacheV`.
///
/// # Returns
///
/// A `mini_moka::sync::Cache` instance with default settings.
fn default_cache<K: TCacheK, V: TCacheV>() -> Cache<K, V> {
    Cache::builder()
        .max_capacity(1000)
        .time_to_idle(Duration::from_secs(60 * 3))
        .build()
}

/// Creates a sized cache with a custom maximum capacity and a default time-to-idle of 3 minutes.
///
/// # Type Parameters
///
/// * `K`: The type of the cache key, must implement `TCacheK`.
/// * `V`: The type of the cache value, must implement `TCacheV`.
///
/// # Arguments
///
/// * `size`: The maximum capacity of the cache.
///
/// # Returns
///
/// A `mini_moka::sync::Cache` instance with the specified size and default idle time.
fn default_cache_sized<K: TCacheK, V: TCacheV>(size: u64) -> Cache<K, V> {
    Cache::builder()
        .max_capacity(size)
        .time_to_idle(Duration::from_secs(60 * 3))
        .build()
}

/// In-memory cache collection for frequently accessed database data.
///
/// This struct holds caches for prefixes and lastfm sessions to reduce database load.
pub struct DatabaseCache {
    /// Cache for guild prefixes, keyed by guild ID.
    prefixes: Cache<u64, Prefix>,
    /// Cache for lastfm sessions, keyed by user ID.
    sessions: Cache<u64, Lastfm>,
    /// Cache for image colors, keyed by image URL.
    /// This cache is used to store the dominant color of an image for embeds.
    colors: Cache<String, Vec<u8>>,
}

impl DatabaseCache {
    /// Creates a new `DatabaseCache` instance with default sized caches.
    pub fn new() -> Self {
        DatabaseCache {
            // Prefix cache with a capacity of 100,000 entries
            prefixes: default_cache_sized(100000),
            // Session cache with a very large capacity (effectively unbounded for practical purposes)
            sessions: default_cache_sized(u64::MAX),
            // Image color cache with a capacity of 1,000 entries
            colors: default_cache(),
        }
    }

    /// Retrieves a prefix from the cache by guild ID.
    ///
    /// # Arguments
    ///
    /// * `guild_id`: The ID of the guild to retrieve the prefix for.
    ///
    /// # Returns
    ///
    /// An `Option<Prefix>` containing the prefix if found in the cache, or `None` otherwise.
    pub fn get_prefix(&self, guild_id: u64) -> Option<Prefix> {
        self.prefixes.get(&guild_id)
    }

    /// Sets a prefix in the cache for a given guild ID.
    ///
    /// # Arguments
    ///
    /// * `guild_id`: The ID of the guild to set the prefix for.
    /// * `prefix`: The `Prefix` to be cached.
    pub fn set_prefix(&self, guild_id: u64, prefix: Prefix) {
        self.prefixes.insert(guild_id, prefix);
    }

    /// Gets the current size (number of entries) of the prefixes cache.
    ///
    /// # Returns
    ///
    /// The number of entries in the prefixes cache as a `usize`.
    pub fn get_prefixes_cache_size(&self) -> usize {
        self.prefixes.entry_count() as usize
    }

    /// Retrieves a lastfm session from the cache by user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id`: The ID of the user to retrieve the session for.
    ///
    /// # Returns
    ///
    /// An `Option<Lastfm>` containing the session if found in the cache, or `None` otherwise.
    pub fn get_session(&self, user_id: u64) -> Option<Lastfm> {
        self.sessions.get(&user_id)
    }

    /// Sets a lastfm session in the cache for a given user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id`: The ID of the user to set the session for.
    /// * `session`: The `Lastfm` session to be cached.
    pub fn set_session(&self, user_id: u64, session: Lastfm) {
        self.sessions.insert(user_id, session);
    }

    /// Retrieves an image color from the cache by image URL.
    ///
    /// # Arguments
    ///
    /// * `image_url`: The URL of the image to retrieve the color for.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<u8>>` containing the color if found in the cache, or `None` otherwise.
    pub fn get_image_color(&self, image_url: &str) -> Option<Vec<u8>> {
        self.colors.get(&image_url.to_string())
    }

    /// Sets an image color in the cache for a given image URL.
    ///
    /// # Arguments
    ///
    /// * `image_url`: The URL of the image to set the color for.
    /// * `color`: The `Rgba<u8>` color to be cached.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<u8>>` containing the color if found in the cache, or `None` otherwise.
    pub fn set_image_color(&self, image_url: &str, colors: Vec<u8>) {
        self.colors.insert(image_url.to_string(), colors);
    }

    pub fn size_of(&self) -> u64 {
        let mut size = 0;

        // Iterate over each entry in the prefixes cache.
        for prefix in &self.prefixes {
            // Add the size of the key (guild ID - u64).
            size += size_of::<u64>() as u64;
            // Add the size of the value (Prefix).
            size += prefix.value().size_of();
        }

        size
    }
}

impl Default for DatabaseCache {
    /// Creates a default `DatabaseCache` instance.
    fn default() -> Self {
        Self::new()
    }
}