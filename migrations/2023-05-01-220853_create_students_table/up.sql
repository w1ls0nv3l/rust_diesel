-- Your SQL goes here
CREATE TABLE students (
  id VARCHAR NOT NULL PRIMARY KEY,
  name_student VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);