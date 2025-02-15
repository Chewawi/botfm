// See config.template.toml for information on the variables here.
// stealed from assyst owo

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bot_id: u64,
    pub authentication: Authentication,
    pub database: Database,
    pub prefix: Prefixes,
    pub logging_webhooks: LoggingWebhooks,
}

#[derive(Deserialize)]
pub struct Authentication {
    pub discord_token: String,
    pub lastfm_key: String,
    pub lastfm_secret: String,
    pub lastfm_redirect_uri: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub host: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub port: u16,
}
impl Database {
    #[must_use]
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.username, self.password, self.host, self.database
        )
    }

    #[must_use]
    pub fn to_url_safe(&self) -> String {
        let mut host = self.host.split('.').take(2).collect::<Vec<_>>();
        host.push("###");
        host.push("###");

        format!(
            "postgres://{}@{}/{}",
            self.username,
            &host.join("."),
            self.database
        )
    }
}

#[derive(Deserialize)]
pub struct Prefixes {
    pub default: String,
}

#[derive(Deserialize)]
pub struct LoggingWebhooks {
    pub panic: LoggingWebhook,
    pub error: LoggingWebhook,
    pub enable_webhooks: bool,
}

#[derive(Deserialize, Clone)]
pub struct LoggingWebhook {
    pub token: String,
    pub id: u64,
}
