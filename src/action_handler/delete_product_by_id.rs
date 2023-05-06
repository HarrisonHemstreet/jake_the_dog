use actix_web::web::Json;
use crate::db::query;
use crate::db::QueryBuilder;
use crate::data_types::structs::Id;

pub async fn execute(id: Json<Id>) {
    query(QueryBuilder::new("DELETE FROM product WHERE id = $1", Some(&[&id.id]))).await.unwrap();
}
