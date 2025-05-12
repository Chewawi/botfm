use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use database::DatabaseHandler;
use lastfm::LastFmClient;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = lumi::Context<'a, Data, Error>;
pub type PrefixContext<'a> = lumi::PrefixContext<'a, Data, Error>;
pub type FrameworkContext<'a> = lumi::FrameworkContext<'a, Data, Error>;
pub type Command = lumi::Command<Data, Error>;

#[derive()]
pub struct Data {
    /// If the bots startup has been handled in the `on_ready` event.
    pub has_started: AtomicBool,
    /// Time the bot started.
    pub time_started: std::time::Instant,
    /// Http client.
    pub http_client: reqwest::Client,
    /// Wrapper for the bot database with helper functions.
    pub db: Arc<DatabaseHandler>,
    pub lastfm: Arc<LastFmClient>,
}