-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS stop_words;
DROP TABLE IF EXISTS stem_words;
DROP TRIGGER IF EXISTS doc_insert_trigger ON docs;
DROP TABLE IF EXISTS docs;
