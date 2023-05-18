use serde_json;
use crate::db::query;
use crate::db::QueryBuilder;
use crate::data_types::structs::Id;
use crate::traits::test_my_trait;

pub struct db_test {}
impl test_my_trait for db_test {
    async fn execute() -> String {
        let rows = query(QueryBuilder::new("SELECT * FROM product;", None)).await.unwrap();
    
        let id: i32 = rows[0].get(0);
        let new_id: Id = Id {id};
        let serialized = serde_json::to_string(&new_id).unwrap();
    
        serialized
    }
}

