-- Your SQL goes here
CREATE TABLE IF NOT EXISTS jtrack
(
    id   SERIAL PRIMARY KEY NOT NULL,
    body JSONB
);