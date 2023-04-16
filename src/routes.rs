pub mod routes {
    use actix_web::{get, web, App, HttpServer, Responder};

    #[get("/hello/{name}")]
    pub async fn greet(name: web::Path<String>) -> impl Responder {
        format!("Hello bitch boi {name}!")
    }
}
