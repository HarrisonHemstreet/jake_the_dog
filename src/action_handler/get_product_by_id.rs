use actix_web::web::Json;
use crate::routes::route_structs::Id;
use serde::{Serialize, Deserialize};
use postgres_types::{FromSql, ToSql};
use crate::db::query;
use crate::db::QueryBuilder;

#[derive(Debug, PartialEq, FromSql, ToSql, Serialize, Deserialize)]
#[postgres(name = "variant_type")]
enum VariantType {
    #[postgres(name = "color")]
    Color,
    #[postgres(name = "image")]
    Image
}

#[derive(Debug, PartialEq, FromSql, ToSql, Serialize, Deserialize)]
#[postgres(name = "status")]
enum Status {
    #[postgres(name = "New in")]
    NewIn,
    #[postgres(name = "Sold Out")]
    SoldOut,
    #[postgres(name = "50% Discount")]
    HalfOff,
    #[postgres(name = "Limited Edition")]
    LimitedEdition
}

#[derive(Serialize, Deserialize, Debug)]
struct Product<'a> {
    id: i32,
    ds_name: &'a str,
    vi_price: f64,
    ds_image_url: &'a str,
    ds_description: &'a str,
    ds_category: &'a str,
    ar_tags: Vec<&'a str>,
    ds_link: &'a str,
    ar_variants: Vec<&'a str>,
    en_variant_type: VariantType,
    ar_sizes: Vec<&'a str>,
    ar_all_of_sizes: Vec<&'a str>,
    en_status: Status,
    ds_rating: &'a str,
    vi_number_of_reviews: i32
}

#[derive(Serialize, Debug)]
struct Return<'a> {
    data: Vec<Product<'a>>
}

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let row = query(QueryBuilder::new("SELECT * FROM product WHERE id = $1 LIMIT 1;", Some(&[&id.id]))).await;

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
