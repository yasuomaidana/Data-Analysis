Since we will need a PostgreSQL server, we will use Docker to run it. You can create your own PostgreSQL server, but I will build mine using the `starting_postgresql.dockerfile` 

To create the image using my file, run
1. Run `docker build -t architechture_sql_image . -f starting_posgresql.dockerfile` the `.` tells you where the Docker image is located and -f specifies which Dockerfile to use.
2. `docker run --name my-postgres-server -d -p 5432:5432 architechture_sql_image`

> `-d` means run in  detach command
> `-p` means port mapping HOST: CONTAINER

## Starting PostgreSQL Command Line
1. Go to Docker Desktop, go to `my-postgres-server`'s console
2. Run `psql -U postgres`

### List databases

Run `\l` as a super user to list all **databases**; you should see 'postgres=#' in the console to confirm you are in super user mode.

### Creating a User and Database

1. Create user: `CREATE USER pg4e WITH PASSWORD 'secret';`
2. Create database: `CREATE DATABASE people WITH OWNER 'pg4e';`

> To close the session, you can run `\q` while inside the `psql`

## Logging in using a database and a user

Run `psql DATABASE USERNAME`
> `\dt`list tables in the current database

## Create table

```postgresql
CREATE TABLE users(name VARCHAR(128), email VARCHAR(128));
```
