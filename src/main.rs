use actix_web::{App, HttpServer};
use std::io::Result;

pub mod action_handler;
pub mod db;
pub mod routes;

/*
* TODO:
* 1. separate out the routes, database connection, and server initialization into their own files DONE
* 2. make it so that for each route in the route file, each route is very clean and any
*    functionality associated with it calls an external file DONE
* 3. make sure that I can easily make db queries in any file outside of the db connection file DONE
* 4. look into using state w/ actix-web -> I'll look into this later, so DONE
* 5. try to figure out how I can programmatically set the "service" entry for each route without
*    having to explicitly add it each time like below DONE
* 6. I need to figure out how to use query params on the tokio-postgres crate DONE
*/

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(routes::routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
