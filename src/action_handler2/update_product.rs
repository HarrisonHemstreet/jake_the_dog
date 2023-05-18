use actix_web::web::Json;
use crate::db::update;
use crate::data_types::structs::{ColumnValue, ProductIdentifier};

pub async fn execute(product_update: Json<ProductIdentifier>) -> Vec<tokio_postgres::Row> {
    let value: String = match &product_update.col_value {
        ColumnValue::Integer(num) => format!("{}", num),
        ColumnValue::Float(num) => format!("{}", num),
        ColumnValue::Text(text) => text.clone(),
    };

    update("product", Some(vec![&product_update.col_name]), Some(vec!["id"]), Some(&[&value, &product_update.id.unwrap()]),).await
}
