use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use sqlx::{PgConnection, PgPool, Pool, Postgres};
use std::net::TcpListener;

pub fn run(listener: TcpListener, pg_pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(pg_pool.clone())
            .service(routes::profile)
            .service(routes::health_check)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

mod routes {
    use actix_web::{get, web, HttpResponse, Responder};

    #[get("/health")]
    pub async fn health_check() -> impl Responder {
        HttpResponse::Ok()
    }
    #[get("/u/{name}")]
    pub async fn profile(name: web::Path<String>) -> impl Responder {
        println!("/u/{}", name);
        HttpResponse::Ok()
    }
}
