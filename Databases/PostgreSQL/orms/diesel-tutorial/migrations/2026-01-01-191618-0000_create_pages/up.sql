CREATE TABLE pages
(
    id          SERIAL PRIMARY KEY,
    page_number INT  NOT NULL,
    content     TEXT NOT NULL,
    book_id     INT  NOT NULL REFERENCES books (id) ON DELETE CASCADE
);

