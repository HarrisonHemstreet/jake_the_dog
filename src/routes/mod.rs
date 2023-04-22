use actix_web::{get, web, Responder, dev::HttpServiceFactory};
use actix_web::web::Json;
use crate::action_handler;
use crate::data_types::structs::Id;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello boi {name}!")
}

#[get("/products")]
async fn get_all_products() -> impl Responder {
    action_handler::get_all_products::execute().await
}

#[get("/product")]
async fn get_product_by_id(id: Json<Id>) -> impl Responder {
    action_handler::get_product_by_id::execute(id).await
}
// #[post("/product")]
// async fn create_new_product(product: Json<Product>)) -> impl Responder {
//     // should create a product
//     action_handler::get_all_products::execute().await
// }

/*
TODO:
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
        get_all_products,
        get_product_by_id,
        // create_new_product
    )
}
/*
* 1. create some basic post, update, delete requests on product now. I want to make sure I am
*    making ACID apis (i think that's the term?)
*/
