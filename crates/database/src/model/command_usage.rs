use crate::DatabaseHandler;

/// Represents command usage statistics.
#[derive(sqlx::FromRow)]
pub struct CommandUsage {
    /// The name of the command.
    pub command_name: String,
    /// The number of times the command has been used.
    pub uses: i32,
}

impl CommandUsage {
    /// Retrieves command usage statistics for all commands, ordered by usage count in descending order.
    ///
    /// # Arguments
    ///
    /// * `handler`: A reference to the `DatabaseHandler` for database interaction.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec<CommandUsage>` with usage stats for all commands, or an `sqlx::Error` on failure.
    pub async fn get_command_usage_stats(
        handler: &DatabaseHandler,
    ) -> Result<Vec<Self>, sqlx::Error> {
        // SQL query to select all command usage stats, ordered by usage count descending
        let query = "SELECT * FROM command_uses order by uses desc";
        sqlx::query_as::<_, Self>(query)
            .fetch_all(&handler.pool)
            .await
    }

    /// Retrieves command usage statistics for a specific command.
    ///
    /// # Arguments
    ///
    /// * `handler`: A reference to the `DatabaseHandler` for database interaction.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `CommandUsage` struct for the specified command, or an `sqlx::Error` if not found or on failure.
    pub async fn get_command_usage_stats_for(
        &self,
        handler: &DatabaseHandler,
    ) -> Result<Self, sqlx::Error> {
        // SQL query to select usage stats for a specific command
        let query = "SELECT * FROM command_uses where command_name = $1 order by uses desc";
        sqlx::query_as::<_, Self>(query)
            .bind(&self.command_name)
            .fetch_one(&handler.pool)
            .await
    }

    /// Increments the usage count for a specific command in the database.
    ///
    /// If the command does not exist, it will be inserted with a usage count of 1.
    ///
    /// # Arguments
    ///
    /// * `handler`: A reference to the `DatabaseHandler` for database interaction.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `sqlx::Error` on failure.
    pub async fn increment_command_uses(
        &self,
        handler: &DatabaseHandler,
    ) -> Result<(), sqlx::Error> {
        // SQL query to increment command usage count, inserting if command doesn't exist
        let query = "insert into command_uses (command_name, uses) values ($1, 1) on conflict (command_name) do update set uses = command_uses.uses + 1 where command_uses.command_name = $1;";
        sqlx::query(query)
            .bind(&self.command_name)
            .execute(&handler.pool)
            .await?;
        Ok(())
    }
}
