use crate::core::structs::Data;
use atomic_time::AtomicInstant;
use common::config::CONFIG;
use database::DatabaseHandler;
use lastfm::LastFmClient;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

pub async fn setup() -> Arc<Data> {
    let http_client = reqwest::Client::new();
    let db = Arc::new(
        DatabaseHandler::new(CONFIG.database.to_url(), CONFIG.database.to_url_safe())
            .await
            .unwrap(),
    );
    let lastfm = Arc::new(
        LastFmClient::new(http_client.clone(), db.clone())
            .await
            .unwrap(),
    );

    Arc::new(Data {
        has_started: AtomicBool::new(false),
        time_started: std::time::Instant::now(),
        command_started: AtomicInstant::new(Instant::now()),
        http_client,
        db,
        lastfm,
    })
}
