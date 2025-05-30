A query within a query. It can use a value or set of values in a query that are computed by another query
```postgresql
SELECT * FROM account
	WHERE email='a@b.c'; -- Query 1

SELECT content FROM comment
	WHERE account_id=1; -- Query 2

SELECT content FROM comment -- Query 2 contains query 1 (subquery)
	WHERE (SELECT * FROM account WHERE email='a@b.c'); -- query 1
```
## HAVING clause

You should try to use `HAVING` as much as possible because it is more efficient. For example, the following command can be achieved using subqueries.
```postgresql
SELECT COUNT (abbrev) AS ct, abbrev FROM pg_timezone_names 
		WHERE is_dst= 't' --filters grouping
			GROUP BY abbrey HAVING COUNT (abbrev) > 10; --groups after 
```
In the example below, we are using subqueries, and you can see how this process is less efficient than using the HAVING clause. In the example above, the process is not halted; it is executed in a single operation. However, when using subqueries, we split the process by generating temporary tables.
```postgresql
SELECT ct, abbrev 
	FROM(
		-- Sub query, the process is halted here until this part finishes.
		SELECT COUNT(abbrev) AS ct, abbrev FROM pg_timezone_names 
		WHERE is_dst='t' GROUP BY abbrev
	)AS zap WHERE ct>10 ORDER BY ct;
```
