## Strategy 1: Limiting URL Length
This initial attempt uses a `VARCHAR(128)` type for the `url` column, enforcing a maximum length. This can save space but poses a problem for longer URLs, as demonstrated by the error during insertion.
```postgresql
CREATE TABLE cr1(
	id SERIAL,
	url VARCHAR(128) UNIQUE,
	content TEXT
);
```
The following `INSERT` statement attempts to generate very long URLs, which then causes an error because they exceed the `VARCHAR(128)` limit. This highlights the limitation of using a fixed-length `VARCHAR` for potentially long URLs.
```shell
discuss=> insert into crl(url)
discuss-> select repeat ('Neon', 1000) || generate_series (1,5000) ;
ERROR: value too long for type character varying (128)
discuss=>
```
## Strategy 2: Flexible URL Length with Direct Indexing
This strategy allows for URLs of arbitrary length by using the `TEXT` data type. It then explores two indexing approaches: a direct `UNIQUE` index on the `url` column and an `MD5` hash index on the `url` column.
```postgresql
CREATE TABLE cr2(
	id SERIAL,
	url TEXT,
	content TEXT
);

-- Index on the full URL column
CREATE UNIQUE INDEX cr2_unique ON cr2(url);

-- MD5 hash index on the URL
CREATE UNIQUE INDEX cr2_md5 ON cr2(md5(url));
```
## Strategy 3: Storing MD5 Hashes Separately
This approach pre-calculates and stores the MD5 hash of the URL in a separate `UUID` column. A `UNIQUE` constraint is then applied to this `url_md5` column, which can be beneficial for very long URLs, as indexing a fixed-size UUID is often more efficient than indexing a variable-length `TEXT` field.
```postgresql
CREATE TABLE cr3(
	id SERIAL,
	url TEXT,
	url_md5 UUID UNIQUE,
	content TEXT
);

-- WHAT ARE WE DOING HERE?
UPDATE cr3 SET url_md5 = md5(url)::uuid;
```
**What are we doing here?** The `UPDATE` statement calculates the MD5 hash of each `url` and casts it to a `UUID` type, then stores it in the `url_md5` column. This pre-computation ensures that the `url_md5` column is populated with unique, fixed-size identifiers derived from the URLs, which can be efficiently indexed.
## Strategy 4: Hash Indexing on URL Column
This strategy uses a **hash index** directly on the `url` column. Hash indexes are optimized for equality lookups and can be very fast for exact matches, but they do not support range scans or ordering.
```postgresql
CREATE TABLE cr4(
	id SERIAL, 
	url TEXT, 
	content TEXT
);

CREATE INDEX cr4_hash ON cr4 USING hash(url);
```

## Comparison
|           Strategy           | Relation Size | Index size | `SELECT` time |
| :--------------------------: | :-----------: | :--------: | :-----------: |
|       `cr2` No `INDEX`       |    507904     |     0      |    1.784ms    |
| `cr2` `MD5` `INDEX` on `url` |    507904     |   311296   |    0.142ms    |
|           cr3 NAME           |    1097728    |   368640   |    0.030ms    |
|           cr4 NAME           |    507904     |   278528   |    0.045ms    |
