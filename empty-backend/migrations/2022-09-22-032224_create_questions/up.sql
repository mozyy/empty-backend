-- Your SQL goes here
CREATE TABLE questions (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  "desc" TEXT
);

CREATE TABLE answers (
  id SERIAL PRIMARY KEY,
  question_id INTEGER  NOT NULL REFERENCES questions(id),
  content TEXT NOT NULL
);

CREATE TABLE question_answers (
  id SERIAL PRIMARY KEY,
  question_id INTEGER NOT NULL REFERENCES questions(id),
  answer_id INTEGER NOT NULL REFERENCES answers(id),
  content TEXT NOT NULL
);

-- ALTER TABLE questions
--     ADD COLUMN answer_id INTEGER REFERENCES answers(id);
