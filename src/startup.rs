use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use uuid::Uuid;

use sqlx::{Connection, Executor, PgConnection, PgPool};

use crate::configuration::{get_configuration, DatabaseSettings};
use crate::routes::health_check;
use crate::routes::subscribe;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub fn run(listener: TcpListener, pg_pool: PgPool) -> Result<Server, std::io::Error> {
    let pg_pool = web::Data::new(pg_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i %s"))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pg_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Erro ao bindar o endereço");

    tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_withouth_db())
        .await
        .expect("failed to connect to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Migration failed");

    connection_pool
}
