-- Your SQL goes here
CREATE TABLE gin_array_docs
(
    id  SERIAL,
    doc TEXT,
    PRIMARY KEY (id)
);
CREATE INDEX gin_array ON gin_array_docs USING gin (string_to_array(doc, ' ') array_ops);

INSERT INTO gin_array_docs (doc)
VALUES ('This is SQL and Python and other fun teaching stuff'),
       ('More people should learn SQL from UMSI'),
       ('UMSI also teaches Python and also SQL');


INSERT INTO gin_array_docs (doc)
SELECT 'Neon ' || generate_series(10000, 20000);

CREATE TABLE gin_ts_docs
(
    id  SERIAL,
    doc TEXT,
    PRIMARY KEY (id)
);
CREATE INDEX gin_ts ON gin_ts_docs USING gin (to_tsvector('english', doc));

INSERT INTO gin_ts_docs (doc)
VALUES ('This is SQL and Python and other fun teaching stuff'),
       ('More people should learn SQL from UMSI'),
       ('UMSI also teaches Python and also SQL'),
       ('Hola este es un ejemplo de una oración en español'),
       ('A veces un texto puede estar en otros idiomas')
;

-- Filler rows
INSERT INTO gin_ts_docs (doc)
SELECT 'Neon ' || generate_series(10000, 20000);
