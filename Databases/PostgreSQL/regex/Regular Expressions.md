- A text-based programming language
- Clever wild-card strings for matching and parsing text
- PostgreSQL uses the POSIX variant
## Quick guide

|   Symbol   | Meaning                                             |
| :--------: | :-------------------------------------------------- |
|    `^`     | Matches the beginning of a line                     |
|    `$`     | Matches the end of the line                         |
|    `.`     | Matches any character                               |
|    `*`     | Repeats a character zero or more times              |
|    `*?`    | Repeats a character zero or more times (non-greedy) |
|    `+`     | Repeats a character one or more times               |
|    `+?`    | Repeats a character one or more times (non-greedy)  |
| `[aeiou]`  | Matches a single character in the listed set        |
|  `[^XYZ]`  | Matches a single character not in the listed set    |
| `[a-Z0-9]` | The set of characters can include a range           |
|    `(`     | Indicates where the string extraction is to start   |
|    `)`     | Indicates where the string extraction is to end     |
>  Let's **match** `<em>Hello World</em>`
>  `<.+>` is greedy then it will **match** `<em>Hello World</em>` since it is the longest string enclosed by `<`and `>`
>  Instead, `<.+?>` will **match** `<em>` and `</em>`
## POSIX  ReGex

| Operator | Description                                         | Example                   |
| :------: | --------------------------------------------------- | ------------------------- |
|   `~`    | Matches regular expression, case sensitive          | 'thomas' ~ '.*thomas.*'   |
|   `~*`   | Matches regular expression, case insensitive        | 'thomas' ~\* '.*Thomas.*' |
|   `!~`   | Does not match regular expression, case sensitive   | 'thomas' !~ '.*Thomas.*'  |
|  `!~*`   | Does not match regular expression, case insensitive | 'thomas' !~\* '.*vadim.*' |
## `LIKE`, `ILIKE`, and Regex on Postgresql

| Feature                 | `LIKE`                                    | `ILIKE`                                   | Regular Expressions (`~`, `~*`)                          |
| :---------------------- | :---------------------------------------- | :---------------------------------------- | :------------------------------------------------------- |
| **Basic Purpose** | Simple string pattern matching.           | Simple string pattern matching (case-insensitive). | Complex pattern matching, text extraction, and validation. |
| **Wildcards / Metacharacters** | `%` (zero or more chars), `_` (single char) | `%` (zero or more chars), `_` (single char) | POSIX-style metacharacters (e.g., `.` `*` `+` `?` `[]` `()` `|` `^` `$`, `\d`, `\w`, `\s`, etc.) |
| **Case Sensitivity** | **Case-sensitive** by default.           | **Case-insensitive** by default.         | `~` is **case-sensitive**; `~*` is **case-insensitive**. |
| **SQL Standard** | Yes (ANSI SQL standard).                  | No (PostgreSQL extension).                | No (PostgreSQL extension, based on POSIX standard).      |
| **Index Support** | Good for prefix matches (`'prefix%'`) with B-tree index (especially with `text_pattern_ops`). Poor for leading wildcards. | Similar to `LIKE` for prefix matches with `text_pattern_ops` (if collation allows). | Can use B-tree for anchored patterns (`'^prefix'`). `pg_trgm` extension with GIN/GiST indexes can improve performance for complex/unanchored patterns. |
| **Performance (General)** | Generally faster for simple patterns, especially prefix matches that can use an index. | Slightly slower than `LIKE` due to case conversion overhead. | Generally slower due to the overhead of a more powerful engine. Performance varies greatly with pattern complexity. |
| **Complexity** | Simple to learn and use for basic needs.  | Simple to use.                             | More powerful, but steeper learning curve due to a wide range of metacharacters and concepts. |
| **Matching Scope** | Matches the **entire string** by default (unless `%` is at both ends). | Matches the **entire string** by default. | Matches **anywhere** within the string by default (unless anchored with `^` and `$`). |
| **Common Use Cases** | Finding names starting with 'J', emails ending with '.com', simple IDs. | Same as `LIKE`, but when case doesn't matter (e.g., user input). | Validating email formats, parsing log files, extracting structured data from text, complex search patterns. |
