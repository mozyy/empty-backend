-- Your SQL goes here
CREATE TABLE questions (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  answer_id INTEGER,
  "desc" TEXT
);

CREATE TABLE answers (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  question_id INTEGER REFERENCES questions(id)
);

-- ALTER TABLE questions
--     ADD COLUMN answer_id INTEGER REFERENCES answers(id);
