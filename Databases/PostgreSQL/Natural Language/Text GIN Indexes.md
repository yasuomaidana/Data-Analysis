# PostgreSQL GIN Indexes: TS-Vectors, Trigrams, and Arrays

This document compares three common approaches for indexing text data in PostgreSQL using GIN (Generalized Inverted Index) indexes: Full Text Search (`tsvector`), Trigrams (`gin_trgm_ops`), and Array indexing (`array_ops`).

## 1. Full Text Search (`tsvector`)

This is the "intelligent" approach. It doesn't just look at the characters; it understands the language. Instead of manual arrays, Postgres has a built-in engine that breaks text down into linguistic tokens.

* **Stemming**: It knows that "run," "running," and "ran" share the same root. If you search for "run," you'll find results containing "running."
* **Stop Words**: It ignores common words like "the," "is," and "at" which bloat indexes and don't help find specific content.
* **Ranking**: It provides a `ts_rank` so you can show the most relevant results at the top.

**SQL Example:**
```sql
CREATE INDEX idx_fts ON docs USING gin(to_tsvector('english', doc));
```

**Best for**: Articles, blog posts, or large bodies of natural language text where you want "Google-like" behavior.

## 2. Trigrams (`gin_trgm_ops`)

This is the "literal" approach. It uses the `pg_trgm` extension to break strings into trigrams (sequences of three consecutive characters). For example, indexing the word "apple" stores `a`, `ap`, `app`, `ppl`, `ple`, `le`.

* **Substrings**: It can find strings inside other strings, even if they aren't words. Searching for "berry" will find "blackberry" and "strawberry." `tsvector` would miss these because it treats them as distinct words.
* **Fuzzy Matching**: It handles typos exceptionally well. If a user types "appple," trigrams can still find "apple" based on character similarity.
* **Language Agnostic**: It doesn't care about grammar or language rules; it just looks at character patterns.

**Query Support**: Works with `LIKE '%search%'`, `ILIKE`, and similarity operators (`%`).

**Best for**: Search-as-you-type bars, partial matches, names, SKUs, or source code where word roots don't matter. It is the gold standard in Postgres for flexible text searching without moving to a full-blown search engine like Elasticsearch.

*(Note: Don't forget to run `CREATE EXTENSION IF NOT EXISTS pg_trgm;` first, or the `gin_trgm_ops` operator class won't exist!)*

## 3. The Array Approach (`array_ops` via `string_to_array`)

This is a Functional Index. It manually forces the text into a standard PostgreSQL array by splitting the string at every space character. It turns the string "apple orange banana" into `{'apple', 'orange', 'banana'}` and indexes those specific elements.

* **Exact Match Only**: Searching for "apple" won't find "apples".
* **Query Support**: Works with array operators like "contains" (`@>`), "is contained by" (`<@`), or "overlaps" (`&&`).

**Best for**: Exact word matching in a simple list or keyword tagging where you don't care about partial matches.

---

## Comprehensive Comparison

| Feature | Full Text Search (`tsvector`) | Trigrams (`gin_trgm_ops`) | Array (`array_ops`) |
| :--- | :--- | :--- | :--- |
| **Search Granularity** | Linguistic (Words/Roots) | Character-level (3-char chunks) | Word-level (delimited by space) |
| **Handles Typos?** | No | Yes | No |
| **Handles Word Variations?**| Yes ("Running" vs "Run") | No | No |
| **Partial Matches** | Not natively | Excellent (`LIKE '%cat%'`) | None (must match full array element) |
| **Index Size** | Smaller (ignores stop words)| Usually larger (more entries) | Smaller |
| **Performance (Writes)** | Slower due to processing | Slower on heavy writes | Faster |
| **Primary Use Case** | Articles, Documents, Prose | Fuzzy search, Auto-complete | Keyword/Tag filtering |

## The Verdict

Which is "better" depends entirely on your query patterns:

* **Use `tsvector` if**: You are searching through human-written prose (descriptions, comments, articles) and you want the search to feel "smart" by understanding word variations.
* **Use `gin_trgm_ops` if**: You need to support partial matches (like a serial number ABC-123 via `LIKE '%123%'`) or if you want to be forgiving of user typos.
* **Use `array_ops` if**: You are strictly matching tags or specific tokens and want maximum performance. However, if you are doing this, you are usually better off using Full Text Search (`tsvector`) instead.

**Pro Tip:** Many modern applications actually use both. They use `tsvector` for the primary search and `pg_trgm` to provide "Did you mean?" suggestions or to handle prefix matching as the user types in the search bar.

---

## Other Available GIN Options

If you are working with text or complex data, you should also consider these alternatives:

1. **JSONB Indexing (`jsonb_ops` / `jsonb_path_ops`)**: If your data is in JSON format, GIN is the default way to index keys and values for lightning-fast lookups.
2. **`btree_gin`**: A standard B-tree index is usually better for simple equals (`=`), but if you have a multi-column index where one column is a UUID and the other is a text array, the `btree_gin` extension allows you to combine them into one GIN index.

---

## Appendix: PostgreSQL Access Methods (Index Types)

PostgreSQL uses different access methods (index types) for different kinds of data and queries. These methods are cataloged internally in the `pg_am` system catalog. Here is a comparison of the primary access methods, their advantages, disadvantages, and use cases:

### 1. B-Tree (`btree`)
* **What it is**: The default and most common index type in PostgreSQL. It stores data in a balanced tree, maintaining strict ordering.
* **Advantages**: Exceptionally fast for exact matches (`=`) and range queries (`<`, `<=`, `>=`, `>`). It can also be used to retrieve data in sorted order (avoiding an explicit sort step).
* **Disadvantages**: Not suitable for complex composite data types like arrays, JSON documents, or full-text vectors where a single value contains multiple searchable elements.
* **Use Cases**: Primary keys, unique constraints, dates, numerical values, and standard text columns where you query exact matches or ranges.

### 2. Hash (`hash`)
* **What it is**: Uses a hash table to map keys to table rows.
* **Advantages**: Can be slightly faster and take up less space than B-Tree for simple equality (`=`) checks on very large tables.
* **Disadvantages**: Only supports equality (`=`) operations. It cannot handle range queries (`<`, `>`), sorting, or complex types. 
* **Use Cases**: Fast lookups on long strings (like session IDs, URLs, or UUIDs) where only exact equality is ever checked.

### 3. GIN (`gin` - Generalized Inverted Index)
* **What it is**: An "inverted index" designed to handle composite data types where one row contains multiple searchable values.
* **Advantages**: Excellent for searching for elements *inside* a container (e.g., does this array contain `X`? does this JSON document have key `Y`? does this text contain word `Z`?).
* **Disadvantages**: Slower to update than B-Tree. Heavy write operations (inserts/updates) can incur a performance penalty because one row change might require updating multiple index entries (e.g., one for each word in a document).
* **Use Cases**: Full-Text Search (`tsvector`), Arrays, JSONB, and Trigrams (`pg_trgm`).

### 4. GiST (`gist` - Generalized Search Tree)
* **What it is**: A flexible, tree-based structure that allows developers to define custom indexing rules for complex data types.
* **Advantages**: Supports "nearest-neighbor" searches and overlapping conditions. It is lossy, meaning it can keep index sizes very small (though it may require checking the actual row to confirm an exact match).
* **Disadvantages**: Slower to query than GIN for full-text search. Index creation and querying can be CPU-intensive.
* **Use Cases**: Geographic/Spatial data (PostGIS), overlapping ranges (e.g., scheduling/time ranges), and occasionally full-text search when disk space is severely limited or when updating speed is prioritized over read speed.