## Full-Text Search 

This chapter explains the lecture script in `lec05.sql` step by step. The file gradually builds from simple string splitting to a manual inverted index, then to PostgreSQL's built-in full-text search features.

The script teaches five big ideas:
1. How to break text into words.
2. How an inverted index works.
3. Why does lowercasing, stop words, and stemming improve search?
4. How PostgreSQL's `GIN` index can speed up keyword lookups.
5. Why PostgreSQL's `tsvector` / `tsquery` features are easier and more powerful than hand-rolled SQL search.

> Source used in the lecture file: `https://www.pg4e.com/lectures/05-FullText.sql` and `https://www.pg4e.com/lectures/05-FullText`

---

## 1. Strings, arrays, and rows

The first two queries show the building blocks used throughout the rest of the script.

```
SELECT string_to_array('Hello world', ' ');
SELECT unnest(string_to_array('Hello world', ' '));
```

### What each statement does

#### `SELECT string_to_array('Hello world', ' ');`
- `string_to_array(text, delimiter)` splits a string into an array.
- `'Hello world'` is the input text.
- `' '` is the delimiter, so PostgreSQL splits on spaces.
- The result is roughly `{"Hello","world"}`.

This matters because search often starts by tokenizing text into individual words.

#### `SELECT unnest(string_to_array('Hello world', ' '));`
- `string_to_array(...)` again produces an array of words.
- `unnest(...)` expands the array into one row per element.
- Instead of one array value, you get two rows: `Hello` and `world`.

This is important because relational databases work naturally with rows. Once words become rows, they can be filtered, joined, deduplicated, and indexed.

---

## 2. Building a manual inverted index with plain SQL

An **inverted index** maps a word to the documents that contain it. Search engines use this idea constantly.

### 2.1 Create a table of documents

```
CREATE TABLE docs (id SERIAL, doc TEXT, PRIMARY KEY(id));
INSERT INTO docs (doc) VALUES
('This is SQL and Python and other fun teaching stuff'),
('More people should learn SQL from UMSI'),
('UMSI also teaches Python and also SQL');
SELECT * FROM docs;
```

### What each statement does

#### `CREATE TABLE docs (id SERIAL, doc TEXT, PRIMARY KEY(id));`
- Creates a table named `docs`.
- `id SERIAL` creates an auto-incrementing integer-like identifier.
- `doc TEXT` stores the full document text.
- `PRIMARY KEY(id)` makes `id` unique and indexed.

This gives every document a stable identifier so words can later point back to the document that contains them.

#### `INSERT INTO docs (doc) VALUES ...`
- Inserts three sample documents.
- Each row is one small text document.
- The `id` values are generated automatically.

These rows are the corpus that will be indexed.

#### `SELECT * FROM docs;`
- Displays the data that was just inserted.
- Useful as a quick sanity check before building the index.

### 2.2 Split each document into one row per word

```
SELECT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(D.doc, ' ')) s(keyword)
ORDER BY id;
```

### Explanation
- `docs AS D` gives the table a short alias.
- `string_to_array(D.doc, ' ')` splits each document into words.
- `unnest(...)` turns those words into rows.
- `s(keyword)` assigns the output column the name `keyword`.
- The query returns one row per `(document_id, word)` pair.
- `ORDER BY id` groups results by document for readability.

This is the key conceptual step: one document row becomes many word rows.

### 2.3 Remove duplicate words per document

```
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(D.doc, ' ')) s(keyword)
ORDER BY id;
```

### Explanation
- This is the same query as before, but now it uses `DISTINCT`.
- If a word appears more than once in the same document, only one `(id, keyword)` pair remains.
- For indexing, this can reduce duplication and shrink the inverted index.

For example, if a document contains `and` twice, the index only needs one entry saying that document contains `and`.

### 2.4 Create a table to store the inverted index

```
CREATE TABLE docs_gin (
  keyword TEXT,
  doc_id INTEGER REFERENCES docs(id) ON DELETE CASCADE
);
```

### Explanation
- Creates a second table named `docs_gin`.
- `keyword TEXT` stores one indexed word.
- `doc_id INTEGER` stores the document id where that word appears.
- `REFERENCES docs(id)` creates a foreign key to `docs`.
- `ON DELETE CASCADE` means if a document is deleted, its keyword rows are deleted too.

