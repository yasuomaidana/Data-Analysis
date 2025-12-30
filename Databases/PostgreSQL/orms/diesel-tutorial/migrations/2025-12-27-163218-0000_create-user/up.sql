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

CREATE OR REPLACE FUNCTION update_users_updated_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_users_updated
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_users_updated_column();