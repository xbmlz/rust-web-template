use actix_web::{get, App, HttpServer, Responder};
use clap::Parser;
use config::Config;

mod config;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let config = Config::parse();

    println!("Starting server on http://{0}:{1}", config.host, config.port);
    HttpServer::new(|| {
        App::new()
        .service(hello)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
