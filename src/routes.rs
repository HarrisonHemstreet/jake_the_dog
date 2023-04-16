pub mod routes {
    use actix_web::{get, web, Responder};
    use serde::{Serialize, Deserialize};
    use serde_json;
    use uuid::Uuid;
    use crate::db::db::query;

    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[get("/hello/{name}")]
    pub async fn greet(name: web::Path<String>) -> impl Responder {
        format!("Hello bitch boi {name}!")
    }

    #[get("/db_test")]
    pub async fn db_test() -> impl Responder {

        #[derive(Serialize, Deserialize, Debug)]
        struct Id {
            id: Uuid
        }

        let rows = query("SELECT * FROM product;", "").await;

        let id: uuid::Uuid = rows[0].get(0);
        let new_id: Id = Id {id};
        let serialized2 = serde_json::to_string(&new_id).unwrap();

        serialized2
    }
}
