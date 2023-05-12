use actix_web::web::Json;
use crate::db::delete;
use crate::data_types::structs::Id;

pub async fn execute(id: Json<Id>) {
    delete("product", Some(vec!["id"]), Some(&[&id.id])).await;
}
