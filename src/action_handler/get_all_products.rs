use serde::{Serialize, Deserialize};
use serde_json;
use crate::db::query;
use crate::db::QueryBuilder;
use postgres_types::{FromSql, ToSql};

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

/*
fn process_status(mut status: &str) -> &str {
    if status == "NewIn" {
        status = "New In";
    }
    if status == "SoldOut" {
        status = "Sold Out";
    }
    if status == "HalfOff" {
        status = "50% Discount";
    }
    if status == "LimitedEdition" {
        status = "Limited Edition";
    }
    status
}
*/

pub async fn execute() -> String {
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

    let rows = query(QueryBuilder::new("SELECT * FROM product LIMIT 1;", None)).await;

    let product: Product = Product {
        id: rows[0].get(0),
        ds_name: rows[0].get(1),
        vi_price: rows[0].get(2),
        ds_image_url: rows[0].get(3),
        ds_description: rows[0].get(4),
        ds_category: rows[0].get(5),
        ar_tags: rows[0].get(6),
        ds_link: rows[0].get(7),
        ar_variants: rows[0].get(8),
        en_variant_type: rows[0].get(9),
        ar_sizes: rows[0].get(10),
        ar_all_of_sizes: rows[0].get(11),
        en_status: rows[0].get(12),
        ds_rating: rows[0].get(13),
        vi_number_of_reviews: rows[0].get(14)
    };

    serde_json::to_string(&product).unwrap()
}
