use serde::{Serialize, Deserialize};
use serde_json;
use std::io::Result;
pub mod db;
use crate::db::db::query;
pub mod routes;
use crate::routes::routes::greet;
use actix_web::{App, HttpServer};

/*
* TODO:
* 1. separate out the routes, database connection, and server initialization into their own files
* 2. make it so that for each route in the route file, each route is very clean and any
*    functionality associated with it calls an external file
* 3. make sure that I can easily make db queries in any file outside of the db connection file
* 4. look into using state w/ actix-web
*/

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

/*
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
*/

/*
pub mod routes {
    use actix_web::{get, web, App, HttpServer, Responder};

    #[get("/hello/{name}")]
    async fn greet(name: web::Path<String>) -> impl Responder {
        format!("Hello bitch {name}!")
    }
}
*/

/*
mod db {
    use tokio_postgres::NoTls;
    use uuid::Uuid;

    pub async fn execute_db_stuff() {

        // Connect to the database.
        let (client, connection) =
            tokio_postgres::connect("host=10.0.0.154 user=root port=5440 password=root dbname=root", NoTls).await.unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Now we can execute a simple statement that just returns its parameter.
        let rows = client
            .query("SELECT * FROM product;", &[])
            .await.unwrap();
        for row in &rows {
            let id: Uuid  = row.get(0);
            println!("id: {:?}", &id);
        }
    }

    pub async fn connect_to_db() -> tokio_postgres::Client {

        let (client, connection) =
            tokio_postgres::connect("host=10.0.0.154 user=root port=5440 password=root dbname=root", NoTls).await.unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        client
    }

    pub async fn query(queryStr: &str, queryParams: &str) -> Vec<tokio_postgres::Row> {
        let (client, connection) =
            tokio_postgres::connect("host=10.0.0.154 user=root port=5440 password=root dbname=root", NoTls).await.unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let rows = client
            .query(queryStr, &[])
            .await.unwrap();
        rows
    }
}
*/

#[tokio::main]
async fn main() -> Result<()> {
    // db::execute_db_stuff().await;

    /*
    let client: tokio_postgres::Client = db::connect_to_db().await;
    let rows = client
        .query("SELECT * FROM product;", &[])
        .await.unwrap();
    for row in &rows {
        let id: uuid::Uuid  = row.get(0);
        println!("test to see if working id: {:?}", &id);
    }
    */

    let rows = query("SELECT * FROM product;", "").await;
    for row in &rows {
        let id: uuid::Uuid  = row.get(0);
        println!("test to see if still working bitch id: {:?}", &id);
    }

    // // Connect to the database.
    // let (client, connection) =
    //     tokio_postgres::connect("host=10.0.0.154 user=root port=5440 password=root dbname=root", NoTls).await.unwrap();

    // // The connection object performs the actual communication with the database,
    // // so spawn it off to run on its own.
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });

    // // Now we can execute a simple statement that just returns its parameter.
    // let rows = client
    //     .query("SELECT * FROM product;", &[])
    //     .await.unwrap();
    // for row in &rows {
    //     let id: Uuid  = row.get(0);
    //     println!("id: {:?}", &id);
    // }

    /////////////////////////////////////////////////////////////////////////

    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
