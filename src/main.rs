use actix_web::{App, HttpServer};
use std::io::Result;
use better_todos::better_todo as bt;

pub mod action_handler;
pub mod action_handler2;
pub mod db;
pub mod routes;
pub mod data_types;

#[tokio::main]
async fn main() -> Result<()> {
bt("
main.rs: line: 12:
Todo:
1. Think about ways I can improve this server. Maybe I can add authentication or something like that
2. clean this whole project up. get it ready for submitting to some larger repo as an example

");
    HttpServer::new(|| App::new().service(routes::routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .expect("\nERROR: src/main.rs: server initialization fail\n");

        Ok(())
}

