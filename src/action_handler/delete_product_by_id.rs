use actix_web::web::Json;
use crate::db::delete;
use crate::data_types::structs::Id;

pub async fn execute(id: Json<Id>) {
    delete("product", Some(vec!["id"]), Some(&[&id.id])).await;
    // query(QueryBuilder::new("DELETE FROM product WHERE id = $1", Some(&[&id.id]))).await.unwrap();
}
