CREATE TABLE log_entries (
  id SERIAL PRIMARY KEY,
  title VARCHAR(256) NOT NULL,
  serialised TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT Now()
);
