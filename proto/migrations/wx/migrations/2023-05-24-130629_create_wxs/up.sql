-- Your SQL goes here

CREATE TABLE IF NOT EXISTS wx_users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL,
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

-- data

INSERT INTO public.wx_users (id, user_id, openid, unionid, session_key, name, avatar, mobile, created_at, updated_at) VALUES ('3c233329-54ea-4ef8-b4a4-fe7b789cbcf6', 'be70fa85-dbea-45a5-8648-5ff148bceb32', 'o_hTz5FLVfNsCTCcmb0NWTkEz-HQ', NULL, '+K8wil48ztwTPsWa77+g5Q==', 'user name', NULL, NULL, '2023-07-13 13:42:51.843053', '2023-07-13 13:42:51.843053');