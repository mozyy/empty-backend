-- Your SQL goes here

CREATE TABLE IF NOT EXISTS templates (
  id serial PRIMARY KEY,
  lottery_id integer NOT NULL REFERENCES lotterys(id),
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('templates');

CREATE TABLE IF NOT EXISTS favorites (
  id serial PRIMARY KEY,
  user_id uuid NOT NULL,
  lottery_id integer NOT NULL REFERENCES lotterys(id),
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('favorites');