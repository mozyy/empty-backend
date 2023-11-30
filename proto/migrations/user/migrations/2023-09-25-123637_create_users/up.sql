-- Your SQL goes here

-- ---- user.proto start ----
CREATE TABLE IF NOT EXISTS infos (
  id serial PRIMARY KEY,
  name text NOT NULL,
  avatar integer,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('infos');

CREATE TABLE IF NOT EXISTS mobiles (
  id serial PRIMARY KEY,
  mobile text NOT NULL,
  password text NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('mobiles');

CREATE TABLE IF NOT EXISTS weixins (
  id serial PRIMARY KEY,
  openid text NOT NULL,
  unionid text,
  session_key text NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('weixins');

CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  info_id integer NOT NULL REFERENCES infos(id),
  mobile_id integer REFERENCES mobiles(id),
  weixin_id integer REFERENCES weixins(id),
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('users');
-- ---- user.proto end ----


-- ---- auth.proto start ----
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

CREATE TABLE IF NOT EXISTS authorization_codes (
  id serial PRIMARY KEY,
  code text NOT NULL,
  user_id uuid NOT NULL REFERENCES users(id),
  client_id uuid NOT NULL REFERENCES clients(id),
  scope text NOT NULL,
  redirect_uri text NOT NULL,
  until timestamp NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('authorization_codes');

CREATE TABLE IF NOT EXISTS refresh_resources (
  id serial PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id),
  client_id uuid NOT NULL REFERENCES clients(id),
  refresh_token text NOT NULL,
  scope text NOT NULL,
  token_type text NOT NULL,
  until timestamp NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('refresh_resources');
-- ---- auth.proto end ----


-- data


INSERT INTO public.clients (id, name, redirect_uri, default_scope, default_expires_in, passdata) VALUES ('f2e69885-951a-4538-b0c8-67385f0c1420', 'zuoyinyun', 'https://zuoyinyun.com', 'logined', 2592000, NULL);

INSERT INTO public.configs (id, pattern, scope) VALUES (1, '(3,.*)', 'logined');
INSERT INTO public.configs (id, pattern, scope) VALUES (2, '(3,"^/lottery.LotteryService/Lotterys/\\d$")', NULL);
INSERT INTO public.configs (id, pattern, scope) VALUES (3, '(3,"/user.user.UserService/login.*")', NULL);
INSERT INTO public.configs (id, pattern, scope) VALUES (4, '(3,.*)', NULL);
INSERT INTO public.configs (id, pattern, scope) VALUES (5, '(3,.*)', NULL);

SELECT pg_catalog.setval('public.configs_id_seq', 5, true);