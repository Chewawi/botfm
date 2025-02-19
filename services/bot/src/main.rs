use actix_web::{web, App, HttpServer, Responder};
use anyhow::Result;

use common::config::CONFIG;
use common::utils::tracing_init;

use database::DatabaseHandler;
use lastfm::LastFmClient;
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use std::sync::Arc;

mod core;

mod commands;

#[derive(Clone)]
struct Data {
    http_client: reqwest::Client,
    db: Arc<DatabaseHandler>,
    lastfm: Arc<LastFmClient>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_init();

    let intents = serenity::GatewayIntents::all();

    let http_client = reqwest::Client::new();
    let db = Arc::new(
        DatabaseHandler::new(CONFIG.database.to_url(), CONFIG.database.to_url_safe()).await?,
    );
    let lastfm = Arc::new(LastFmClient::new(http_client.clone(), db.clone()).await?);

    let data = Arc::new(Data {
        http_client,
        db,
        lastfm: lastfm.clone(),
    });

    let framework = {
        let data_clone_for_framework = data.clone();

        poise::Framework::builder()
            .options(poise::FrameworkOptions {
                prefix_options: PrefixFrameworkOptions {
                    prefix: Some("!".to_string()),
                    mention_as_prefix: true,
                    case_insensitive_commands: true,
                    ..Default::default()
                },

                commands: commands::register_all_commands(),

                ..Default::default()
            })
            .setup(move |ctx, _ready, framework| {
                let data_clone_for_setup = data_clone_for_framework.clone();
                Box::pin(async move {
                    println!("Registering commands...");
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                    println!("Ready as {}!", _ready.user.name);
                    Ok((*data_clone_for_setup).clone())
                })
            })
            .build()
    };

    let data_for_server = data.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data_for_server.lastfm.clone()))
            .configure(api::config)
    })
    .bind("127.0.0.1:8080")?
    .run();

    let mut serenity_client =
        serenity::ClientBuilder::new(&CONFIG.authentication.discord_token, intents)
            .framework(framework)
            .await?;

    tokio::select! {
        _ = server => {
            println!("Actix server stopped.");
            Ok(())
        },
        result = serenity_client.start() => {
            println!("Serenity client stopped.");
            result.map_err(|e| e.into())
        }
    }
}
