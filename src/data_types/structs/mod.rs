use serde::{Serialize, Deserialize};
use postgres_types::{FromSql, ToSql};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product<'a> {
    pub id: i32,
    pub ds_name: &'a str,
    pub vi_price: f64,
    pub ds_image_url: &'a str,
    pub ds_description: &'a str,
    pub ds_category: &'a str,
    pub ar_tags: Vec<&'a str>,
    pub ds_link: &'a str,
    pub ar_variants: Vec<&'a str>,
    pub en_variant_type: VariantType,
    pub ar_sizes: Vec<&'a str>,
    pub ar_all_of_sizes: Vec<&'a str>,
    pub en_status: Status,
    pub ds_rating: &'a str,
    pub vi_number_of_reviews: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    pub id: i32
}

#[derive(Debug, PartialEq, FromSql, ToSql, Serialize, Deserialize)]
#[postgres(name = "variant_type")]
pub enum VariantType {
    #[postgres(name = "color")]
    Color,
    #[postgres(name = "image")]
    Image
}

#[derive(Debug, PartialEq, FromSql, ToSql, Serialize, Deserialize)]
#[postgres(name = "status")]
pub enum Status {
    #[postgres(name = "New in")]
    NewIn,
    #[postgres(name = "Sold Out")]
    SoldOut,
    #[postgres(name = "50% Discount")]
    HalfOff,
    #[postgres(name = "Limited Edition")]
    LimitedEdition
}

#[derive(Serialize, Debug)]
pub struct Return<'a> {
    pub data: Vec<Product<'a>>
}
