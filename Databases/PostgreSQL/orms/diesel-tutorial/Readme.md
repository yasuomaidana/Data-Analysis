# README

Here, I'll have instructions for using PostgreSQL using diesel and rust.

I'll cover from the docker configuration, migrations, and raw queries calling

## Docker Configuration

1. Create a docker file for PostgreSQL:
    ```Dockerfile
   FROM postgres:latest
   ENV POSTGRES_DB=mydb
   EXPOSE 5432
    ```
2. Create the container using the command:
    ```bash
   docker build -t my_postgres_image .
   docker run --name my_postgres_container -e POSTGRES_USER=myuser -e POSTGRES_PASSWORD=mypassword -p 5432:5432 -d my_postgres_image
    ```


## Installing 

> windows `choco install postgresql --params "'/Components:postgresql-client'" -y`