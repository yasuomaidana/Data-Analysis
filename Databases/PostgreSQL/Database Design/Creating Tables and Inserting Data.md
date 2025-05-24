## Creating database
You can do it by going to the server where you are running the PostgreSQL database and running `sudo -u postgres psql postgres`, then running the following SQL query:
```postgresql
CREATE DATABASE music WITH OWNER 'OWNER_NAME' ENCODING 'UTF8';
```
This command will create the database `music` with the owner `'OWNER_NAME'`.
## Creating the tables
```postgresql
CREATE TABLE artist(
	id SERIAL,
	name VARCHAR(128) UNIQUE,
	PRIMARY KEY(id)
);

CREATE TABLE album(
	id SERIAL,
	title VARCHAR(128) UNIQUE,
	artist_id INTEGER REFERENCES artist(id) ON DELETE CASCADE,
	PRIMARY KEY(id)
);
```
`ON DELETE CASCADE` means that if you delete an `artist`, it will also delete all `album`s where that artist appears.

```postgresql
CREATE TABLE genre(
	id SERIAL,
	name VARCHAR (128) UNIQUE, 
	PRIMARY KEY(id)
);

CREATE TABLE track (
	id SERIAL, 
	title VARCHAR(128), 
	len INTEGER, 
	rating INTEGER, 
	count INTEGER, 
	album_id INTEGER REFERENCES album(id) ON DELETE CASCADE,
	genre INTEGER REFERENCES genre(id) ON DELETE CASCADE,
	UNIQUE (title, album_id),
	PRIMARY KEY(id)
);
```
## Inserting data
Artist
```postgresql
INSERT INTO artist (name) VALUES ('Led Zeppelin');
```
Albums
```postgresql
INSERT INTO album(title, artist) VALUES ('Who Made Who',2);
```
Genre
```postgresql
INSERT INTO genre(name) VALUES ('Rock');
```
Track
```postgresql
INSERT INTO track(title, rating, len, count, album_id, genre_id) 
		VALUES ('Stairway', 5,   482,    0,     2,        1) ;
```
