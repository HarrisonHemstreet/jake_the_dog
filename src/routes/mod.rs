use actix_web::{get, post, put, delete, http, web, Responder, dev::HttpServiceFactory, HttpResponse};
use actix_web::web::Json;
use crate::action_handler;
use crate::data_types::structs::{Id, NewProduct, ProductIdentifier};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello boi {name}!")
}

#[get("/products")]
async fn get_all_products() -> impl Responder {
    let res = action_handler::get_all_products::execute().await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[get("/product")]
async fn get_product_by_id(id: Json<Id>) -> HttpResponse {
    let res = action_handler::get_product_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[post("/product")]
async fn create_new_product(product: Json<NewProduct>) -> HttpResponse {
    action_handler::create_new_product::execute(product).await;
    HttpResponse::Created()
        .status(http::StatusCode::CREATED)
        .finish()
}

#[put("/product")]
async fn update_product(product_update: Json<ProductIdentifier>) -> HttpResponse {
    action_handler::update_product::execute(product_update).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[delete("/product")]
async fn delete_product_by_id(id: Json<Id>) -> impl Responder {
    action_handler::delete_product_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

pub fn routes() -> impl HttpServiceFactory {
    (
        greet,
        get_all_products,
        get_product_by_id,
        create_new_product,
        update_product,
        delete_product_by_id
    )
}
