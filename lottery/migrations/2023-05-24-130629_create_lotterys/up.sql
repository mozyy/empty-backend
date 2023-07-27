-- Your SQL goes here


CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('users');

CREATE TABLE IF NOT EXISTS wx_users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES users(id),
  openid text NOT NULL,
  unionid text,
  session_key text NOT NULL,
  name text NOT NULL,
  avatar text,
  mobile text,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('wx_users');


CREATE TABLE IF NOT EXISTS lotterys (
  id serial PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id),
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
  user_id uuid NOT NULL REFERENCES users(id),
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


CREATE TYPE oauth_pattern AS (
  type integer,
  value text 
);

CREATE TABLE IF NOT EXISTS oauth_clients (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  redirect_uri text NOT NULL,
  default_scope text NOT NULL,
  passdata text
);

CREATE TABLE IF NOT EXISTS oauth_configs (
  id serial PRIMARY KEY,
  pattern oauth_pattern,
  scope text
);


-- data

INSERT INTO public.users (id, created_at, updated_at) VALUES ('be70fa85-dbea-45a5-8648-5ff148bceb32', '2023-07-13 13:42:51.820676', '2023-07-13 13:42:51.820676');


INSERT INTO public.oauth_clients (id, name, redirect_uri, default_scope, passdata) VALUES ('f2e69885-951a-4538-b0c8-67385f0c1420', 'zuoyinyun', 'https://zuoyinyun.com', 'logined', NULL);

INSERT INTO public.oauth_configs (id, pattern, scope) VALUES (1, '(3,.*)', 'logined');
INSERT INTO public.oauth_configs (id, pattern, scope) VALUES (2, '(3,"^/lottery.LotteryService/Lotterys/\\d$")', NULL);
INSERT INTO public.oauth_configs (id, pattern, scope) VALUES (3, '(1,/user.UserService/Login)', NULL);
INSERT INTO public.oauth_configs (id, pattern, scope) VALUES (4, '(3,.*)', NULL);
INSERT INTO public.oauth_configs (id, pattern, scope) VALUES (5, '(3,.*)', NULL);

INSERT INTO public.wx_users (id, user_id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('3c233329-54ea-4ef8-b4a4-fe7b789cbcf6', 'be70fa85-dbea-45a5-8648-5ff148bceb32', 'o_hTz5FLVfNsCTCcmb0NWTkEz-HQ', NULL, '+K8wil48ztwTPsWa77+g5Q==', 'user name', NULL, NULL, '2023-07-13 13:42:51.843053', '2023-07-13 13:42:51.843053');

SELECT pg_catalog.setval('public.oauth_configs_id_seq', 5, true);
SELECT pg_catalog.setval('public.records_id_seq', 1, false);