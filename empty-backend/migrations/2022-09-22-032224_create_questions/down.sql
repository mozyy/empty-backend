-- This file should undo anything in `up.sql`
ALTER TABLE questions DROP COLUMN answer_id;

DROP TABLE answers;
DROP TABLE questions;