Despite the name `docs_gin`, this table is not yet a real PostgreSQL `GIN` index. It is a manually built inverted-index table.

### 2.5 Populate the manual index table

```
INSERT INTO docs_gin (doc_id, keyword)
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(D.doc, ' ')) s(keyword)
ORDER BY id;
```

### Explanation
- Inserts data into `docs_gin`.
- The `SELECT DISTINCT` produces one row per unique word per document.
- `id` is inserted into `doc_id`.
- `keyword` is inserted into `keyword`.

After this step, `docs_gin` acts like a searchable map from a word to the documents containing that word.

### 2.6 Inspect the manual index

```
SELECT * FROM docs_gin ORDER BY doc_id;
```

### Explanation
- Shows the rows stored in the manual inverted index.
- `ORDER BY doc_id` makes it easier to see which keywords belong to each document.

### 2.7 Find matching document ids for one keyword

```
SELECT DISTINCT keyword, doc_id FROM docs_gin AS G
WHERE G.keyword = 'UMSI';
```

### Explanation
- Looks directly in the inverted-index table.
- Filters the rows to the keyword `UMSI`.
- Returns the documents that contain that word.

This is the heart of inverted indexing: search the word first, not the full documents first.

### 2.8 Get the full documents for one keyword

```
SELECT DISTINCT id, doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword = 'UMSI';
```

### Explanation
- `docs_gin` tells us which `doc_id` values match the keyword.
- The `JOIN` connects those ids back to the original `docs` table.
- The result is the full matching documents instead of only ids.
- `DISTINCT` avoids repeated documents if duplication exists.

### 2.9 Search for several keywords at once

```
SELECT DISTINCT doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword IN ('fun', 'people');
```

### Explanation
- `IN ('fun', 'people')` means “match either of these words”.
- This behaves like a simple OR search.
- Any document containing `fun` or `people` is returned.

### 2.10 Turn a phrase into several searchable words

```
SELECT DISTINCT doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword = ANY(string_to_array('I want to learn', ' '));
```

### Explanation
- `string_to_array('I want to learn', ' ')` splits the phrase into `I`, `want`, `to`, `learn`.
- `= ANY(array)` checks whether `G.keyword` matches any element in that array.
- This is another OR-style search.

Important limitation: this does **not** search for the exact phrase. It only searches for any individual word from the phrase.

### 2.11 The stop-word problem

```
SELECT DISTINCT id, doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword = ANY(string_to_array('Search for Lemons and Neons', ' '));
```

### Explanation
- This query again splits a user search string into words.
- One of the words is `and`, which is extremely common and low in meaning.
- If common words are indexed and searched the same way as useful words, they can produce noisy results.

This is the script's foreshadowing: natural-language search needs more than raw text splitting.

---

## 3. Replacing the manual table with a real `GIN` index on arrays

The next section shows that PostgreSQL can index tokenized data directly.

```
DROP TABLE docs cascade;
CREATE TABLE docs (id SERIAL, doc TEXT, PRIMARY KEY(id));
```

### Explanation
- `DROP TABLE docs cascade;` removes `docs` and dependent objects.
- `CASCADE` is needed because related objects may depend on `docs`.
- Then `docs` is recreated from scratch.

This resets the experiment.

### 3.1 Create a real GIN index on the tokenized document

```
DROP INDEX gin1;

CREATE INDEX gin1 ON docs USING gin(string_to_array(doc, ' ')  array_ops);
```

### What this means

#### `DROP INDEX gin1;`
- Removes the old index if it exists.
- In a rerunnable script, `DROP INDEX IF EXISTS gin1;` would usually be safer.

#### `CREATE INDEX gin1 ON docs USING gin(string_to_array(doc, ' ') array_ops);`
- Creates an index named `gin1`.
- `USING gin` tells PostgreSQL to use a **Generalized Inverted Index**.
- The indexed expression is `string_to_array(doc, ' ')`.
- So PostgreSQL indexes the array of words produced from each document.
- `array_ops` tells PostgreSQL which operator class to use for arrays.

