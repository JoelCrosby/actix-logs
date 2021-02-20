CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(256) NOT NULL,
  password VARCHAR(256) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT Now()
);

CREATE UNIQUE INDEX users_email_unique
ON users (email);
