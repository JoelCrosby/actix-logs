CREATE TABLE logs (
  id SERIAL PRIMARY KEY,
  title VARCHAR(256) NOT NULL,
  serialised json,
  created_at TIMESTAMP NOT NULL DEFAULT Now()
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(256) NOT NULL,
  password VARCHAR(256) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT Now()
);
