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
  "created" timestamp DEFAULT NOW(),
  "lastmodified" timestamp DEFAULT NOW()
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

insert into product (id, main_photo, title, subtitle, option, price, tag, available_size, description, product_details)
values (
	'8350fd39-02f5-4317-abe9-54ce28b477d4',
	'https://imgs.search.brave.com/JUcKDltOcY0Tsl9KTafDEMfQFs5HE6oouplg47urRPI/rs:fit:1200:768:1/g:ce/aHR0cHM6Ly9leHRl/cm5hbC1wcmV2aWV3/LnJlZGQuaXQvUUJK/bmlOMTFSSHNNM0s0/V3ZkWkpCc0VBZDVz/YXpZSUtPcnFUdzZk/eWVfYy5qcGc_YXV0/bz13ZWJwJnM9Yzc0/YzRkMDg3NjlkMmQx/NGViZWUyMWY1MTYx/N2JmZmU1YjNmYWE5/OA',
	'test',
	'test',
	'{"hi": "test"}',
	5,
	'test',
	'{"yo": "yoyo"}',
	'test',
	'test'
);
