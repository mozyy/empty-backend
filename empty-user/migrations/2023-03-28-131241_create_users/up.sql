-- Your SQL goes here

CREATE TABLE infos (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  mobile TEXT NOT NULL,
  password TEXT NOT NULL,
  username TEXT,
  avatar TEXT,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('infos');

CREATE TABLE refresh_tokens (
  refresh_token TEXT PRIMARY KEY NOT NULL,
  info_id UUID NOT NULL REFERENCES infos(id),
  expires_in INTEGER NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('refresh_tokens');

CREATE TABLE scopes (
  id SERIAL PRIMARY KEY,
  scope TEXT NOT NULL,
  pattern TEXT NOT NULL,
  "desc" TEXT,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('scopes');

CREATE TABLE access_tokens (
  access_token TEXT PRIMARY KEY NOT NULL,
  info_id UUID NOT NULL REFERENCES infos(id),
  scope TEXT NOT NULL,
  expires_in INTEGER NOT NULL,
  refresh_token TEXT NOT NULL REFERENCES refresh_tokens(refresh_token),

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('access_tokens');