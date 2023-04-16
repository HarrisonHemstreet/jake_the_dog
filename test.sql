CREATE TABLE "product" (
  "id" uuid PRIMARY KEY,
  "main_photo" varchar,
  "title" varchar,
  "subtitle" varchar,
  "option" jsonb,
  "price" int,
  "tag" varchar,
  "available_size" jsonb,
  "description" varchar,
  "product_details" varchar,
  "FAQ" varchar,
  "created" timestamp,
  "lastmodified" timestamp
);

CREATE TABLE "review" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "star_score" integer,
  "review_blurb" varchar,
  "created" timestamp
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
