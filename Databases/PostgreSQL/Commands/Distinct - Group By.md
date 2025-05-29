Reducing the result set
- `DISTINCT` only returns unique rows in a result set - and row will only appear once
- `DISTINCT ON` limits duplicate removal to a set of columns
- `GROUP BY` is combined with aggregate functions like `COUNT()`, `MAX()`, `SUM()`, `AVE()` ...
## Distinct
```postgresql
SELECT DISTINCT model FROM racing;
```
Returns only distinct model's values from racing
## Distinct on
```postgresql
SELECT DISTINCT ON(model) make, model FROM racing;
```
Returns make and model on distinct models
## Aggregate / `GROUP BY`
```postgresql
SELECT COUNT (abbrev), abbrev FROM pg_timezone_names GROUP BY abbrev;
```
## HAVING clause
```postgresql
SELECT COUNT (abbrev) AS ct, abbrev FROM pg_timezone_names 
		WHERE is_dst= 't' --filters grouping
			GROUP BY abbrey HAVING COUNT (abbrev) > 10; --groups after filtering, then applies having, you can't use where with COUNT
```
