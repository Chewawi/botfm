use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lastfm::LastFmClient;
use std::sync::Arc;
use common::config::CONFIG;
use common::utils::tracing_init;
use database::DatabaseHandler;

#[derive(serde::Deserialize)]
struct CallbackQuery {
    token: String,
}

async fn lastfm_callback_handler(
    lastfm_client: web::Data<Arc<LastFmClient>>,
    query: web::Query<CallbackQuery>,
    path: web::Path<u64>,
) -> impl Responder {
    let user_id = path.into_inner();
    let token = query.token.clone();

    println!("{}, {}", user_id, token);

    match lastfm_client.handle_callback(&token, user_id).await {
        Ok(_) => HttpResponse::Ok().body("¡Auth completed!"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/lastfm_callback/{user_id}").route(web::get().to(lastfm_callback_handler)),
    );
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_init();

    let http_client = reqwest::Client::new();
    let db = Arc::new(
        DatabaseHandler::new(CONFIG.database.to_url(), CONFIG.database.to_url_safe()).await.unwrap(),
    );
    let lastfm = Arc::new(LastFmClient::new(http_client.clone(), db.clone()).await.unwrap());

    let data = web::Data::new(lastfm);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(config)
    })
        .bind(("0.0.0.0", CONFIG.api.port))?
        .run()
        .await
}
