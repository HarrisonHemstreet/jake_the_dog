CREATE TYPE variant_type AS ENUM (
    'Color',
    'Image'
);

CREATE TYPE status AS ENUM (
    'NewIn',
    'LimitedEdition',
    'SoldOut',
    'HalfOff'
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
    'Color',
    ARRAY['M 13 / W 14.5', 'M 9 / W 10.5'],
    ARRAY['M 13 / W 14.5', 'M 9 / W 10.5', 'M 10.5 / W 12'],
    'NewIn',
    '4.9',
    37
  );

INSERT INTO
  product
    (ds_name, vi_price, ds_image_url, ds_description, ds_category, ar_tags, ds_link, ar_variants, en_variant_type, ar_sizes, ar_all_of_sizes, en_status, ds_rating, vi_number_of_reviews)
  values (
    'Air Jordan III',
    209.99,
    'https://i.imgur.com/iXQ4UXH.jpeg',
    'Elevate your sneaker game with Air Jordan 3. Designed for comfort and style, the iconic silhouette is a must-have for any sneakerhead.',
    'shoes',
    ARRAY['shoes', 'feet', 'Michael Jordan'],
    'localhost:3000/',
    ARRAY['green', 'black', 'purple'],
    'Color',
    ARRAY['M 13 / W 14.5', 'M 9 / W 10.5'],
    ARRAY['M 13 / W 14.5', 'M 9 / W 10.5', 'M 10.5 / W 12'],
    'NewIn',
    '4.9',
    37
  );

-- TODO: Evetually implement authentication in some way
-- CREATE TABLE api_key (
--   id SERIAL PRIMARY KEY NOT NULL,
--   name VARCHAR(50) NOT NULL,
--   api_key VARCHAR NOT NULL
-- );

-- INSERT INTO
--   api_key
--     (name, api_key)
--   VALUES (
--     'Harrison Hemstreet',
--     (SELECT gen_random_uuid())
--   );
