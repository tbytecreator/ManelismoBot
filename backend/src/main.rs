use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use log::info;

mod handlers;
mod models;
mod manelismo_bot;

use handlers::ask;
use models::QuestionRequest;

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub bot_context: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting Manelismo Bot Backend...");
    
    let app_state = web::Data::new(Mutex::new(AppState {
        bot_context: String::from("Eu sou o Manoel Alves Ferreira Neto, criador do TByteCreator"),
    }));

    info!("Server starting on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(actix_cors::Cors::permissive())
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(health_check))
                    .route("/ask", web::post().to(ask::ask_question))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "Manelismo Bot Backend"
    }))
}
