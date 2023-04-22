use actix_web::{App, HttpServer};
use std::io::Result;

pub mod action_handler;
pub mod db;
pub mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(routes::routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

