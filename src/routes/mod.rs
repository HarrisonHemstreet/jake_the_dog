use actix_web::{get, web, Responder, dev::HttpServiceFactory};
// use actix_web::dev::{ServiceFactory};
use crate::action_handler;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello boi {name}!")
}

#[get("/db_test")]
async fn db_test2() -> impl Responder {
    action_handler::db_test::execute().await
}

pub fn routes() -> impl HttpServiceFactory {
    (greet, db_test2)
}
