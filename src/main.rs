use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use baseboard;
use baseboard::config::{get_config, Settings};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::Ipv4Addr;
use std::net::TcpListener;
use std::path::Path;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = Path::new("config/init.yaml").to_str();

    if let Some(path) = path {
        let config: Settings = get_config(path).expect("Config parsed.");
        let listener = TcpListener::bind(config.get_tcp_address()).expect("App binds to TCP");
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.get_database_url())
            .await
            .expect("Postgres connection established.");

        baseboard::app::run(listener, pg_pool)?.await
        //baseboard::app::run(listener, pool).unwrap().await;
    } else {
        panic!("No Config File Found!")
    }
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
