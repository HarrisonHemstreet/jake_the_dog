pub mod db;
pub mod routes;
pub mod action_handler;

use std::io::Result;
use crate::routes::greet;
use crate::routes::db_test2;
use actix_web::{App, HttpServer};


/*
* TODO:
* 1. separate out the routes, database connection, and server initialization into their own files DONE
* 2. make it so that for each route in the route file, each route is very clean and any
*    functionality associated with it calls an external file DONE
* 3. make sure that I can easily make db queries in any file outside of the db connection file DONE
* 4. look into using state w/ actix-web
*/

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(db_test2)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
