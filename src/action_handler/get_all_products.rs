use crate::db::query;
use crate::db::QueryBuilder;
use crate::data_types::structs::{Product, Return};

pub async fn execute() -> String {
    let mut data = vec![];

    let rows = query(QueryBuilder::new("SELECT * FROM product;", None)).await;
    
    let mut i: usize = 0;
    loop {
        if i > rows.len() - 1 {
            break;
        }

        data.push(Product {
            id: rows[i].get(0),
            ds_name: rows[i].get(1),
            vi_price: rows[i].get(2),
            ds_image_url: rows[i].get(3),
            ds_description: rows[i].get(4),
            ds_category: rows[i].get(5),
            ar_tags: rows[i].get(6),
            ds_link: rows[i].get(7),
            ar_variants: rows[i].get(8),
            en_variant_type: rows[i].get(9),
            ar_sizes: rows[i].get(10),
            ar_all_of_sizes: rows[i].get(11),
            en_status: rows[i].get(12),
            ds_rating: rows[i].get(13),
            vi_number_of_reviews: rows[i].get(14)
        });
        
        i += 1;
    }

    let return_data: Return = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
