-- Add up migration script here
CREATE TABLE IF NOT EXISTS users(
   email TEXT NOT NULL PRIMARY KEY,
   password_hash TEXT NOT NULL,
   requires_2fa BOOLEAN NOT NULL DEFAULT FALSE
);