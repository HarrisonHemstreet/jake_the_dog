use actix_web::{App, HttpServer};
use std::io::Result;

pub mod action_handler;
pub mod db;
pub mod routes;
pub mod data_types;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(routes::routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .expect("\nERROR: src/main.rs: server initialization fail\n");

        Ok(())
}

/*
* 1. I need to figure out how to work with query params better
* 2. I need to add error handling
*/
