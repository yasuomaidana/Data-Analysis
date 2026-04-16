The `substring()` gets the first match in a text column
We can get an array of matches using `regexp_matches()`

```postgresql
CREATE TABLE tw(
	id SERIAL PRIMARY KEY,
	tweet TEXT
);

INSERT INTO tw (tweet) VALUES ('This is #SQL and #FUN stuff');
INSERT INTO tw (tweet) VALUES ('More people should learn #SQL from #UMSI');
INSERT INTO tw (tweet) VALUES ('#UMSI also teaches #PYTHON' );
```

The `regexp_matches` function returns a set of text arrays of matching substring(s) within matches of a POSIX regular expression pattern to a string. It has the same syntax as `regexp_match`. This function returns no rows if there is no match, one row if there is a match and the `g` flag is not given, or _`N`_ rows if there are _`N`_ matches and the `g` flag is given. Each returned row is a text array containing the whole matched substring or the substrings matching parenthesized subexpressions of the _`pattern`_, just as described above for `regexp_match`. `regexp_matches` accepts all the flags shown in [Table 9.24](https://www.postgresql.org/docs/current/functions-matching.html#POSIX-EMBEDDED-OPTIONS-TABLE "Table 9.24. ARE Embedded-Option Letters"), plus the `g` flag, which commands it to return all matches, not just the first one.

> Greedy ⬇️
```postgresql
SELECT regexp_matches(tweet, '#(.+)[\s$]') FROM tw;
```

|     regex_matches     |
| :-------------------: |
|   {"SQL and #FUN"}    |
|     {"SQL from"}      |
| {"UMSI also teaches"} |

``` postgresql
SELECT regexp_matches(tweet, '#(.+?)[\s$]','g') FROM tw;
```

| regex_matches |
| :-----------: |
|     {SQL}     |
|     {FUN}     |
|     {SQL}     |
|    {UMSI}     |
