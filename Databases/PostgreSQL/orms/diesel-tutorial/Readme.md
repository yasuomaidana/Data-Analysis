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

## Configuration

* [Cargo Config Toml](./rust-related-notes/Config%20Toml.md)

## Diesel related notes

1. [Initial Setup](./diesel-notes/Initial%20setup.md)
2. First Steps:
    1. [Check Connection](./check-connection/src/main.rs)
    2. [ORM Module](./orm-module/src/lib.rs)
    3. [Insert Cars](./insert-cars/src/main.rs)
    4. [Get Cars](./get-cars/src/main.rs)
    5. [Update Car](./update-car/src/main.rs)
    6. [One Off Update](./one-off-update/src/main.rs)
    7. [Delete Car](./delete-car/src/main.rs)

## Troubleshooting

If you have problems installing diesel and postgres in windows try running
> windows `choco install postgresql --params "'/Components:postgresql-client'" -y`