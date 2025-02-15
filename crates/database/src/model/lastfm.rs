use crate::DatabaseHandler;
use sqlx::postgres::PgRow;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Lastfm {
    pub user_id: i64,
    pub lastfm_username: String,
    pub session_key: String,
    pub token: String,
}

impl Lastfm {
    pub async fn set(
        &self,
        handler: &DatabaseHandler,
        user_id: u64,
        lastfm_username: String,
        session_key: String,
        token: String,
    ) -> anyhow::Result<()> {
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

        handler.cache.set_session(user_id, self.clone());

        Ok(())
    }

    pub async fn get(handler: &DatabaseHandler, user_id: u64) -> anyhow::Result<Option<Self>> {
        if let Some(session) = handler.cache.get_session(user_id) {
            return Ok(Some(session));
        }

        let query = "SELECT user_id, lastfm_username, session_key, token FROM lastfm_sessions WHERE user_id = $1";

        match sqlx::query_as::<_, Lastfm>(query)
            .bind(user_id as i64) // Bind as i64
            .fetch_one(&handler.pool)
            .await
        {
            Ok(session) => {
                handler.cache.set_session(user_id, session.clone());
                Ok(Some(session))
            }
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    // pub async fn _load(_handler: &DatabaseHandler) -> Result<(), sqlx::Error> {
    // stfu
    // }
}