This is the database-managed equivalent of the manual inverted index idea.

### 3.2 Insert documents again

```
INSERT INTO docs (doc) VALUES
('This is SQL and Python and other fun teaching stuff'),
('More people should learn SQL from UMSI'),
('UMSI also teaches Python and also SQL');
```

### Explanation
- Reloads the same three sample documents.
- The `GIN` index is updated as rows are inserted.

### 3.3 Add many filler rows

```
INSERT INTO docs (doc) SELECT 'Neon ' || generate_series(10000,20000);
```

### Explanation
- `generate_series(10000,20000)` generates many numbers.
- `'Neon ' || generate_series(...)` concatenates the word `Neon` with each number.
- The result is thousands of synthetic documents.

Why do this?
- Small tables often do not clearly show index behavior.
- Adding many rows makes it more likely that PostgreSQL will choose the index in the query plan.

### 3.4 Query with an array containment operator

```
SELECT id, doc FROM docs WHERE '{learn}' <@ string_to_array(doc, ' ');
EXPLAIN SELECT id, doc FROM docs WHERE '{learn}' <@ string_to_array(doc, ' ');
```

### Explanation

#### `SELECT id, doc FROM docs WHERE '{learn}' <@ string_to_array(doc, ' ');`
- `'{learn}'` is a one-element text array.
- `string_to_array(doc, ' ')` produces the array of words in each document.
- `<@` means “is contained by”.
- So the query asks: is the array `{'learn'}` contained within the document's word array?
- In simpler terms: does this document contain the word `learn`?

#### `EXPLAIN ...`
- Shows PostgreSQL's execution plan instead of returning query results.
- This lets you see whether the `GIN` index is being used.

---

## 4. Improving the manual index for natural language

The next section rebuilds the manual inverted index, but now it treats the text more like natural language.

Goals:
1. Ignore case.
2. Remove low-value stop words.
3. Later, reduce words to stems.

```
DROP TABLE docs CASCADE;
CREATE TABLE docs (id SERIAL, doc TEXT, PRIMARY KEY(id));
INSERT INTO docs (doc) VALUES
('This is SQL and Python and other fun teaching stuff'),
('More people should learn SQL from UMSI'),
('UMSI also teaches Python and also SQL');
SELECT * FROM docs;
```

### Explanation
- This resets the `docs` table again.
- The three sample documents are reinserted.
- `SELECT * FROM docs;` confirms the reset data.

### 4.1 See the raw tokenization again

```
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(D.doc, ' ')) s(keyword)
ORDER BY id;
```

### Explanation
- This is the same deduplicated tokenization used earlier.
- It serves as a baseline before improving the index.

### 4.2 Normalize to lowercase

```
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
ORDER BY id;
```

### Explanation
- `lower(D.doc)` converts the document text to lowercase before splitting.
- This makes `SQL`, `Sql`, and `sql` index the same way.
- Case normalization avoids missing results because of capitalization differences.

### 4.3 Recreate the manual index table

```
DROP TABLE docs_gin CASCADE;
CREATE TABLE docs_gin (
  keyword TEXT,
  doc_id INTEGER REFERENCES docs(id) ON DELETE CASCADE
);
```

### Explanation
- Drops the old `docs_gin` table and any dependent objects.
- Recreates it cleanly for the new, improved indexing strategy.

### 4.4 Create a stop-word list

```
DROP TABLE stop_words;
CREATE TABLE stop_words (word TEXT unique);
INSERT INTO stop_words (word) VALUES ('is'), ('this'), ('and');
```

### Explanation

#### `DROP TABLE stop_words;`
- Removes the stop-word table from a prior run.

#### `CREATE TABLE stop_words (word TEXT unique);`
- Creates a table containing words that should not be indexed.
- `unique` prevents duplicate stop words.

#### `INSERT INTO stop_words (word) VALUES ('is'), ('this'), ('and');`
- Adds sample stop words.
- These are common words with low search value in this small demo.

### 4.5 Remove stop words during tokenization

```
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
WHERE s.keyword NOT IN (SELECT word FROM stop_words)
ORDER BY id;
```

### Explanation
- Converts text to lowercase.
- Splits documents into words.
- Removes words found in `stop_words`.
- `NOT IN (SELECT word FROM stop_words)` filters out low-information terms.

