-- Your SQL goes here

ALTER TABLE users ADD COLUMN nickname VARCHAR NOT NULL DEFAULT '';

UPDATE users SET nickname = 'nick';
