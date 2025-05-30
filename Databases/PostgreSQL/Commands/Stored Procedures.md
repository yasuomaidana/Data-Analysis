A stored procedure is a bit of reusable code that runs inside the database server.
Technically, there are multiple language choices, but just use "`plpgsql`".
Generally quite non-portable
Usually, the goal is to have fewer SQL statements

You should have a **strong reason** to use a stored procedure:
- Major performance problem
- Harder to test/modify
- No database portability
- Some rules that *must* be enforced

## Creating a stored procedure

```postgresql
CREATE OR REPLACE FUNCTION count_abbrev_counts(
									p_abbrev TEXT, 
									p_is_dst BOOLEAN)
	RETURNS BIGINT AS $$
	BEGIN
	RETURN (
		SELECT COUNT(abbrev) FROM pg_timezone_names 
			WHERE abbrev = p_abbrev and is_dst = p_is_dst);
	END;
$$ language plpgsql stable;
```
You can call and get the result using
```postgresql
select count_abbrev_counts('PDT', true);
```
You can also retrieve tables.
```postgresql
CREATE OR REPLACE FUNCTION get_timezone_abbrev_counts(
								p_is_dst BOOLEAN,
								p_min_count INTEGER)
	RETURNS TABLE(abbrev TEXT, ct BIGINT) AS $$
	BEGIN
	RETURN QUERY
		SELECT tmz.abbrev, COUNT(tmz.abbrev) AS abbreviation_count 
		FROM pg_timezone_names AS tmz
			WHERE tmz.is_dst = p_is_dst
			GROUP BY tmz.abbrev HAVING COUNT(tmz.abbrev) > p_min_count
		-- Using the alias from the SELECT list for clarity
		ORDER BY abbreviation_count; 
	END;
$$ LANGUAGE plpgsql STABLE;
```
You can call it using.
```postgresql
SELECT * FROM get_timezone_abbrev_counts(true, 20);
```

You can also use an `INOUT` variable, 
```postgresql
CREATE OR REPLACE PROCEDURE count_abbrev_counts(
	p_abbrev TEXT,
	p_is_dst BOOLEAN,
	-- INOUT parameter
	INOUT result_count BIGINT )
	AS $$
		BEGIN
		-- Assign the count to the INOUT parameter
		SELECT COUNT(tmz.abbrev) INTO result_count FROM pg_timezone_names AS tmz
			WHERE tmz.abbrev = p_abbrev AND tmz.is_dst = p_is_dst;
		END;
$$ LANGUAGE plpgsql;
```
You can call it using 
```postgresql
DO $$ 
DECLARE
	RESULT BIGINT := 0;
BEGIN
	CALL count_abbrev_counts('EST', false, result);
	RAISE NOTICE 'Count: %', result;
END;
$$;
```
## Triggers
They are used to enforce business rules, perform automatic actions, or maintain data consistency.
> Triggers are special types of stored procedures that automatically execute (or “fire”) in response to certain events on a table, such as **inserts, updates, or deletes**.
### Key Features
- **Automatic Execution:** Triggers are invoked automatically in response to specific database events.
- **Event Handling:** Can be set to fire before or after an insert, update, or delete operation.
- **Enforcement:** Useful for maintaining data integrity and automating tasks.
### Trigger Types
- **BEFORE Trigger:** Executes before the triggering event (insert, update, delete).
- **AFTER Trigger:** Executes after the triggering event.
- **INSTEAD OF Trigger:** Executes in place of the triggering event (mainly used for views).
### Example
1. Creating a Trigger Function
```postgresql
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
	RETURNS TRIGGER AS $$
	BEGIN
		NEW.updated_at = NOW();
	RETURN NEW;
	END;
$$ LANGUAGE plpgsql;
```
2. Creating a Trigger
```postgresql
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON fav
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
```
## Resources
[Power of Stored Procedures in PostgreSQL](https://medium.com/@mail2anajain/power-of-stored-procedures-in-postgres-345de42f5995)
[Stored Procedures, Functions, and Triggers In PostgreSQL](https://medium.com/byte-of-knowledge/stored-procedures-functions-and-triggers-in-postgresql-fddd0630b99c)
