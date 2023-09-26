-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('users');

CREATE TYPE pattern AS (
  type integer,
  value text 
);

CREATE TABLE IF NOT EXISTS clients (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  redirect_uri text NOT NULL,
  default_scope text NOT NULL,
  default_expires_in integer NOT NULL,
  passdata text,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('clients');

CREATE TABLE IF NOT EXISTS configs (
  id serial PRIMARY KEY,
  pattern pattern,
  scope text,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('configs');

CREATE TABLE IF NOT EXISTS resources (
  id serial PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id),
  client_id uuid NOT NULL REFERENCES clients(id),
  access_token text NOT NULL,
  refresh_token text NOT NULL,
  scope text NOT NULL,
  token_type text NOT NULL,
  until timestamp NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('resources');


-- data

INSERT INTO public.users (id) VALUES ('be70fa85-dbea-45a5-8648-5ff148bceb32');


INSERT INTO public.clients (id, name, redirect_uri, default_scope, default_expires_in, passdata) VALUES ('f2e69885-951a-4538-b0c8-67385f0c1420', 'zuoyinyun', 'https://zuoyinyun.com', 'logined', 2592000, NULL);

INSERT INTO public.configs (id, pattern, scope) VALUES (1, '(3,.*)', 'logined');
INSERT INTO public.configs (id, pattern, scope) VALUES (2, '(3,"^/lottery.LotteryService/Lotterys/\\d$")', NULL);
INSERT INTO public.configs (id, pattern, scope) VALUES (3, '(1,/user.UserService/Login)', NULL);
INSERT INTO public.configs (id, pattern, scope) VALUES (4, '(3,.*)', NULL);
INSERT INTO public.configs (id, pattern, scope) VALUES (5, '(3,.*)', NULL);

SELECT pg_catalog.setval('public.configs_id_seq', 5, true);