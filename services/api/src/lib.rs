// api.rs
use actix_web::{web, HttpResponse, Responder};
use lastfm::LastFmClient;
use std::sync::Arc;

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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/lastfm_callback/{user_id}").route(web::get().to(lastfm_callback_handler)),
    );
}
