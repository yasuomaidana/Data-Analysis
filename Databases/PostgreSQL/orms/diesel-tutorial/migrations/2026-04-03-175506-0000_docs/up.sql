-- Your SQL goes here
CREATE TABLE IF NOT EXISTS docs
(
    id  serial PRIMARY KEY,
    doc text NOT NULL
);


CREATE TABLE IF NOT EXISTS stop_words
(
    id   serial PRIMARY KEY,
    word text NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS stem_words
(
    id   serial PRIMARY KEY,
    word text NOT NULL,
    stem text NOT NULL
);

INSERT INTO stop_words (word)
VALUES ('the'),
       ('is'),
       ('in'),
       ('and'),
       ('to'),
       ('of'),
       ('a'),
       ('that'),
       ('it'),
       ('with'),
       ('as'),
       ('for'),
       ('was'),
       ('on'),
       ('are'),
       ('by'),
       ('this'),
       ('be'),
       ('at'),
       ('from'),
       ('or'),
       ('which'),
       ('but'),
       ('not'),
       ('all'),
       ('any'),
       ('we'),
       ('they'),
       ('their'),
       ('his'),
       ('her'),
       ('its'),
       ('my'),
       ('your'),
       ('our'),
       ('hers'),
       ('theirs');


INSERT INTO stem_words (word, stem)
VALUES ('running', 'run'),
       ('jogging', 'jog'),
       ('happily', 'happy'),
       ('happier', 'happy'),
       ('happiest', 'happy'),
       ('swimming', 'swim'),
       ('swam', 'swim'),
       ('swum', 'swim'),
       ('easily', 'easy'),
       ('easier', 'easy'),
       ('easiest', 'easy'),
       ('flying', 'fly'),
       ('flew', 'fly'),
       ('flown', 'fly'),
       ('studies', 'study'),
       ('studying', 'study'),
       ('studied', 'study'),
       ('cities', 'city'),
       ('caring', 'care'),
       ('cared', 'care'),
       ('teaching', 'teach'),
       ('taught', 'teach'),
       ('teaches', 'teach')
;

CREATE TABLE IF NOT EXISTS doc_gin
(
    key_word TEXT    NOT NULL,
    doc_id   INTEGER NOT NULL REFERENCES docs (id) ON DELETE CASCADE,
    PRIMARY KEY (key_word, doc_id)
);


-- Create trigger function to populate doc_gin on docs insert
CREATE OR REPLACE FUNCTION docs_insert_trigger_fn() RETURNS trigger AS
$$
BEGIN
    INSERT INTO doc_gin (key_word, doc_id)
    SELECT COALESCE(stem, keyword), NEW.id
    FROM (SELECT DISTINCT s.keyword
          FROM unnest(string_to_array(lower(NEW.doc), ' ')) s(keyword)
          WHERE s.keyword NOT IN (SELECT word FROM stop_words)) AS k
             LEFT JOIN stem_words AS sw ON sw.word = k.keyword;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

--- SELECT K.id, COALESCE(stem,keyword) FROM (SELECT DISTINCT id, s.keyword AS keyword
-- FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
-- WHERE s.keyword NOT IN (SELECT word FROM stop_words)
-- ORDER BY id)
-- AS K LEFT JOIN stem_words AS SW ON SW.word = K.keyword;

CREATE TRIGGER docs_after_insert
    AFTER INSERT
    ON docs
    FOR EACH ROW
EXECUTE FUNCTION docs_insert_trigger_fn();

INSERT INTO docs (doc)
SELECT v.doc
FROM (VALUES ('This is SQL and Python and other fun teaching stuff'),
             ('More people should learn SQL from UMSI'),
             ('UMSI also teaches Python and also SQL')) AS v(doc)
WHERE NOT EXISTS (SELECT 1 FROM docs d WHERE d.doc = v.doc);