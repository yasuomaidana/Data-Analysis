Databases are designed to accept SQL commands from a variety of sources simultaneously and make them atomically

## Transactions and Atomicity
To implement atomicity, PostgreSQL "locks" areas before it starts an SQL command that might change an area of the database. All other access to that area must wait until the area is unlocked
## Single SQL Statements are Atomic
- All the inserts will work and get a unique primary key
- Which account gets which key is not predictable
## Compound Statements
Some statements perform more than one action within a single statement for efficiency and concurrency.
```postgresql
INSERT INTO fav (post_id, account_id, howmuch)
	VALUES (1,1,1)
	RETURNING *; -- Inserts and returns the values (two actions: insert, return)
```
## ON CONFLICT
Sometimes you "bump into" a constraint on purpose. Similar to `try-except` in Python.
```postgresql
INSERT INTO fav (post_id, account_id, hownuch) VALUES (1,1,1) -- instruction
	ON CONFLICT (post_id, account_id) -- error/conflict
	DO UPDATE SET howmuch = fav.howmuch + 1 -- what to do
RETURNING *;
```
## Multi-Statement Transactions

PostgreSQL treats every SQL statement as being executed within a transaction. If you do not issue a `BEGIN` command, then each individual statement has an implicit `BEGIN` and (if successful) `COMMIT` wrapped around it. A group of statements surrounded by `BEGIN` and `COMMIT` is sometimes called a _transaction block_.

> Some client libraries issue `BEGIN` and `COMMIT` commands automatically, so that you might get the effect of transaction blocks without asking. Check the documentation for the interface you are using.

It's possible to control the statements in a transaction in a more granular fashion through the use of _savepoints_. Savepoints allow you to discard parts of the transaction while committing the rest selectively. After defining a savepoint with `SAVEPOINT`, you can, if needed, roll back to the savepoint with `ROLLBACK TO`. All the transaction's database changes between defining the savepoint and rolling back to it are discarded, but changes earlier than the savepoint are kept.

[See documentation](https://www.postgresql.org/docs/current/tutorial-transactions.html)
### Example
`FOR UPDATE OF` locks the selection and doesn't enable other commands to use it until the current `BEGIN` block is rolled back or committed.
```postgresql
BEGIN;
SELECT howmuch FROM fav WHERE account_id=1 AND post_id=1 FOR UPDATE OF fav;
-- Time passes.
UPDATE SET howmuch=999 WHERE account_id=1 AND post_id=1;
ROLLBACK;
SELECT howmuch FROM fav WHERE account_id=1 AND post_id=1;
```
Second query, trying to modify the selected rows in the previous block.
```postgresql
BEGIN;
SELECT howmuch FROM fav WHERE account_id=1 AND post_id=1 FOR UPDATE OF fav;
-- Time passes..•
UPDATE SET howmuch=999 WHERE account_id=1 AND post_id=1;
COMMIT;
SELECT howmuch FROM fav WHERE account_id=1 AND post_id=1;
```
## Transactions and Performance
The implementation of transactions makes a big difference in database performance
- Lock granularity
- Lock implementation
## Transaction Topics
- Lock strength UPDATE, NO KEY UPDATE
- What to do when encountering a lock (WAIT), NOWAIT, SKIP LOCKED