This makes the index smaller and search results more relevant.

### 4.6 Insert the cleaned words into the manual index

```
INSERT INTO docs_gin (doc_id, keyword)
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
WHERE s.keyword NOT IN (SELECT word FROM stop_words)
ORDER BY id;
```

### Explanation
- Builds the index using the improved token list.
- Only lowercase, non-stop-word tokens are inserted.

### 4.7 Inspect the cleaned index

```
SELECT * FROM docs_gin;
```

### Explanation
- Shows the indexed keywords after stop-word removal.
- You should no longer see words such as `is`, `this`, or `and`.

### 4.8 Query examples on the cleaned index

#### One-word query

```
SELECT DISTINCT doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword = lower('UMSI');
```

Explanation:
- `lower('UMSI')` normalizes the search term.
- Because the index stores lowercase tokens, the query must do the same.
- Matching documents are returned.

#### Multi-word query

```
SELECT DISTINCT doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword =
  ANY(string_to_array(lower('Meet fun people'), ' '));
```

Explanation:
- Lowercases the user query.
- Splits it into words.
- Returns documents containing any of those words.
- Again, this is keyword matching, not exact phrase matching.

#### Stop-word query

```
SELECT DISTINCT doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword = lower('and');
```

Explanation:
- Searches for `and`.
- But `and` was excluded from the index.
- So the query returns no rows.

This demonstrates the effect of stop-word removal: the search engine behaves as if those words were never indexed.

---

## 5. Adding stemming with a custom dictionary

**Stemming** reduces related words to a common base form, such as `teaching` and `teaches` becoming `teach`.

### 5.1 Create a small stemming dictionary

```
CREATE TABLE docs_stem (word TEXT, stem TEXT);
INSERT INTO docs_stem (word, stem) VALUES
('teaching', 'teach'), ('teaches', 'teach');
```

### Explanation
- `docs_stem` maps original words to their stem.
- The sample data says:
  - `teaching` → `teach`
  - `teaches` → `teach`

This is a very small, manual version of what a real linguistic dictionary would do automatically.

### 5.2 Move word extraction into a subquery

```
SELECT id, keyword FROM (
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
) AS X;
```

### Explanation
- The inner query extracts distinct lowercase words per document.
- The outer query selects from that result.
- This structure makes it easier to add joins or transformations later.

The point here is not new functionality yet; it is preparing the query to become more sophisticated.

### 5.3 Attach optional stems with a `LEFT JOIN`

```
SELECT id, keyword, stem FROM (
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
) AS K
LEFT JOIN docs_stem AS S ON K.keyword = S.word;
```

### Explanation
- The subquery `K` extracts document keywords.
- `LEFT JOIN docs_stem AS S ON K.keyword = S.word` looks for a stem for each word.
- If a stem exists, `stem` is filled in.
- If not, `stem` is `NULL`.

`LEFT JOIN` is used because most words may not have an entry in the stem table.

### 5.4 Prefer the stem when it exists

```
SELECT id,
CASE WHEN stem IS NOT NULL THEN stem ELSE keyword END AS awesome,
keyword, stem
FROM (
SELECT DISTINCT id, lower(s.keyword) AS keyword
FROM docs AS D, unnest(string_to_array(D.doc, ' ')) s(keyword)
) AS K
LEFT JOIN docs_stem AS S ON K.keyword = S.word;
```

### Explanation
- The inner query extracts lowercase keywords.
- The `LEFT JOIN` adds stem values where available.
- `CASE WHEN stem IS NOT NULL THEN stem ELSE keyword END` chooses the stem if it exists; otherwise it keeps the original keyword.
- The result is labeled `awesome`, meaning “the final normalized token to index”.

### 5.5 Demonstrate `COALESCE`

```
SELECT COALESCE(NULL, NULL, 'umsi');
SELECT COALESCE('umsi', NULL, 'SQL');
```

### Explanation
- `COALESCE(...)` returns the first non-`NULL` value.
- First query returns `'umsi'` because the first two values are `NULL`.
- Second query also returns `'umsi'` because it is already the first non-`NULL` value.

