-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
  id TEXT PRIMARY KEY NOT NULL UNIQUE,
  created INT NOT NULL,
  updated INT NOT NULL,
  text TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 0
);