-- Add migration script here
ALTER TABLE examples
DROP COLUMN IF EXISTS email,
DROP COLUMN IF EXISTS created_at;