This is a compact replacement for the earlier `CASE` expression.

### 5.6 Use `COALESCE` to pick stem or keyword

```
SELECT id, COALESCE(stem, keyword) AS keyword
FROM (
SELECT DISTINCT id, s.keyword AS keyword
FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
) AS K
LEFT JOIN docs_stem AS S ON K.keyword = S.word;
```

### Explanation
- `COALESCE(stem, keyword)` means:
  - use `stem` if present,
  - otherwise use `keyword`.
- This is the same logic as the previous `CASE`, just shorter.

### 5.7 Rebuild the index with stems only

```
DELETE FROM docs_gin;

INSERT INTO docs_gin (doc_id, keyword)
SELECT id, COALESCE(stem, keyword)
FROM (
  SELECT DISTINCT id, s.keyword AS keyword
  FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
) AS K
LEFT JOIN docs_stem AS S ON K.keyword = S.word;

SELECT * FROM docs_gin;
```

### Explanation

#### `DELETE FROM docs_gin;`
- Clears the existing manual index.
- The table structure stays, but its rows are removed.

#### `INSERT INTO docs_gin ...`
- Extracts lowercase keywords.
- Replaces words with stems when available.
- Inserts the normalized form into the index.

#### `SELECT * FROM docs_gin;`
- Shows the new contents of the index.
- You should see `teach` instead of separate tokens like `teaching` or `teaches` where the dictionary applies.

### 5.8 Combine stop-word removal and stemming

```
DELETE FROM docs_gin;

INSERT INTO docs_gin (doc_id, keyword)
SELECT id, COALESCE(stem, keyword)
FROM (
  SELECT DISTINCT id, s.keyword AS keyword
  FROM docs AS D, unnest(string_to_array(lower(D.doc), ' ')) s(keyword)
  WHERE s.keyword NOT IN (SELECT word FROM stop_words)
) AS K
LEFT JOIN docs_stem AS S ON K.keyword = S.word;

SELECT * FROM docs_gin;
```

### Explanation
- Clears the old index again.
- Extracts lowercase words.
- Removes stop words before indexing.
- Converts words to stems where possible.
- Stores the final normalized tokens.

This is a much more realistic natural-language index than the raw version from earlier.

### 5.9 Normalize the user's search terms too

```
SELECT COALESCE((SELECT stem FROM docs_stem WHERE word=lower('SQL')), lower('SQL'));
```

### Explanation
- Looks for a stem for the word `SQL`.
- If no stem exists, it falls back to `lower('SQL')`, which is `sql`.
- This is important because search terms must be normalized the same way indexed terms were normalized.

### 5.10 Search using the normalized term

```
SELECT DISTINCT id, doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword = COALESCE((SELECT stem FROM docs_stem WHERE word=lower('SQL')), lower('SQL'));
```

### Explanation
- Normalizes the search word first.
- Then compares that normalized form to the normalized keywords in `docs_gin`.
- This ensures consistent matching.

### 5.11 Prefer the stem when the user searches for `teaching`

```
SELECT COALESCE((SELECT stem FROM docs_stem WHERE word=lower('teaching')), lower('teaching'));
```

### Explanation
- Searches the stem table for `teaching`.
- Finds `teach`.
- Returns `teach` instead of `teaching`.

### 5.12 Search for a word through its stem

```
SELECT DISTINCT id, doc FROM docs AS D
JOIN docs_gin AS G ON D.id = G.doc_id
WHERE G.keyword = COALESCE((SELECT stem FROM docs_stem WHERE word=lower('teaching')), lower('teaching'));
```

### Explanation
- If the user types `teaching`, the query normalizes it to `teach`.
- Because the index stores `teach`, the relevant document is still found.

### 5.13 Conflation

The comment in the SQL file says:

> The technical term for converting search terms to their stems is called **conflation**.

That means multiple surface forms such as `teaches`, `teaching`, and `teach` are treated as the same searchable concept.

---

## 6. PostgreSQL built-in full-text search

The previous sections manually recreated features that PostgreSQL already provides.

The built-in approach is:
- easier to write,
- more efficient,
- language-aware.

### 6.1 Convert documents to `tsvector`

