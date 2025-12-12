-- Your SQL goes here
CREATE TABLE car
(
    id    SERIAL PRIMARY KEY,
    model VARCHAR(50) NOT NULL UNIQUE
);