use std::hash::Hash;
use serde::{Serialize, Deserialize};

use crate::DatabaseHandler;

/// Represents a guild-specific command prefix.
///
/// Each guild can have a unique prefix used to invoke bot commands.
#[derive(Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Prefix {
    /// The prefix string itself.
    pub prefix: String,
}

impl Prefix {
    /// Sets the prefix for a guild in the database and updates the cache.
    ///
    /// If a prefix already exists for the guild, it will be updated.
    ///
    /// # Arguments
    ///
    /// * `handler`: A reference to the `DatabaseHandler` for database interaction.
    /// * `guild_id`: The ID of the guild to set the prefix for.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error.
    pub async fn set(&self, handler: &DatabaseHandler, guild_id: u64) -> anyhow::Result<()> {
        // SQL query to insert or update the prefix for a guild
        let query = r"INSERT INTO prefixes(guild, prefix) VALUES($1, $2) ON CONFLICT (guild) DO UPDATE SET prefix = $2 WHERE prefixes.guild = $1";

        sqlx::query(query)
            .bind(guild_id as i64)
            .bind(self.clone().prefix)
            .execute(&handler.pool)
            .await?;

        // Update the cache with the new prefix
        handler.cache.set_prefix(guild_id, self.clone());

        Ok(())
    }

    /// Retrieves the prefix for a guild from the cache or database.
    ///
    /// First, it checks the cache. If not found, it queries the database and updates the cache.
    ///
    /// # Arguments
    ///
    /// * `handler`: A reference to the `DatabaseHandler` for database interaction.
    /// * `guild_id`: The ID of the guild to retrieve the prefix for.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Prefix>`. `Some(Prefix)` if found, `None` if not found, or an error.
    pub async fn get(handler: &DatabaseHandler, guild_id: u64) -> anyhow::Result<Option<Self>> {
        // Check if the prefix is already in the cache
        if let Some(prefix) = handler.cache.get_prefix(guild_id) {
            return Ok(Some(prefix));
        }

        // SQL query to select the prefix for a given guild ID
        let query = "SELECT * FROM prefixes WHERE guild = $1";

        match sqlx::query_as::<_, (String,)>(query)
            .bind(guild_id as i64)
            .fetch_one(&handler.pool)
            .await
        {
            Ok(res) => {
                // Create a Prefix struct from the database result
                let prefix = Prefix { prefix: res.0 };
                // Update the cache with the fetched prefix
                handler.cache.set_prefix(guild_id, prefix.clone());
                Ok(Some(prefix))
            },
            Err(sqlx::Error::RowNotFound) => Ok(None), // Return None if no prefix found in the database
            Err(err) => Err(err.into()), // Propagate other errors
        }
    }

    /// Calculates the size in bytes of the Prefix struct (specifically the prefix string).
    ///
    /// # Returns
    ///
    /// The size of the prefix string in bytes as a `u64`.
    #[must_use] pub fn size_of(&self) -> u64 {
        self.prefix.as_bytes().len() as u64
    }
}