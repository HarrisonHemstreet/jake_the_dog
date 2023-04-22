use actix_web::{get, web, Responder, dev::HttpServiceFactory};
use crate::action_handler;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello boi {name}!")
}

#[get("/db_test")]
async fn db_test() -> impl Responder {
    action_handler::db_test::execute().await
}

#[get("/products")]
async fn get_all_products() -> impl Responder {
    action_handler::get_all_products::execute().await
}

/*
TODO:
#[get("/product")]
async fn get_all_products() -> impl Responder {
    // should take in an id and return that specific product
    action_handler::get_all_products::execute().await
}

#[post("/product")]
async fn get_all_products() -> impl Responder {
    // should create a product
    action_handler::get_all_products::execute().await
}
#[put("/product")]
async fn get_all_products() -> impl Responder {
    // should update a product
    action_handler::get_all_products::execute().await
}
#[delete("/product")]
async fn get_all_products() -> impl Responder {
    // should delete a product
    action_handler::get_all_products::execute().await
}
*/


pub fn routes() -> impl HttpServiceFactory {
    (
        greet,
        db_test,
        get_all_products
    )
}
/*
* 1. create some basic post, update, delete requests on product now. I want to make sure I am
*    making ACID apis (i think that's the term?)
*/
