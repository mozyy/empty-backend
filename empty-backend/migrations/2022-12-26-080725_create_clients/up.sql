-- Your SQL goes here

CREATE TABLE scopes (
  id SERIAL PRIMARY KEY,

  name TEXT NOT NULL,
  "desc" TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('scopes');

CREATE TABLE redirect_uris (
  id SERIAL PRIMARY KEY,

  -- client_id INTEGER  NOT NULL REFERENCES clients(id),
  url TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
  -- updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE clients (
  id UUID PRIMARY KEY,

  redirect_uri_id INTEGER  NOT NULL REFERENCES redirect_uris(id),
  name TEXT NOT NULL,
  "desc" TEXT NOT NULL,
  passphrase TEXT,
  
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE redirect_uris
    ADD COLUMN client_id UUID REFERENCES clients(id);

SELECT diesel_manage_updated_at('clients');

CREATE TABLE client_scope (
  id SERIAL PRIMARY KEY,

  client_id UUID  NOT NULL REFERENCES clients(id),
  redirect_uri_id INTEGER  NOT NULL REFERENCES redirect_uris(id),
  
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
