-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
  id serial PRIMARY KEY,
  addr text NOT NULL,
  user_id uuid NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('users');