-- Add down migration script here
ALTER TABLE users 
    DROP COLUMN verified,
    DROP COLUMN admin;
