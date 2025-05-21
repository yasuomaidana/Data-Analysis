## Create table

```postgresql
CREATE TABLE users(name VARCHAR(128), email VARCHAR(128));
```
## Inserting rows

```postgresql
INSERT INTO users (name, email) VALUES ('Somesh', 'some@mail.com');
```
## Delete rows

```postgresql
DELETE FROM users WHERE email='some@mail.com';
```
> **WARNING**: PostgreSQL will execute the functions DELETE, UPDATE, and SELECT on every row unless you specify which values you want to affect.
## Update
```postgresql
UPDATE users SET name='Charles' WHERE email='some@mail.com';
```
## Select

Get all
```postgresql
SELECT * FROM users;
```
Filter
```postgresql
SELECT *
FROM USers WHERE email='csev@umich.edu';
```
## Sorting

You can add an `ORDER BY` clause to `SELECT` statements to get the results sorted in ascending or descending order
```postgresql
SELECT
* FROM users
ORDER BY email;
```

## The LIKE Clause

We can do wildcard matching in a WHERE clause using the LIKE operator
```postgresql
SELECT * FROM users WHERE name LIKE '%e%';
```

## The LIMIT/OFFSET Clauses

- We can request the first `n` rows, or the first `n` rows after skipping some rows.
- The `WHERE` and `ORDER BY` clauses happen *before* the `LIMIT/OFFSET` are applied.
- The `OFFSET` starts from row `0`
```postgresql
SELECT * FROM users ORDER BY email DESC LIMIT 2;
SELECT * FROM users ORDER BY email OFFSET 1 LIMIT 2;
```
## Count
Counting Rows with `SELECT`
You can request to receive the count of the rows that would be retrieved instead of the rows
```postgresql
SELECT COUNT (*) FROM users;
SELECT COUNT (*) FROM users WHERE email='csev@umich.edu';
```
