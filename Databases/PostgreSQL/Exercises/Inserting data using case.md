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