#![feature(trait_alias)]

use std::borrow::Cow;

use reqwest::Client;
use server_cache::ServerCache;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

mod cache;
pub mod model;
mod server_cache;

// Define the maximum number of database connections
static MAX_CONNECTIONS: u32 = 1;

/// Represents a count from the database, typically used for aggregate queries.
#[derive(sqlx::FromRow, Debug)]
pub struct Count {
    pub count: i64,
}

/// Represents the database size, typically returned from a size query.
#[derive(sqlx::FromRow, Debug)]
pub struct DatabaseSize {
    pub size: String,
}

/// Database handler providing connection and data access methods.
///
/// This struct manages the database connection pool and provides methods
/// to interact with the database, including caching frequently accessed data.
pub struct DatabaseHandler {
    /// Connection pool for PostgreSQL database.
    pool: PgPool,
    /// Server-based cache for database data.
    pub cache: ServerCache,
}

impl DatabaseHandler {
    /// Creates a new `DatabaseHandler` instance and establishes a database connection.
    ///
    /// # Arguments
    ///
    /// * `url`: The connection string for the database, including sensitive information.
    /// * `safe_url`: A sanitized version of the connection string for logging purposes.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `DatabaseHandler` instance or an error if connection fails.
    pub async fn new(url: String, safe_url: String) -> anyhow::Result<Self> {
        info!(
            "Connecting to database on {} with {} max connections",
            safe_url, MAX_CONNECTIONS
        );

        // Create a new connection pool with the specified maximum connections
        let pool = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&url)
            .await?;

        info!("Connected to database on {}", safe_url);
        // Initialize the server cache
        let cache = ServerCache::new(Client::new());
        Ok(Self { pool, cache })
    }

    /// Fetches the size of the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `DatabaseSize` struct with the database size, or an error.
    pub async fn database_size(&self) -> anyhow::Result<DatabaseSize> {
        // SQL query to get the human-readable size of the 'assyst' database
        let query = r"SELECT pg_size_pretty(pg_database_size('assyst')) as size";

        Ok(sqlx::query_as::<_, DatabaseSize>(query)
            .fetch_one(&self.pool)
            .await?)
    }
}

/// Checks if a given `sqlx::Error` is a unique constraint violation error.
///
/// This function is useful for identifying errors when inserting data that violates unique constraints.
///
/// # Arguments
///
/// * `error`: A reference to the `sqlx::Error` to be checked.
///
/// # Returns
///
/// `true` if the error is a unique constraint violation, `false` otherwise.
pub(crate) fn _is_unique_violation(error: &sqlx::Error) -> bool {
    // Define the SQLSTATE code for unique constraint violation
    const UNIQUE_CONSTRAINT_VIOLATION_CODE: Cow<'_, str> = Cow::Borrowed("23505");
    error
        .as_database_error()
        .and_then(sqlx::error::DatabaseError::code)
        == Some(UNIQUE_CONSTRAINT_VIOLATION_CODE)
}
