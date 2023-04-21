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
  ds_description VARCHAR(200) NOT NULL,
  ds_category VARCHAR(50) NOT NULL,
  ar_tags text[] NOT NULL,
  ds_link VARCHAR(50) NOT NULL,
  ar_variants text[] NOT NULL,
  en_variant_type variant_type NOT NULL,
  ar_sizes text[] NOT NULL,
  ar_all_of_sizes text[] NOT NULL,
  en_status status NOT NULL,
  ds_rating VARCHAR(50) NOT NULL,
  vi_number_of_reviews INTEGER NOT NULL,
  dt_created TIMESTAMP DEFAULT now()
);

INSERT INTO
  product
    (ds_name, vi_price, ds_image_url, ds_description, ds_category, ar_tags, ds_link, ar_variants, en_variant_type, ar_sizes, ar_all_of_sizes, en_status, ds_rating, vi_number_of_reviews)
  values (
    'Air Jordan I',
    179.99,
    'https://i.imgur.com/RShKXkx.jpeg',
    'The Air Jordan 1 Retro High remakes the classic sneaker, giving you a fresh look with a familiar feel. Premium materials with new colors and textures give modern expression to an all-time favorite.',
    'shoes',
    ARRAY['shoes', 'feet', 'Michael Jordan'],
    'localhost:3000/',
    ARRAY['green', 'black', 'purple'],
    'color',
    ARRAY['M 13 / W 14.5', 'M 9 / W 10.5'],
    ARRAY['M 13 / W 14.5', 'M 9 / W 10.5', 'M 10.5 / W 12'],
    'New in',
    '4.9',
    37
  );
