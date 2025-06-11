use common::config::CONFIG;
use common::utils::tracing_init;
use std::str::FromStr;

use lumi::serenity_prelude as serenity;

mod commands;
mod core;
mod events;
mod images;

#[tokio::main]
async fn main() {
    tracing_init();

    let data = core::data::setup().await;

    let framework = core::framework::init_framework();

    let mut settings = serenity::Settings::default();
    settings.max_messages = 1000;

    let intents = serenity::GatewayIntents::all();
    let token = serenity::Token::from_str(&CONFIG.authentication.discord_token)
        .expect("Missing TOKEN in config.toml");

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .data(data)
        .cache_settings(settings)
        .event_handler(events::Handler)
        .await
        .unwrap();

    client.start().await.unwrap();
}
