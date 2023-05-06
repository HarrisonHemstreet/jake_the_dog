use actix_web::web::Json;
use crate::db::query;
use crate::db::QueryBuilder;
use crate::data_types::structs::ProductUpdate;

pub async fn execute(product_update: Json<ProductUpdate>) {
    let mut query_string: String = format!("UPDATE product SET {}", &product_update.col_name);
    query_string = format!("{} = $1 WHERE id = $2", query_string);
    query(QueryBuilder::new(&query_string,
        Some(&[
            &product_update.value_to_update,
            &product_update.id,
        ],
    ))).await;
}
