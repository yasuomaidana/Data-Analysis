## 1. Creating the tables
```postgresql
CREATE TABLE textfun (
	content TEXT
);
CREATE INDEX textfun_b ON textfun(content);
```

**What does an index do?**

An index in a database is similar to an index in a book. It helps the database system find data in a table more quickly. When you create an index on a column (or multiple columns), the database stores a sorted list of the values in that column along with pointers to the corresponding rows. This significantly speeds up data retrieval operations (like `SELECT` statements) that involve searching or filtering on the indexed column, especially in large tables.
## 2. Checking table size
```postgresql
SELECT pg_relation_size('textfun'), pg_indexes_size('textfun');
```

## 3. Inserting data
```postgresql
INSERT INTO textfun(content)
SELECT 
	(
		CASE WHEN (random()<0.5)
		THEN 'buuu'
		ELSE 'yeiii'
		END
	) || generate_series(100000,200000)
```

# 4. Retrieving information
This query selects the `content` column from the `textfun` table. It filters the results to include only rows where the `content` column contains the string '150000' at the end. The `%` is a wildcard character that matches any sequence of zero or more characters.
```postgresql
SELECT content FROM textfun WHERE content LIKE '%150000'
```

EXAMPLE OUTPUT: `This is some content ending with 150000`

---
This query selects the `content` column from the `textfun` table, converting all characters in the `content` to uppercase using the `upper()` function. It filters the results to include only rows where the `content` column contains the string '150000' at the end.

```postgresql
SELECT upper(content) FROM textfun WHERE content LIKE '%150000'
```

EXAMPLE OUTPUT: `THIS IS SOME CONTENT ENDING WITH 150000`

---
This query selects the `content` column from the `textfun` table, converting all characters in the `content` to lowercase using the `lower()` function. It filters the results to include only rows where the `content` column contains the string '150000' at the end.
```postgresql
SELECT lower(content) FROM textfun WHERE content LIKE '%150000'
```

EXAMPLE OUTPUT: `this is some content ending with 150000

---
This query selects the last 4 characters of the `content` column from the `textfun` table using the `right()` function. It filters the results to include only rows where the `content` column contains the string '150000' anywhere within it.
```postgresql
SELECT right(content,4) FROM textfun WHERE content LIKE '%150000%';
```

EXAMPLE OUTPUT: `0000`

---
This query selects the first 4 characters of the `content` column from the `textfun` table using the `left()` function. It filters the results to include only rows where the `content` column contains the string '150000' anywhere within it.
```postgresql
SELECT left(content,4) FROM textfun WHERE content LIKE '%150000%';
```
EXAMPLE OUTPUT: `This`

---
This query finds the starting position of the substring 'buuu' within the `content` column using the `strpos()` function. If 'buuu' is not found, it returns 0. It filters the results to include only rows where the `content` column contains the string '150000' anywhere within it.
```postgresql
SELECT strpos(content,'buuu') FROM textfun WHERE content LIKE '%150000%';
```
EXAMPLE OUTPUT: 

| strpos |
| :----: |
|   1    |
|   0    |


> This example works when `SELECT * FROM textfun WHERE content LIKE '%150000%';` retrieves: 

| content                                    |
| ------------------------------------------ |
| buuu150000                                 |
| https://www.funnypage.com/happy/path150000 |


---
This query splits the `content` string by the '/' delimiter and returns the 4th part of the resulting array using the `split_part()` function. It filters the results to include only rows where the `content` column contains the string '150000' anywhere within it.
```postgresql
SELECT split_part(content,'/',4) FROM textfun WHERE content LIKE '%150000%';
```
EXAMPLE OUTPUT: ``

| split_part |
| :--------: |
|            |
|   happy    |

---
This query replaces occurrences of characters in the `content` string. It replaces `'t'` with `'T'`, `'h'` with `'H'`, `'.'` with `'!'`, `'p'` with `'P'`, and `'/'` with `'_'` using the `translate()` function. It filters the results to include only rows where the content column contains the string '150000' anywhere within it.
```postgresql
SELECT translate(content,'th.p/','TH!P_') FROM textfun WHERE content LIKE '%150000%';
```

EXAMPLE OUTPUT: 

|                 translate                  |
| :----------------------------------------: |
|                 buuu150000                 |
| HTTPs:__www!funnyPage!com_HaPPy_PaTH150000 |
