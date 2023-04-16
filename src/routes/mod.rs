use actix_web::{get, web, Responder};
use crate::action_handler;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello boi {name}!")
}

#[get("/db_test")]
pub async fn db_test2() -> impl Responder {
    action_handler::db_test::execute().await
}
