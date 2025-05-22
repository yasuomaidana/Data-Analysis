## String Fields
Understand character sets and are indexable for searching.
- `CHAR(n)` allocates the entire space (faster for small strings where the length is known)
- `VARCHAR(n)` allocates a variable amount of space depending on the data length (less space)
> There is **no performance difference** among these three types, apart from increased storage space when using the blank-padded type, and a few extra CPU cycles to check the length when storing into a length-constrained column

[See](https://www.postgresql.org/docs/current/datatype-character.html)

## Text Fields
Have a character set - paragraphs or HTML pages.
- `TEXT` varying length

Generally **not used with indexing or sorting**, and only then limited to a prefix
## Binary Types (rarely used)
- Character = 8 - 32 bits of information, depending on the character set
- Byte = 8 bits of information
	- `BYTEA(n)` up to 255 bytes
- Small Images - data
- Not indexed or sorted
## Integer Numbers
Integer numbers are very efficient, take little storage, and are easy to process because CPUs can often compare them with a single instruction.
- `SMALLINT` (-32768, +32768)
- `INTEGER` (2 Billion)
- `BIGINT` - ($10^{18}$ ish)
[See](https://www.postgresql.org/docs/current/datatype-numeric.html)
## Floating Point Numbers.
Floating-point numbers can represent a wide range of values, but their accuracy is limited.
- `REAL` (32-bit) $10^{38}$ with seven digits of accuracy
- `DOUBLE PRECISION` (64-bit) $10^{308}$ with 14 digits of accuracy
- `NUMERIC(accuracy, decimal)` - Specified digits of accuracy and digits after the decimal point
## Dates
- `TIMESTAMP` - 'YYYY-MM-DD HH:MM:SS' (4713 BC, 294276 AD) 
- `DATE` - 'YYYY-MM-DD'
- `TIME` - 'HH:MM:SS'
- Built-in PostgreSQL function `NOW`
## Summary Table

| Name                                            | Aliases                        | Description                                                        |
| ----------------------------------------------- | ------------------------------ | ------------------------------------------------------------------ |
| `bigint`                                        | `int8`                         | signed eight-byte integer                                          |
| `bigserial`                                     | `serial8`                      | autoincrementing eight-byte integer                                |
| ``bit [ (_`n`_) ]``                             |                                | fixed-length bit string                                            |
| ``bit varying [ (_`n`_) ]``                     | ``varbit [ (_`n`_) ]``         | variable-length bit string                                         |
| `boolean`                                       | `bool`                         | logical Boolean (true/false)                                       |
| `box`                                           |                                | rectangular box on a plane                                         |
| `bytea`                                         |                                | binary data (“byte array”)                                         |
| ``character [ (_`n`_) ]``                       | ``char [ (_`n`_) ]``           | fixed-length character string                                      |
| ``character varying [ (_`n`_) ]``               | ``varchar [ (_`n`_) ]``        | variable-length character string                                   |
| `cidr`                                          |                                | IPv4 or IPv6 network address                                       |
| `circle`                                        |                                | circle on a plane                                                  |
| `date`                                          |                                | calendar date (year, month, day)                                   |
| `double precision`                              | `float8`                       | double precision floating-point number (8 bytes)                   |
| `inet`                                          |                                | IPv4 or IPv6 host address                                          |
| `integer`                                       | `int`, `int4`                  | signed four-byte integer                                           |
| ``interval [ _`fields`_ ] [ (_`p`_) ]``         |                                | time span                                                          |
| `json`                                          |                                | textual JSON data                                                  |
| `jsonb`                                         |                                | binary JSON data, decomposed                                       |
| `line`                                          |                                | infinite line on a plane                                           |
| `lseg`                                          |                                | line segment on a plane                                            |
| `macaddr`                                       |                                | MAC (Media Access Control) address                                 |
| `macaddr8`                                      |                                | MAC (Media Access Control) address (EUI-64 format)                 |
| `money`                                         |                                | currency amount                                                    |
| ``numeric [ (_`p`_, _`s`_) ]``                  | ``decimal [ (_`p`_, _`s`_) ]`` | exact numeric of selectable precision                              |
| `path`                                          |                                | geometric path on a plane                                          |
| `pg_lsn`                                        |                                | PostgreSQL Log Sequence Number                                     |
| `pg_snapshot`                                   |                                | user-level transaction ID snapshot                                 |
| `point`                                         |                                | geometric point on a plane                                         |
| `polygon`                                       |                                | closed geometric path on a plane                                   |
| `real`                                          | `float4`                       | single precision floating-point number (4 bytes)                   |
| `smallint`                                      | `int2`                         | signed two-byte integer                                            |
| `smallserial`                                   | `serial2`                      | autoincrementing two-byte integer                                  |
| `serial`                                        | `serial4`                      | autoincrementing four-byte integer                                 |
| `text`                                          |                                | variable-length character string                                   |
| ``time [ (_`p`_) ] [ without time zone ]``      |                                | time of day (no time zone)                                         |
| ``time [ (_`p`_) ] with time zone``             | `timetz`                       | time of day, including time zone                                   |
| ``timestamp [ (_`p`_) ] [ without time zone ]`` |                                | date and time (no time zone)                                       |
| ``timestamp [ (_`p`_) ] with time zone``        | `timestamptz`                  | date and time, including time zone                                 |
| `tsquery`                                       |                                | text search query                                                  |
| `tsvector`                                      |                                | text search document                                               |
| `txid_snapshot`                                 |                                | user-level transaction ID snapshot (deprecated; see `pg_snapshot`) |
| `uuid`                                          |                                | universally unique identifier                                      |
| `xml`                                           |                                | XML data                                                           |
[Documentation](https://www.postgresql.org/docs/current/datatype.html)