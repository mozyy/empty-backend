-- Your SQL goes here


CREATE TABLE lotterys (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  type INTEGER NOT NULL,
  remark BOOLEAN NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('lotterys');

CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  value INTEGER NOT NULL,
  lottery_id INTEGER  NOT NULL REFERENCES lotterys(id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('items');

CREATE TABLE remarks (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  require BOOLEAN NOT NULL,
  lottery_id INTEGER  NOT NULL REFERENCES lotterys(id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('remarks');
