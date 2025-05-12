use std::str::FromStr;
use common::config::CONFIG;
use common::utils::tracing_init;

use lumi::{serenity_prelude as serenity, PrefixFrameworkOptions};
use std::sync::Arc;
use std::time::Duration;

mod commands;
mod core;

#[tokio::main]
async fn main() {
    tracing_init();
    
    let data = core::data::setup().await;

    let framework = lumi::Framework::new(lumi::FrameworkOptions {
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(CONFIG.prefix.get().into()),
            mention_as_prefix: true,
            case_insensitive_commands: true,
            edit_tracker: Some(Arc::new(lumi::EditTracker::for_timespan(
                Duration::from_secs(600),
            ))),
            ..Default::default()
        },
        commands: commands::register_all_commands(),
        ..Default::default()
    });
    
    let mut settings = serenity::Settings::default();
    settings.max_messages = 1000;

    let intents = serenity::GatewayIntents::all();
    let token = serenity::Token::from_str(&CONFIG.authentication.discord_token)
        .expect("Missing TOKEN in config.toml");
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .data(data)
        .cache_settings(settings)
        .await
        .unwrap();
    
    client.start().await.unwrap();
}