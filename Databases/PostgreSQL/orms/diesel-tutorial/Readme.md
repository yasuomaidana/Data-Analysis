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

2. Initial tutorial:
   1. [Check Connection](./first-steps/src/check_connection.rs)
      Establishes a simple connection to the database to verify configuration. It ensures that the environment variables and database server are correctly set up.
   2. [ORM Module](./orm-module/src/lib.rs)
      Defines the shared logic, including schema definitions and models. It acts as the central library that other binaries depend on for database interactions.
   3. [Insert Cars](./first-steps/src/insert_cars.rs)
      Demonstrates how to create and insert new `Car` records into the database. It takes user input and persists the data using Diesel's insert API.
   4. [Get Cars](./first-steps/src/get_cars.rs)
      Retrieves `Car` records from the database with filtering and sorting. It showcases how to select data and map it to Rust structs.
   5. [Update Car](./first-steps/src/update_cars.rs)
      Updates a car by first loading it, modifying the struct, and saving changes. This method is useful when business logic needs to be applied to the current state.
      1. [One Off Update](./first-steps/src/one_off_update.rs)
         Directly updates records in the database using a query without loading them first. This is efficient for simple field updates or bulk operations.
   6. [Delete Car](./first-steps/src/delete_car.rs)
      Removes car records from the database based on user input. It handles both single record deletion and bulk deletion based on criteria.
## Troubleshooting

If you have problems installing diesel and postgres in windows try running
> windows `choco install postgresql --params "'/Components:postgresql-client'" -y`