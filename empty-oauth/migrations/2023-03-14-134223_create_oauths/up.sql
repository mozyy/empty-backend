-- Your SQL goes here

CREATE TABLE registered_urls (
  id SERIAL PRIMARY KEY

  -- client_id INTEGER  NOT NULL REFERENCES clients(id),
  -- url TEXT NOT NULL,
  -- -- 类型： 1: Exact(ExactUrl), 2: Semantic(Url), 3: IgnorePortOnLocalhost(IgnoreLocalPortUrl)
  -- type SMALLINT NOT NULL, 
  -- created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  -- updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('registered_urls');


CREATE TABLE clients (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

  redirect_uri_id INTEGER NOT NULL REFERENCES registered_urls(id),
  -- name TEXT NOT NULL,
  -- "desc" TEXT NOT NULL,
  -- passphrase TEXT,
  --  check (default_scope <> '{}' and array_position(default_scope, null) is null)
  default_scope TEXT[] NOT NULL,
  -- 空为Public, 有值为Confidential{passdata: Vec<u8>}
  client_type TEXT,
  
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE registered_urls
    ADD COLUMN client_id UUID REFERENCES clients(id),
    ADD COLUMN url TEXT NOT NULL,
    -- -- 类型： 1: Exact(ExactUrl), 2: Semantic(Url), 3: IgnorePortOnLocalhost(IgnoreLocalPortUrl)
    ADD COLUMN type SMALLINT NOT NULL,
    ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;

SELECT diesel_manage_updated_at('clients');
