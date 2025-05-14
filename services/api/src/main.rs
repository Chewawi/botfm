use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use common::config::CONFIG;
use common::utils::tracing_init;
use database::DatabaseHandler;
use lastfm::LastFmClient;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(serde::Deserialize)]
struct CallbackQuery {
    token: String,
}

async fn lastfm_callback_handler(
    tmpl: web::Data<Tera>,
    lastfm_client: web::Data<Arc<LastFmClient>>,
    query: web::Query<CallbackQuery>,
    path: web::Path<u64>,
) -> impl Responder {
    let user_id = path.into_inner();
    let token = query.token.clone();

    let result = lastfm_client.handle_callback(&token, user_id).await;

    let mut context = Context::new();

    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/html")
            .body(tmpl.render("auth.html", &context).unwrap()),
        Err(err) => {
            context.insert("error", &err.to_string());
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(tmpl.render("auth_error.html", &context).unwrap())
        }
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
        DatabaseHandler::new(CONFIG.database.to_url(), CONFIG.database.to_url_safe())
            .await
            .unwrap(),
    );
    let lastfm = Arc::new(
        LastFmClient::new(http_client.clone(), db.clone())
            .await
            .unwrap(),
    );

    let data = web::Data::new(lastfm);

    let static_path: PathBuf = {
        let mut dir = env::current_dir().expect("can't get current dir");
        dir.push("services/api/static");
        dir
    };

    let tera = web::Data::new(
        Tera::new(format!("{}/**/*", static_path.to_str().unwrap()).as_str()).unwrap(),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .app_data(data.clone())
            .configure(config)
            .service(Files::new("/static", static_path.clone()).show_files_listing())
    })
    .bind(("0.0.0.0", CONFIG.api.port))?
    .run()
    .await
}
