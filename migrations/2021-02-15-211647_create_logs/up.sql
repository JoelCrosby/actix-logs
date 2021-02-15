CREATE TABLE logs (
  id SERIAL PRIMARY KEY,
  title VARCHAR(256) NOT NULL,
  serialised json,
  created_at timestamp NOT NULL DEFAULT Now()
);
