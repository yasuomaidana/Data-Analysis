```postgresql
-- Creating table
CREATE TABLE em (
	id SERIAL PRIMARY KEY,
	email TEXT
);

-- Inserting data

INSERT INTO em (email) VALUES ('csev@umich.edu');
INSERT INTO em (email) VALUES ('coleen@umich.edu');
INSERT INTO em (email) VALUES ('sally@uiuc.edu');
INSERT INTO em (email) VALUES ('ted79@umuc.edu');
INSERT INTO em (email) VALUES ('glenn1@apple.com');
INSERT INTO em (email) VALUES ('nbody@apple.com');
```
## Using a simple match
```postgresql
SELECT email FROM em WHERE email ~ 'umich';
```
It will return any email that contains `umich`

|      email       |
| :--------------: |
|  csev@umich.edu  |
| coleen@umich.edu |
## Using regex
### Finding by starting letter 
```postgresql
SELECT email FROM em WHERE email ~ '^c';
```
It will return any email that starts with `c`

|      email       |
| :--------------: |
|  csev@umich.edu  |
| coleen@umich.edu |
#### Finding by starting letters (using `[]`)
```postgresql
SELECT email FROM em WHERE email ~ '^[gnt]';
```
It will return any email that starts with 'g', 'n', or 't' since they are enclosed by `[]`.

|        email         |
| :------------------: |
|    ted79@umuc.edu    |
| glenn1@apple.com<br> |
|   nbody@apple.com    |
It will return any email that starts with `c`
----
## Finding by ending letters/patterns
```postgresql
SELECT email FROM em WHERE email ~ 'edu$';
```

|      email       |
| :--------------: |
|  csev@umich.edu  |
| coleen@umich.edu |
|  sally@uiuc.edu  |
|  ted79@umuc.edu  |
## Finding patterns relax `[]`
```postgresql
SELECT email FROM em WHERE email ~ '[0-9]';
```

|      email       |
| :--------------: |
| glenn1@apple.com |
|  ted79@umuc.edu  |
We can make it more restrictive by forcing to have two numbers

```postgresql
SELECT email FROM em WHERE email ~ '[0-9][0-9]';

-- You can also write the above query the following way
SELECT email FROM em WHERE email ~ '[0-9]{2}';

```

|      email       |
| :--------------: |
|  ted79@umuc.edu  |

