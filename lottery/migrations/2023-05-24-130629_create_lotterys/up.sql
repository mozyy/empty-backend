-- Your SQL goes here

CREATE TYPE IF NOT EXISTS item AS (
  name TEXT,
  value INTEGER
);

CREATE TYPE IF NOT EXISTS remark AS (
  name TEXT,
  require BOOLEAN
);

CREATE TABLE IF NOT EXISTS lotterys (
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
