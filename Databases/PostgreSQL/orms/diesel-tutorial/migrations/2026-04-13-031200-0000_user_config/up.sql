-- Your SQL goes here
CREATE TABLE user_configs
(
    id      SERIAL PRIMARY KEY,
    user_id INT REFERENCES users ON DELETE CASCADE NOT NULL,
    enabled BOOLEAN                                NOT NULL DEFAULT TRUE,
    points  INT                                    NOT NULL DEFAULT 0
);