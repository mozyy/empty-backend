-- This file should undo anything in `up.sql`
DROP TABLE client_scope;
ALTER TABLE redirect_uris
    DROP COLUMN client_id;
DROP TABLE clients;
DROP TABLE redirect_uris;
DROP TABLE scopes;
