-- Your SQL goes here


CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
SELECT diesel_manage_updated_at('users');

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

SELECT pg_catalog.setval('public.oauth_configs_id_seq', 5, true);