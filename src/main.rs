use actix_web::{App, HttpServer};
use std::io::Result;

pub mod action_handler;
pub mod db;
pub mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(routes::routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/*
* todos:
* 1. look at the frontend and try to figure out exactly what the shape of my product table should
*    be. fix the sql, etc
* 2. try to find the json? or whatever is getting passed to each product object
*
*
*
  {
    id: 1,
    name: "Mastermind Toys",
    description: "Brown cockroach wings",
    price: 74,
    image: productSport1,
    category: "Category 1",
    tags: ["tag1", "tag2"],
    link: "/product-detail/",
    variants: DEMO_VARIANT_COLORS,
    variantType: "color",
    sizes: ["XS", "S", "M", "L", "XL"],
    allOfSizes: ["XS", "S", "M", "L", "XL", "2XL", "3XL"],
    status: "New in",
    rating: "4.9",
    numberOfReviews: 98,
  },
*
*
*
export interface Product {
  id: number;
  name: string;
  price: number;
  image: StaticImageData | string;
  description: string;
  category: string;
  tags: string[];
  link: "/product-detail/";
  variants?: ProductVariant[];
  variantType?: "color" | "image";
  sizes?: string[];
  allOfSizes?: string[];
  status?: "New in" | "limited edition" | "Sold Out" | "50% Discount";
  rating?: string;
  numberOfReviews?: number;
}

CREATE TYPE variant_type AS ENUM (
    'color',
    'image'
);

CREATE TYPE status AS ENUM (
    "New in",
    "limited edition",
    "Sold Out",
    "50% Discount"
);

CREATE TABLE "product" (
  id SERIAL NOT NULL PRIMARY KEY;
  ds_name VARCHAR(50)
  vi_price FLOAT
  ds_image_url VARCHAR(50)
  ds_description VARCHAR(50)
  ds_category VARCHAR(50)
  ar_tags text[]
  ds_link VARCHAR(50)
  ar_variants text[]
  en_variant_type variant_type NOT NULL
  ar_sizes text[]
  ar_all_of_sizes text[]
  en_status status NOT NULL
  ds_rating VARCHAR(50) NOT NULL
  vi_number_of_reviews INTEGER
);

*
*
*
*
*
*
*
*
*
*/
