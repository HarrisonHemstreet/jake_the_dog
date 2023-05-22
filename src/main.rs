use actix_web::{App, HttpServer};
use std::io::Result;
use better_todos::better_todo as bt;

pub mod action_handler;
pub mod db;
pub mod routes;
pub mod data_types;

#[tokio::main]
async fn main() -> Result<()> {
bt("
main.rs: line: 12:
Todo:
1. add authentication. maybe the oauth2 crate? or oauth2 in general? or maybe I leave authentication up to the frontend? idk

");
    HttpServer::new(|| App::new().service(routes::routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .expect("\nERROR: src/main.rs: server initialization fail\n");

        Ok(())
}

