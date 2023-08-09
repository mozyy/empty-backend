-- Your SQL goes here

CREATE TABLE IF NOT EXISTS lotterys (
  id serial PRIMARY KEY,
  user_id uuid NOT NULL,
  title text NOT NULL,
  type integer NOT NULL,
  remark boolean NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('lotterys');

CREATE TABLE IF NOT EXISTS items (
  id serial PRIMARY KEY,
  lottery_id integer NOT NULL REFERENCES lotterys(id),
  name text NOT NULL,
  value integer NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('items');

CREATE TABLE IF NOT EXISTS remarks (
  id serial PRIMARY KEY,
  lottery_id integer NOT NULL REFERENCES lotterys(id),
  name text NOT NULL,
  require boolean NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('remarks');

CREATE TABLE IF NOT EXISTS records (
  id serial PRIMARY KEY,
  lottery_id integer NOT NULL REFERENCES lotterys(id),
  user_id uuid NOT NULL,
  item_id integer NOT NULL REFERENCES items(id),
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('records');

CREATE TABLE IF NOT EXISTS record_remarks (
  id serial PRIMARY KEY,
  record_id integer NOT NULL REFERENCES records(id),
  remark_id integer NOT NULL REFERENCES remarks(id),
  value text NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('record_remarks');
