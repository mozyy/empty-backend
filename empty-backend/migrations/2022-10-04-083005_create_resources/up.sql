
CREATE TABLE resources (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER  NOT NULL REFERENCES resources(id),
  key VARCHAR NOT NULL,
  rtype INTEGER NOT NULL,
  name TEXT NOT NULL,
  "desc" TEXT,
  sort INTEGER NOT NULL,
  path TEXT NOT NULL,
  index BOOLEAN,
  menu BOOLEAN,
  icon TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('resources');

-- ALTER TABLE resources
--     ADD COLUMN resource_id INTEGER REFERENCES resources(id);
