## Functions

| Function                                                       | Return Type | Description                                                                                                                                                                   | Example                                       | Result     |
| -------------------------------------------------------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- | ---------- |
| string \| string                                               | text        | String concatenation                                                                                                                                                          | 'Post' \| 'greSQL'                            | PostgreSQL |
| string \| non-string or non-string \| string                   | text        | String concatenation with one non-string input                                                                                                                                | 'Value: ' \| 42                               | Value: 42  |
| `bit_length(string)`                                           | int         | Number of bits in string                                                                                                                                                      | bit_length('jose')                            | 32         |
| `char_length(string)` or `character_length(string)`            | int         | Number of characters in string                                                                                                                                                | char_length('jose')                           | 4          |
| `lower(string)`                                                | text        | Convert string to lower case                                                                                                                                                  | lower('TOM')                                  | tom        |
| `octet_length(string)`                                         | int         | Number of bytes in string                                                                                                                                                     | octet_length('jose')                          | 4          |
| `overlay(string placing string from int [for int])`            | text        | Replace substring                                                                                                                                                             | overlay('Txxxxas' placing 'hom' from 2 for 4) | Thomas     |
| `position(substring in string)`                                | int         | Location of specified substring                                                                                                                                               | position('om' in 'Thomas')                    | 3          |
| `substring(string [from int] [for int])`                       | text        | Extract substring                                                                                                                                                             | substring('Thomas' from 2 for 3)              | hom        |
| `substring(string from pattern)`                               | text        | Extract substring matching POSIX regular expression. See [Section 9.7](https://www.postgresql.org/docs/9.1/functions-matching.html) for more information on pattern matching. | substring('Thomas' from '...$')               | mas        |
| `substring(string from pattern for escape)`                    | text        | Extract substring matching SQL regular expression. See [Section 9.7](https://www.postgresql.org/docs/9.1/functions-matching.html) for more information on pattern matching.   | substring('Thomas' from '%#"o_a#"_' for '#')  | oma        |
| `trim([leading \| trailing \| both] [characters] from string)` | text        | Remove the longest string containing only the characters (a space by default) from the start/end/both ends of the string                                                      | trim(both 'x' from 'xTomxx')                  | Tom        |
| `upper(string)`                                                | text        | Convert string to upper case                                                                                                                                                  | upper('tom')                                  | TOM        |
## Pattern matching
### Like
```postgresql
string LIKE pattern [ESCAPE escape-character]
string NOT LIKE pattern [ESCAPE escape-character]
```
The `LIKE` expression returns true if the _`string`_ matches the supplied _`pattern`_. (As expected, the `NOT LIKE` expression returns false if `LIKE` returns true, and vice versa. An equivalent expression is ``NOT (_`string`_ LIKE _`pattern`_)``.)
### `SIMILAR TO` Regular Expressions
```postgresql
string SIMILAR TO pattern [ESCAPE escape-character]
string NOT SIMILAR TO pattern [ESCAPE escape-character]
```
The `SIMILAR TO` operator returns true or false depending on whether its pattern matches the given string. It is similar to `LIKE`, except that it interprets the pattern using the SQL standard's definition of a regular expression. SQL regular expressions are a curious cross between `LIKE` notation and common (POSIX) regular expression notation.

Like `LIKE`, the `SIMILAR TO` operator succeeds only if its pattern matches the entire string; this is unlike common regular expression behavior where the pattern can match any part of the string. Also like `LIKE`, `SIMILAR TO`uses `_` and `%` as wildcard characters denoting any single character and any string, respectively (these are comparable to `.` and `.*` in POSIX regular expressions).

In addition to these facilities borrowed from `LIKE`, `SIMILAR TO` supports these pattern-matching metacharacters borrowed from POSIX regular expressions:

- `|` denotes alternation (either of two alternatives).
- `*` denotes repetition of the previous item zero or more times.
- `+` denotes repetition of the previous item one or more times.
- `?` denotes repetition of the previous item zero or one time.
- `{`_`m`_`}` denotes repetition of the previous item exactly _`m`_ times.
- `{`_`m`_`,}` denotes repetition of the previous item _`m`_ or more times.
- `{`_`m`_`,`_`n`_`}` denotes repetition of the previous item at least _`m`_ and not more than _`n`_ times.
- Parentheses `()` can be used to group items into a single logical item.
- A bracket expression `[...]` specifies a character class, just as in POSIX regular expressions.
    
> Notice that the period (`.`) is not a metacharacter for `SIMILAR TO`.
- 
See more [here](https://www.postgresql.org/docs/current/functions-matching.html)
