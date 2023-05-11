use actix_web::web::Json;
use crate::db::get;
// use crate::db::QueryBuilder;
use crate::data_types::structs::{Id, Product, Return};

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let row = get("product", None, Some(vec!["id"]), Some(&[&id.id])).await;

    data.push(Product {
        id: row[0].get(0),
        ds_name: row[0].get(1),
        vi_price: row[0].get(2),
        ds_image_url: row[0].get(3),
        ds_description: row[0].get(4),
        ds_category: row[0].get(5),
        ar_tags: row[0].get(6),
        ds_link: row[0].get(7),
        ar_variants: row[0].get(8),
        en_variant_type: row[0].get(9),
        ar_sizes: row[0].get(10),
        ar_all_of_sizes: row[0].get(11),
        en_status: row[0].get(12),
        ds_rating: row[0].get(13),
        vi_number_of_reviews: row[0].get(14)
    });

    let return_data: Return = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
