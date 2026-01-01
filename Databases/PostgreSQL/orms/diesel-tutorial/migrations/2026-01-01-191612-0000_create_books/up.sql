CREATE TABLE books
(
    id        SERIAL PRIMARY KEY,
    title     VARCHAR NOT NULL,
    author_id INT     REFERENCES users (id) ON DELETE SET NULL
);