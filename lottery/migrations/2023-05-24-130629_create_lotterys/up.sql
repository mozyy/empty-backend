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

CREATE TYPE item AS (
  name text,
  value integer
);

CREATE TYPE remark AS (
  name text,
  require boolean
);

CREATE TABLE IF NOT EXISTS lotterys (
  id serial PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id),
  title text NOT NULL,
  type integer NOT NULL,
  items item[] NOT NULL,
  remark boolean NOT NULL,
  remarks remark[] NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('lotterys');

CREATE TABLE IF NOT EXISTS records (
  id serial PRIMARY KEY,
  lottery_id integer NOT NULL REFERENCES lotterys(id),
  user_id uuid NOT NULL REFERENCES users(id),
  value text NOT NULL,
  remarks text[] NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('records');



-- data

-- INSERT INTO public.users (id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('377fbb26-f017-411d-8c42-5933e478c99d', '152fd562-3e8a-4da8-9205-8a061a08b3e7', '4c49e4fb-f0fc-4b89-9bd4-5bb401d835a0', '3cd428d7-565c-47e3-a765-0e7509759b1a', 'yyue', NULL, NULL, '2023-06-14 14:52:17.608501', '2023-06-14 14:52:17.608501');
-- INSERT INTO public.users (id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('5ef323b2-dbe5-4260-8c6b-bde402b99cfa', '5c64ec9d-7300-47a2-962c-0eb01d81f6eb', '930e9e51-6578-4fe0-96c7-ef56495af1c5', 'bfa15305-dcb8-4bcd-9810-65c021713360', 'yyue', NULL, NULL, '2023-06-14 14:52:48.094519', '2023-06-14 14:52:48.094519');
-- INSERT INTO public.users (id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('58b3c2dd-b361-4f71-ad72-ff135ec26051', '9b399361-adf6-4816-921b-cc61cedb132b', '7246b2ec-c49a-4a87-b629-6ff9aab1721f', 'bac3c31c-d2b6-452c-9bda-1689eb1677fe', 'yyue', NULL, NULL, '2023-06-14 15:04:38.80616', '2023-06-14 15:04:38.80616');
-- INSERT INTO public.users (id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('64f2069a-1dab-4dff-89f4-ae96a4d44b61', '7aa1b7cb-ea6f-40c5-bf85-f4599fc5d600', '6027c30d-d3eb-49d8-bf0b-0c237fa02986', 'd6f78085-7b29-4b29-b205-d63bba16b859', 'yyue', NULL, NULL, '2023-06-14 15:04:40.61343', '2023-06-14 15:04:40.61343');
-- INSERT INTO public.users (id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('b5e771a2-a9e2-4b0e-8292-1e4ca09241d9', '6d64b374-e4d5-432e-a69e-7c25e797f60a', '200d5ee3-4676-4c99-9121-e827794405ec', '95f68c1e-f380-4bf4-b6ee-faa1f51b2261', 'yyue', NULL, NULL, '2023-06-14 15:05:09.482175', '2023-06-14 15:05:09.482175');
-- INSERT INTO public.users (id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('6595bad4-b4df-421e-9ead-6d2f5bc2e9df', 'ca1c2153-19e6-479a-8527-1bed78f36d42', 'dea7526b-27f9-44be-ac15-5059e3316b45', 'bdf7f927-0e8a-4b01-819d-aba00343e7dd', 'yyue', NULL, NULL, '2023-06-14 15:24:28.942451', '2023-06-14 15:24:28.942451');
-- INSERT INTO public.users (id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('a9deef41-16f3-4fba-9403-4d41204250fc', '6db3de71-7557-4578-a083-bae4eef37d97', '8fe0ad8e-ea71-4d99-9628-4b4de572d19e', 'c314b8ce-ab7f-4c6b-8f2c-57b105655e10', 'yyue', NULL, NULL, '2023-06-14 15:24:45.770527', '2023-06-14 15:24:45.770527');


-- INSERT INTO public.lotterys (id, user_id, title, type, items, remark, remarks, created_at, updated_at) VALUES (2, '377fbb26-f017-411d-8c42-5933e478c99d', 'title', 1, '{"(\"item name1\",1)","(\"item name2\",2)"}', true, '{}', '2023-06-14 14:52:17.654092', '2023-06-14 14:52:17.654092');
-- INSERT INTO public.lotterys (id, user_id, title, type, items, remark, remarks, created_at, updated_at) VALUES (3, '5ef323b2-dbe5-4260-8c6b-bde402b99cfa', 'title', 1, '{"(\"item name1\",1)","(\"item name2\",2)"}', true, '{}', '2023-06-14 14:52:48.11257', '2023-06-14 14:52:48.11257');
-- INSERT INTO public.lotterys (id, user_id, title, type, items, remark, remarks, created_at, updated_at) VALUES (4, '58b3c2dd-b361-4f71-ad72-ff135ec26051', 'title', 1, '{"(\"item name1\",1)","(\"item name2\",2)"}', true, '{}', '2023-06-14 15:04:38.845235', '2023-06-14 15:04:38.845235');
-- INSERT INTO public.lotterys (id, user_id, title, type, items, remark, remarks, created_at, updated_at) VALUES (5, '64f2069a-1dab-4dff-89f4-ae96a4d44b61', 'title', 1, '{"(\"item name1\",1)","(\"item name2\",2)"}', true, '{}', '2023-06-14 15:04:40.630229', '2023-06-14 15:04:40.630229');
-- INSERT INTO public.lotterys (id, user_id, title, type, items, remark, remarks, created_at, updated_at) VALUES (6, 'b5e771a2-a9e2-4b0e-8292-1e4ca09241d9', 'title', 1, '{"(\"item name1\",1)","(\"item name2\",2)"}', true, '{}', '2023-06-14 15:05:09.498969', '2023-06-14 15:05:09.498969');
-- INSERT INTO public.lotterys (id, user_id, title, type, items, remark, remarks, created_at, updated_at) VALUES (7, '6595bad4-b4df-421e-9ead-6d2f5bc2e9df', 'title', 1, '{"(\"item name1\",1)","(\"item name2\",2)"}', true, '{}', '2023-06-14 15:24:28.983', '2023-06-14 15:24:28.983');
-- INSERT INTO public.lotterys (id, user_id, title, type, items, remark, remarks, created_at, updated_at) VALUES (8, 'a9deef41-16f3-4fba-9403-4d41204250fc', 'title', 1, '{"(\"item name1\",1)","(\"item name2\",2)"}', true, '{}', '2023-06-14 15:24:45.787016', '2023-06-14 15:24:45.787016');



-- SELECT pg_catalog.setval('public.lotterys_id_seq', 8, true);

-- SELECT pg_catalog.setval('public.records_id_seq', 1, false);
