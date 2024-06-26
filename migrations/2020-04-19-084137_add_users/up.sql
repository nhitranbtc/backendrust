CREATE TABLE users (
  id VARCHAR NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email VARCHAR(100) NOT NULL,
  company TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);
CREATE INDEX users_email_company_idx ON users (email, company);