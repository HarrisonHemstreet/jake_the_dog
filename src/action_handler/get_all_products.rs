use serde::{Serialize, Deserialize};
use tokio_postgres::types::{FromSql, ToSql, Type};
// use tokio_postgres::error::Error;
use serde::ser::StdError;
use serde_json;
use crate::db::query;
use crate::db::QueryBuilder;

#[derive(Debug, PartialEq, FromSql, ToSql)]
enum VariantType {
    Color,
    Image
}

// enum status {
//     "New in",
//     "Limited Edition",
//     "Sold out",
//     "50% Discount"
// }

// /*
impl<'a> FromSql<'a> for VariantType {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<VariantType, Box<(dyn StdError + Send + Sync + 'static)>> {
        println!("IN FROM SQL!");
        println!("raw: {:?}", raw);
        match raw {
            b"color" => Ok(VariantType::Color),
            b"image" => Ok(VariantType::Image),
            _ => Err(Box::new(std::fmt::Error)),
        }
    }

    fn accepts(ty: &Type) -> bool {
        // This implementation returns true if the type is a string type
        match ty.name() {
            "varchar" | "text" => true,
            _ => false,
        }
    }
}
// */

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
        // en_variant_type: VariantType,
        ar_sizes: Vec<&'a str>,
        ar_all_of_sizes: Vec<&'a str>,
        // en_status: &'a str,
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
        // en_variant_type: rows[0].get(9),
        ar_sizes: rows[0].get(10),
        ar_all_of_sizes: rows[0].get(11),
        // en_status: rows[0].get(12),
        ds_rating: rows[0].get(13),
        vi_number_of_reviews: rows[0].get(14)
    };

    serde_json::to_string(&product).unwrap()
}
