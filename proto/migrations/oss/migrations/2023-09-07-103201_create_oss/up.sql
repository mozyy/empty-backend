-- Your SQL goes here

CREATE TABLE IF NOT EXISTS oss (
  id serial PRIMARY KEY,
  bucket_name integer NOT NULL,
  object_key text NOT NULL,
  name text NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('oss');