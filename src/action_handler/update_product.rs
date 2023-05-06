use actix_web::web::Json;
use crate::db::query;
use crate::db::QueryBuilder;
use crate::data_types::structs::{ColumnValue, ProductIdentifier};

pub async fn execute(product_update: Json<ProductIdentifier>) {
    let value: String = match &product_update.col_value {
        ColumnValue::Integer(num) => format!("{}", num),
        ColumnValue::Float(num) => format!("{}", num),
        ColumnValue::Text(text) => text.clone(),
    };
    
    let mut query_string: String = format!("UPDATE product SET {}", &product_update.col_name);
    query_string = format!("{} = $1 WHERE id = $2", query_string);
    query(QueryBuilder::new(&query_string,
        Some(&[
            &value,
            &product_update.id.unwrap(),
        ],
    ))).await.unwrap();
}
