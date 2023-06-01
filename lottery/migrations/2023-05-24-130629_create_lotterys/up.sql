-- Your SQL goes here

CREATE TYPE item AS (
  name TEXT,
  value INTEGER
);

CREATE TYPE remark AS (
  name TEXT,
  require BOOLEAN
);

CREATE TABLE lotterys (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  type INTEGER NOT NULL,
  items item[] NOT NULL,
  remark BOOLEAN NOT NULL,
  remarks remark[] NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('lotterys');
