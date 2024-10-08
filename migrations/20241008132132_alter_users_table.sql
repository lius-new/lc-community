-- Add migration script here
ALTER TABLE users
DROP COLUMN IF EXISTS email,
DROP COLUMN IF EXISTS created_at;
