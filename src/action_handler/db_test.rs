use serde::{Serialize, Deserialize};
use serde_json;
use crate::db::query;
use crate::db::QueryBuilder;

pub async fn execute() -> String {
    #[derive(Serialize, Deserialize, Debug)]
    struct Id {
        id: i32
    }

    let rows = query(QueryBuilder::new("SELECT * FROM product;", None)).await;

    let id: i32 = rows[0].get(0);
    let new_id: Id = Id {id};
    let serialized = serde_json::to_string(&new_id).unwrap();

    serialized
}
