use serde::{Serialize, Deserialize};
use crate::DatabaseHandler;

/// Represents a Last.fm session for a user.
///
/// Stores user ID, Last.fm username, session key, and token for API interactions.
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Lastfm {
    /// Discord user ID associated with the Last.fm session.
    pub user_id: i64,
    /// Last.fm username.
    pub lastfm_username: String,
    /// Last.fm session key.
    pub session_key: String,
    /// Last.fm API token.
    pub token: String,
}

impl Lastfm {
    /// Sets or updates a Last.fm session for a user in the database and cache.
    ///
    /// If a session already exists for the user, it will be updated.
    ///
    /// # Arguments
    ///
    /// * `handler`: A reference to the `DatabaseHandler` for database interaction.
    /// * `user_id`: The Discord user ID.
    /// * `lastfm_username`: The Last.fm username.
    /// * `session_key`: The Last.fm session key.
    /// * `token`: The Last.fm API token.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error.
    pub async fn set(
        &self,
        handler: &DatabaseHandler,
        user_id: u64,
        lastfm_username: String,
        session_key: String,
        token: String,
    ) -> anyhow::Result<()> {
        // SQL query to insert or update Last.fm session data
        let query = r"INSERT INTO lastfm_sessions (user_id, lastfm_username, session_key, token)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id) DO UPDATE
            SET lastfm_username = $2, session_key = $3, token = $4";

        sqlx::query(query)
            .bind(user_id as i64)
            .bind(lastfm_username)
            .bind(session_key)
            .bind(token)
            .execute(&handler.pool)
            .await?;

        // Update the cache with the new session data
        handler.cache.set_session(user_id, self).await?;

        Ok(())
    }

    /// Retrieves a Last.fm session for a user from the cache or database.
    ///
    /// First checks the cache. If not found, it queries the database and updates the cache.
    ///
    /// # Arguments
    ///
    /// * `handler`: A reference to the `DatabaseHandler` for database interaction.
    /// * `user_id`: The Discord user ID to retrieve the session for.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Lastfm>`. `Some(Lastfm)` if found, `None` if not found, or an error.
    pub async fn get(handler: &DatabaseHandler, user_id: u64) -> anyhow::Result<Option<Self>> {
        // Check if the session is already in the cache
        if let Some(session) = handler.cache.get_session(user_id).await? {
            return Ok(Some(session));
        }

        // SQL query to select Last.fm session data by user ID
        let query = "SELECT * FROM lastfm_sessions WHERE user_id = $1";

        match sqlx::query_as::<_, Lastfm>(query)
            .bind(user_id as i64) // Bind user ID as i64 for SQL query
            .fetch_one(&handler.pool)
            .await
        {
            Ok(session) => {
                // Update the cache with the fetched session data
                handler.cache.set_session(user_id, &session).await?;
                Ok(Some(session))
            }
            Err(sqlx::Error::RowNotFound) => Ok(None), // Return None if no session found in the database
            Err(err) => Err(err.into()), // Propagate other errors
        }
    }

}
