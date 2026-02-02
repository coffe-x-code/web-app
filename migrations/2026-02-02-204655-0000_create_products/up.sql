-- Your SQL goes here
CREATE TABLE product (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description  VARCHAR NOT NULL,
  image_url VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)