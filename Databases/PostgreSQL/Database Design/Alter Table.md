```postgresql
CREATE TABLE account (
	id SERIAL,
	email VARCHAR (128) UNIQUE, 
	created_at DATE NOT NULL DEFAULT NOW()
	updated_at DATE NOT NULL DEFAULT NOW(),
	PRIMARY KEY (id)
);
	
CREATE TABLE post ( 
	id SERIAL,
	title VARCHAR (128) UNIQUE NOT NULL,
	content VARCHAR (1024), -- Will extend with ALTER 
	account_id INTEGER REFERENCES account(id) ON DELETE CASCADE,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
	updated _at TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
	PRIMARY KEY (id)
);
```

```postgresql
CREATE TABLE comment (
	id SERIAL, 
	content TEXT NOT NULL,
	account_id INTEGER REFERENCES account(id) ON DELETE CASCADE, 
	post_id INTEGER REFERENCES post (id) ON DELETE CASCADE,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
	PRIMARY KEY (id)
);

CREATE TABLE fav ( 
	id SERIAL, 
	oops TEXT, -- Will remove later with ALTER
	post_id INTEGER REFERENCES post(id) ON DELETE CASCADE,
	account_id INTEGER REFERENCES account(id) ON DELETE CASCADE, 
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	UNIQUE (post_id, account_id),
	PRIMARY KEY (id)
) ;
```

## Fixing by removing a column
```postgresql
ALTER TABLE fav DROP COLUMN oops;
```

## Altering column
```postgresql
ALTER TABLE post ALTER COLUMN content TYPE TEXT;
```
## Adding column
```postgresql
ALTER TABLE fav ADD COLUMN howmuch INTEGER;
```
## Deleting and restarting indices
```postgresql
DELETE FROM account;
ALTER SEQUENCE account_id_seq RESTART WITH 1;
ALTER SEQUENCE post_id_seq RESTART WITH 1;
ALTER SEQUENCE comment_id_seq RESTART WITH 1;
ALTER SEQUENCE fav_id_seq RESTART WITH 1;
```

> Here, account, post, comment, and fav are table that applies cascade delete; therefore, removing the account will wipe all the information from those tables.

