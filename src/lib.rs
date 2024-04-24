// ------------------------ IMPORTS ------------------------------------
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// ------------------------ TYPES --------------------------------------

#[derive(serde::Deserialize)]
struct FormData {
    _email: String,
    _name: String,
}

// ------------------------ AUX FUNC  ----------------------------------

// ------------------------ ENDPOINTS ----------------------------------

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// ----------------------- RUN SERVER ----------------------------------

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