```
SELECT to_tsvector('english', 'This is SQL and Python and other fun teaching stuff');
SELECT to_tsvector('english', 'More people should learn SQL from UMSI');
SELECT to_tsvector('english', 'UMSI also teaches Python and also SQL');
```

### Explanation
- `to_tsvector(config, text)` converts text into a searchable full-text representation.
- `'english'` tells PostgreSQL to use English-language rules.
- Those rules typically:
  - lowercase words,
  - remove stop words,
  - stem words,
  - store token positions.

So `to_tsvector` does automatically what earlier sections did manually.

### 6.2 Convert search text to `tsquery`

```
SELECT to_tsquery('english', 'teaching');
SELECT to_tsquery('english', 'teaches');
SELECT to_tsquery('english', 'and');
SELECT to_tsquery('english', 'SQL');
SELECT to_tsquery('english', 'Teach | teaches | teaching | and | the | if');
```

### Explanation

#### `to_tsquery('english', 'teaching');`
- Normalizes `teaching` using English stemming rules.
- It typically becomes a query for the stem `teach`.

#### `to_tsquery('english', 'teaches');`
- Also normalizes to the same stem.
- This shows how PostgreSQL performs conflation automatically.

#### `to_tsquery('english', 'and');`
- `and` is a stop word in English text search.
- It may disappear or produce an empty/ignored search representation.

#### `to_tsquery('english', 'SQL');`
- Normalizes the term for search.
- Even terms without stemming still become part of a `tsquery` value.

#### `to_tsquery('english', 'Teach | teaches | teaching | and | the | if');`
- `|` means OR.
- The English configuration stems related terms and removes stop words such as `and`, `the`, and `if`.
- The result is a logical search query over meaningful normalized terms.

### 6.3 Plain-text query parsing

```
SELECT plainto_tsquery('english', 'SQL Python');
SELECT plainto_tsquery('english', 'Teach teaches teaching and the if');
```

### Explanation
- `plainto_tsquery` treats input as normal text, not as a query syntax string.
- PostgreSQL extracts useful words and combines them into a search query.
- Stop words are removed.
- Stemming is applied.

This is safer and friendlier for user-entered search strings than raw `to_tsquery`.

### 6.4 Phrase search

```
SELECT phraseto_tsquery('english', 'SQL Python');
```

### Explanation
- `phraseto_tsquery` searches for words appearing in order as a phrase.
- This is different from simple keyword matching.
- It is closer to how users expect quoted text search to behave.

### 6.5 Web-style search syntax

```
SELECT websearch_to_tsquery('english', 'SQL -not Python');
```

### Explanation
- `websearch_to_tsquery` accepts user-friendly search syntax inspired by web search.
- `-not` means exclusion.
- This makes it easier to build search interfaces that feel familiar to users.

### 6.6 Match a query against a vector

```
SELECT to_tsquery('english', 'teaching') @@
  to_tsvector('english', 'UMSI also teaches Python and also SQL');
```

### Explanation
- `@@` is the full-text match operator.
- Left side: a normalized search query.
- Right side: a normalized searchable representation of the document.
- Because both sides apply English stemming, `teaching` can match `teaches`.

This is the built-in version of the stemming logic we manually implemented earlier.

---

## 7. A real full-text `GIN` index with `tsvector`

This is the most practical section in the file.

```
DROP TABLE docs cascade;
DROP INDEX gin1;

CREATE TABLE docs (id SERIAL, doc TEXT, PRIMARY KEY(id));
CREATE INDEX gin1 ON docs USING gin(to_tsvector('english', doc));
```

### Explanation

#### `DROP TABLE docs cascade;`
- Removes the existing `docs` table and dependents.

#### `DROP INDEX gin1;`
- Removes the older index so the new one can be created cleanly.

#### `CREATE TABLE docs (id SERIAL, doc TEXT, PRIMARY KEY(id));`
- Recreates the document table.

#### `CREATE INDEX gin1 ON docs USING gin(to_tsvector('english', doc));`
- Creates a `GIN` index on the full-text representation of `doc`.
- PostgreSQL now indexes normalized linguistic tokens, not raw strings.
- This is the standard pattern for fast full-text search in PostgreSQL.

