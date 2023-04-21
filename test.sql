CREATE TYPE variant_type AS ENUM (
    'color',
    'image'
);

CREATE TYPE status AS ENUM (
    'New in',
    'Limited Edition',
    'Sold Out',
    '50% Discount'
);

CREATE TABLE product (
  id SERIAL PRIMARY KEY NOT NULL,
  ds_name VARCHAR(50) NOT NULL,
  vi_price FLOAT NOT NULL,
  ds_image_url VARCHAR(50) NOT NULL,
  ds_description VARCHAR(50) NOT NULL,
  ds_category VARCHAR(50) NOT NULL,
  ar_tags text[] NOT NULL,
  ds_link VARCHAR(50) NOT NULL,
  ar_variants text[] NOT NULL,
  en_variant_type variant_type NOT NULL,
  ar_sizes text[] NOT NULL,
  ar_all_of_sizes text[] NOT NULL,
  en_status status NOT NULL,
  ds_rating VARCHAR(50) NOT NULL,
  vi_number_of_reviews INTEGER NOT NULL
);

CREATE TABLE "review" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "star_score" integer,
  "review_blurb" varchar,
  "created" timestamp
  "last_modified" timestamp
);

CREATE TABLE "user_shopping_cart" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "cart" jsonb
);

CREATE TABLE "user" (
  "id" uuid PRIMARY KEY,
  "name" varchar,
  "email" varchar,
  "phone" varchar,
  "password" varchar,
  "address" varchar,
  "apt_suite" varchar,
  "city" varchar,
  "country" varchar,
  "state_provinence" varchar,
  "postal_code" varchar,
  "address_type" enum,
  "payment_method" varchar,
  "birth_date" varchar,
  "gender" enum,
  "about_you" varchar,
  "saved_products" jsonb,
  "subscription_state" enum,
  "dark_theme" boolean,
  "created" timestamp
);

CREATE TABLE "order" (
  "id" uuid PRIMARY KEY,
  "order" jsonb,
  "order_status" varchar,
  "created" timestamp,
  "last_modified" timestamp
);
