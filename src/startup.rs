use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer, Responder};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check, subscripe};

async fn greet() -> impl Responder {
    format!("Hello world")
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscripe))
            .route("/", web::get().to(greet))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
