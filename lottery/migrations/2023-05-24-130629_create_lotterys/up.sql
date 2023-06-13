-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  openid TEXT NOT NULL,
  unionid TEXT NOT NULL,
  session_key TEXT NOT NULL,
  name TEXT NOT NULL,
  avatar TEXT,
  mobile TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('users');

CREATE TYPE item AS (
  name TEXT,
  value INTEGER
);

CREATE TYPE remark AS (
  name TEXT,
  require BOOLEAN
);

CREATE TABLE IF NOT EXISTS lotterys (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id),
  title TEXT NOT NULL,
  type INTEGER NOT NULL,
  items item[] NOT NULL,
  remark BOOLEAN NOT NULL,
  remarks remark[] NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('lotterys');
