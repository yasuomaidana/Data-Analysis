CREATE TYPE user_role AS ENUM ('guest', 'user', 'moderator', 'admin');

CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    name       TEXT      NOT NULL,
    email      TEXT      NOT NULL UNIQUE,
    role       user_role NOT NULL DEFAULT 'user',
    birth_date DATE,
    updated    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);