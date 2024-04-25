use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::health_check;
use crate::routes::subscribe;

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

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Erro ao bindar o endereço");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
