# JSON and JSONB in PostgreSQL

This summary explains the lecture script in [`06-JSON.sql`](https://www.pg4e.com/lectures/06-JSON) and the practical session on JSONB operations. It covers how to store, query, and manipulate semi-structured data within PostgreSQL.

The content covers four main areas:
1. **Importing JSON data** using `\copy`.
2. **Accessing fields** with the `->` and `->>` operators.
3. **Filtering and Searching** using containment and existence operators.
4. **Modifying JSONB** documents with merging and path updates.

---

## 1. Creating and Importing JSONB

PostgreSQL provides two JSON types: `JSON` (exact copy of input) and `JSONB` (binary representation). `JSONB` is generally preferred as it is faster to process and supports indexing.

```sql
CREATE TABLE jtrack (id SERIAL, body JSONB);

\copy jtrack (body) FROM 'library.jstxt' WITH CSV QUOTE E'\x01' DELIMITER E'\x02';
```

### Explanation
- **`CREATE TABLE ... JSONB`**: Defines a column to store JSON in an optimized binary format.
- **`\copy ... WITH CSV`**: A trick for importing JSON files where each line is a JSON object. By setting the `QUOTE` and `DELIMITER` to non-printing characters (`\x01`, `\x02`), PostgreSQL treats the entire line as a single field.

---

## 2. Accessing JSON Fields

PostgreSQL uses specific operators to navigate JSON structures.

```sql
SELECT body->'name' FROM jtrack;
SELECT body->>'name' FROM jtrack;
SELECT (body->>'count')::int FROM jtrack;
```

### What each operator does
- **`->`**: Extracts a JSON object or array. The result is still of type `JSONB`.
- **`->>`**: Extracts a field as **text**. This is essential when you want to use the value in `WHERE` clauses or for display.
- **Casting (`::int`)**: Even if a value is a number in JSON, `->>` returns it as text. You must explicitly cast it to `int` or `float` for numeric operations like `MAX()` or `ORDER BY`.

---

## 3. Filtering and Searching

`JSONB` supports specialized operators for searching inside the document.

```sql
-- Check for exact field match
SELECT * FROM jtrack WHERE body->>'name' = 'Summer Nights';

-- Containment operator (very efficient with GIN indexes)
SELECT * FROM jtrack WHERE body @> '{"name": "Summer Nights"}';

-- Key existence operator
SELECT * FROM jtrack WHERE body ? 'favorite';
```

### Explanation
- **`@>` (Contains)**: Checks if the left JSONB value contains the right JSONB path/value. This is the "gold standard" for searching JSONB because it can be accelerated by GIN indexes.
- **`?` (Key exists)**: Checks if a top-level key exists within the JSON object.

---

## 4. Modifying JSONB Data

JSONB is immutable in a sense that you replace the whole document, but PostgreSQL provides operators to make updates easy.

```sql
-- Merging/Adding data
UPDATE jtrack SET body = body || '{"favorite": "yes"}' WHERE (body->>'count')::int > 200;

-- Updating a specific path
UPDATE jtrack SET body = jsonb_set(body, '{ count }', '101'::jsonb)
WHERE body->>'name' = 'Summer Nights';
```

### Explanation
- **`||` (Concatenation/Merge)**: Merges two JSONB objects. If a key already exists, the right-hand value overwrites the left-hand one.
- **`jsonb_set(target, path, new_value)`**: Updates a specific value at a given path (e.g., `'{count}'`). The `new_value` must also be a `JSONB` type.

---

## 5. Summary of JSONB Operators

| Operator     | Return Type | Description                                       |
| :----------- | :---------- | :------------------------------------------------ |
| `->`         | `JSONB`     | Get JSON object/array by key or index.            |
| `->>`        | `text`      | Get JSON object field or array element as text.   |
| `@>`         | `boolean`   | Does the left JSONB value contain the right?      |
| `?`          | `boolean`   | Does the key string exist within the JSONB value? |
| &#124;&#124; | `JSONB`     | Concatenate/Merge two JSONB values.               |
| `-`          | `JSONB`     | Delete a key or index from a JSONB value.         |

---

## Practical Takeaway

When working with JSON in PostgreSQL:
1. **Always use `JSONB`** unless you have a very specific reason to preserve the exact whitespace/formatting of the input.
2. **Use `->>`** for filtering and display, but remember to **cast** to the appropriate type for math.
3. **Use `@>`** for searches that you want to index.
4. **Prefer `jsonb_set`** for targeted updates and the `||` operator for adding new tags or fields to existing records.
