// See config.template.toml for information on the variables here.
// stealed from assyst owo

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bot_id: u64,
    pub api: API,
    pub authentication: Authentication,
    pub database: Database,
    pub cache: Cache,
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
pub struct API {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct Cache {
    pub host: String,
    pub token: String,
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
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    #[must_use]
    pub fn to_url_safe(&self) -> String {
        let labels: Vec<&str> = self.host.split('.').collect();

        let masked_host = match labels.len() {
            n if n > 2 => {
                let mut masked = vec!["***"; n - 2];
                masked.extend_from_slice(&labels[n - 2..]);
                masked.join(".")
            }
            2 => {
                format!("***.{}", labels[1])
            }
            _ => "***".to_string(),
        };

        format!(
            "postgres://{}@{}:{}/{}",
            self.username, masked_host, self.port, self.database
        )
    }
}

#[derive(Deserialize)]
pub struct Prefixes {
    pub default: String,
    pub development: String,
}

impl Prefixes {
    #[must_use]
    pub fn get(&self) -> String {
        match std::env::var("RUST_ENV") {
            Ok(env) => match env.as_str() {
                "production" => self.default.clone(),
                _ => self.development.clone(),
            },
            Err(_) => self.development.clone(),
        }
    }
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
