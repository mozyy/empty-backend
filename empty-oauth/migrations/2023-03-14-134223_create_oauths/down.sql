-- This file should undo anything in `up.sql`
ALTER TABLE registered_urls
    DROP COLUMN client_id;
DROP TABLE clients;
DROP TABLE registered_urls;
