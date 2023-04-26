use actix_web::web::Json;
use crate::db::query;
use crate::db::QueryBuilder;
use crate::data_types::structs::NewProduct;
use postgres_types::{FromSql, ToSql};
use serde::{Serialize, Deserialize};


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


fn format_array_string(vec: &Vec<String>) -> String {
    let mut result = String::from("ARRAY[");
    for (i, item) in vec.iter().enumerate() {
        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(&format!("'{}'", item));
    }
    result.push(']');
    result
}

pub async fn execute(new_product: Json<NewProduct>) -> String {
    // let mut data: Vec<_> = vec![];

    let ds_name = &new_product.ds_name;
    let vi_price: &f64 = &new_product.vi_price;
    let ds_image_url = &new_product.ds_image_url;
    let ds_description = &new_product.ds_description;
    let ds_category = &new_product.ds_category;
    let ar_tags = format_array_string(&new_product.ar_tags);
    let ds_link = &new_product.ds_link;
    let ar_variants = format_array_string(&new_product.ar_variants);
    let en_variant_type = &new_product.en_variant_type;
    let ar_sizes = format_array_string(&new_product.ar_sizes);
    let ar_all_of_sizes = format_array_string(&new_product.ar_all_of_sizes);
    let en_status = &new_product.en_status;
    let ds_rating = &new_product.ds_rating;
    let vi_number_of_reviews = &new_product.vi_number_of_reviews;

    println!("ar_variants: {:?}", ar_variants);
                                                                        //(ds_name, vi_price, ds_image_url, ds_description, ds_category, ar_tags, ds_link, ar_variants, en_variant_type, ar_sizes, ar_all_of_sizes, en_status, ds_rating, vi_number_of_reviews)

    let param_string = format!("'{}', {}, '{}', '{}', '{}', {}, '{}', {}, '{}', {}, {}, '{}', '{}', {}", ds_name, vi_price, ds_image_url, ds_description, ds_category, ar_tags, ds_link, ar_variants, en_variant_type, ar_sizes, ar_all_of_sizes, en_status, ds_rating, vi_number_of_reviews);
    // let param_string = format!("ds_name: '{}', vi_price: {}, ds_image_url: '{}', ds_description: '{}' ds_category: '{}', ar_tags: {}, ds_link: '{}', ar_variants: {}, en_variant_type: '{}', ar_sizes: {}, ar_all_of_sizes: {}, en_status: '{}', ds_rating: '{}', vi_number_of_reviews: {}", ds_name, vi_price, ds_image_url, ds_description, ds_category, ar_tags, ds_link, ar_variants, en_variant_type, ar_sizes, ar_all_of_sizes, en_status, ds_rating, vi_number_of_reviews);
    // let param_string = format!("ds_name: '{}'\n, vi_price: {}\n, ds_image_url: '{}'\n, ds_description: '{}'\n ds_category: '{}'\n, ar_tags: {}\n, ds_link: '{}'\n, ar_variants: {}\n, en_variant_type: '{}'\n, ar_sizes: {}\n, ar_all_of_sizes: {}\n, en_status: '{}'\n, ds_rating: '{}'\n, vi_number_of_reviews: {}", ds_name, vi_price, ds_image_url, ds_description, ds_category, ar_tags, ds_link, ar_variants, en_variant_type, ar_sizes, ar_all_of_sizes, en_status, ds_rating, vi_number_of_reviews);
    // let param_string = format!("ar_tags: {:?}", ar_tags);

    // println!("param_string: {}", param_string);

    // serde_json::to_string("{'status': 'done'}").unwrap()
    // let inserts: Vec<_> = vec![
    //     ds_name,
    //     &vi_price.to_string(),
    //     ds_image_url,
    //     ds_category,
    //     ar_variants,
    //     en_variant_type,
    //     ar_sizes,
    //     &ar_all_of_sizes,
    //     &en_status,
    //     &ds_rating,
    //     &vi_number_of_reviews
    // ];

    /*
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
                ($1);");
    //                  6 = en_variant_type             en_status = 9
    */

    let query_string = format!("
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
                ({});
    ", param_string);

    println!("query_string: {}", query_string);

    // query(QueryBuilder::new(&query_string, Some(&[&param_string]))).await;
    /*
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
    */

    // /*
    // query(QueryBuilder::new(&query_string, Some(&[
    //     &ds_name,
    //     &vi_price,
    //     &ds_image_url,
    //     &ds_description,
    //     &ds_category,
    //     &ar_tags,
    //     &ds_link,
    //     &ar_variants,
    //     &en_variant_type,
    //     &ar_sizes,
    //     &ar_all_of_sizes,
    //     &en_status,
    //     &ds_rating,
    //     &vi_number_of_reviews
    // ]))).await;
    // */
    query(QueryBuilder::new(&query_string, None)).await;
    serde_json::to_string("{'status': 'done'}").unwrap()
    // */

    // String::from("test")
    /*

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
    */
}

/*
ds_name: 'Air Jordan XI',
vi_price: 209.99,
ds_image_url: 'https://i.imgur.com/RShKXkx.jpeg',
ds_description: 'The Air Jordan XI Retro High remakes the classic sneaker, giving you a fresh look with a familiar feel. Premium materials with new colors and textures give modern expression to an all-time favorite.'
ds_category: 'shoes',
ar_tags: ARRAY['shoes', 'feet', 'Michael Jordan'],
ds_link: 'localhost:3000/',
ar_variants: ARRAY['green', 'black', 'purple'],
en_variant_type: 'Color',
ar_sizes: ARRAY['M 13 / W 14.5', 'M 9 / W 10.5'],
ar_all_of_sizes: ARRAY['M 13 / W 14.5', 'M 9 / W 10.5', 'M 10.5 / W 12'],
en_status: 'NewIn',
ds_rating: '4.9',
vi_number_of_reviews: 37
*/





































