-- Your SQL goes here
CREATE TABLE questions (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  "desc" TEXT
);

CREATE TABLE answers (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  question_id INTEGER REFERENCES questions(id)
);