### 7.1 Insert sample and filler rows

```
INSERT INTO docs (doc) VALUES
('This is SQL and Python and other fun teaching stuff'),
('More people should learn SQL from UMSI'),
('UMSI also teaches Python and also SQL');

INSERT INTO docs (doc) SELECT 'Neon ' || generate_series(10000,20000);
```

### Explanation
- Inserts the same three meaningful sample documents.
- Adds many filler rows to make query-plan differences easier to observe.

### 7.2 Search using the full-text index

```
SELECT id, doc FROM docs WHERE
    to_tsquery('english', 'learn') @@ to_tsvector('english', doc);
EXPLAIN SELECT id, doc FROM docs WHERE
    to_tsquery('english', 'learn') @@ to_tsvector('english', doc);
```

### Explanation

#### `SELECT id, doc FROM docs WHERE ...`
- Converts `learn` into a `tsquery`.
- Converts `doc` into a `tsvector`.
- Uses `@@` to test for a full-text match.
- Returns documents containing the concept `learn` after normalization.

#### `EXPLAIN ...`
- Shows how PostgreSQL plans to execute the query.
- On a sufficiently large table, the `GIN` index should help speed up the search.

---

## 8. Inspecting PostgreSQL index capabilities

The final statements step back and look at the database engine itself.

```
SELECT version();
```

### Explanation
- Returns the PostgreSQL server version.
- Useful because available full-text and indexing features can vary by version.

```
SELECT am.amname AS index_method, opc.opcname AS opclass_name
    FROM pg_am am, pg_opclass opc
    WHERE opc.opcmethod = am.oid
    ORDER BY index_method, opclass_name;
```

### Explanation
- `pg_am` is a system catalog describing access methods such as `btree`, `hash`, `gin`, and `gist`.
- `pg_opclass` describes operator classes.
- `WHERE opc.opcmethod = am.oid` connects each operator class to its index method.
- `ORDER BY` sorts the output for easier reading.

This query helps you inspect which operator classes PostgreSQL supports for different index types.

---

## 9. What the whole lecture is teaching

`lec05.sql` is not just a collection of syntax examples. It tells a progression:

1. **Split text into words** using `string_to_array` and `unnest`.
2. **Build a manual inverted index** with `(keyword, doc_id)` pairs.
3. Notice the problems of raw text search:
   - duplicate words,
   - uppercase vs lowercase,
   - stop words,
   - different forms of the same word.
4. Improve the index with:
   - `lower(...)`,
   - a `stop_words` table,
   - a `docs_stem` dictionary,
   - `COALESCE(...)` to prefer stems.
5. Replace the manual system with PostgreSQL's built-in full-text features:
   - `to_tsvector`,
   - `to_tsquery`,
   - `plainto_tsquery`,
   - `phraseto_tsquery`,
   - `websearch_to_tsquery`,
   - `GIN` indexes.

The central lesson is this:

> A simple keyword index can be built with ordinary SQL, but PostgreSQL full-text search is better because it is language-aware, faster, and easier to maintain.

---

## 10. Useful terms from this lecture

- **Tokenization**: splitting text into words or tokens.
- **Inverted index**: a structure mapping terms to the documents that contain them.
- **Stop words**: common low-value words such as `and`, `the`, and `is`.
- **Stemming**: reducing related words to a common root, such as `teaching` → `teach`.
- **Conflation**: treating multiple related word forms as the same searchable concept.
- **`GIN` index**: a PostgreSQL index type designed for composite values such as arrays and full-text search vectors.
- **`tsvector`**: PostgreSQL's internal full-text representation of a document.
- **`tsquery`**: PostgreSQL's internal full-text representation of a search query.

---

## 11. Practical takeaway

If you are experimenting or teaching concepts, the manual `docs_gin` table is useful because it shows how search works internally.

If you are building a real application, the important pattern from this file is usually:

```
CREATE INDEX gin1 ON docs USING gin(to_tsvector('english', doc));

SELECT id, doc
FROM docs
WHERE to_tsquery('english', 'learn') @@ to_tsvector('english', doc);
```

That gives you the benefits of:
- tokenization,
- case normalization,
- stop-word filtering,
- stemming,
- index-assisted search.

