use actix_web::web::Json;
use crate::db::query;
use crate::db::QueryBuilder;
use crate::data_types::structs::{NewProduct, Status, VariantType};

pub async fn execute(new_product: Json<NewProduct>) {

    let ds_name: &String = &new_product.ds_name;
    let vi_price: &f64 = &new_product.vi_price;
    let ds_image_url: &String = &new_product.ds_image_url;
    let ds_description: &String = &new_product.ds_description;
    let ds_category: &String = &new_product.ds_category;
    let ar_tags: &Vec<String> = &new_product.ar_tags;
    let ds_link: &String = &new_product.ds_link;
    let ar_variants: &Vec<String> = &new_product.ar_variants;
    let ar_sizes: &Vec<String> = &new_product.ar_sizes;
    let ar_all_of_sizes: &Vec<String> = &new_product.ar_all_of_sizes;
    let ds_rating: &String = &new_product.ds_rating;
    let vi_number_of_reviews: &i32 = &new_product.vi_number_of_reviews;

    let en_variant_type: VariantType = match &new_product.en_variant_type[..] {
        "Color" => VariantType::Color,
        _ => VariantType::Image,
    };

    let en_status: Status = match &new_product.en_status[..] {
        "NewIn" => Status::NewIn,
        "SoldOut" => Status::SoldOut,
        "HalfOff" => Status::HalfOff,
        _ => Status::LimitedEdition
    };

    let query_string: String = String::from("
        INSERT INTO
            product
                (
                    ds_name,
                    vi_price,
                    ds_image_url,
                    ds_description,
                    ds_category,
                    ar_tags,
                    ds_link,
                    ar_variants,
                    en_variant_type,
                    ar_sizes,
                    ar_all_of_sizes,
                    en_status,
                    ds_rating,
                    vi_number_of_reviews
                )
            values
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14);");

    query(QueryBuilder::new(&query_string, Some(&[
        &ds_name,
        &vi_price,
        &ds_image_url,
        &ds_description,
        &ds_category,
        &ar_tags,
        &ds_link,
        &ar_variants,
        &en_variant_type,
        &ar_sizes,
        &ar_all_of_sizes,
        &en_status,
        &ds_rating,
        &vi_number_of_reviews
    ]))).await;
}
