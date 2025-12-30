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
2. Initial tutorial:
    1. [Check Connection](./first-steps/src/check_connection.rs)
       Establishes a simple connection to the database to verify configuration. It ensures that the environment
       variables and database server are correctly set up.
    2. [ORM Module](./orm-module/src/lib.rs)
       Defines the shared logic, including schema definitions and models. It acts as the central library that other
       binaries depend on for database interactions.
    3. [Insert Cars](./first-steps/src/insert_cars.rs)
       Demonstrates how to create and insert new `Car` records into the database. It takes user input and persists the
       data using Diesel's insert API.
    4. [Get Cars](./first-steps/src/get_cars.rs)
       Retrieves `Car` records from the database with filtering and sorting. It showcases how to select data and map it
       to Rust structs.
    5. [Update Car](./first-steps/src/update_cars.rs)
       Updates a car by first loading it, modifying the struct, and saving changes. This method is useful when business
       logic needs to be applied to the current state.
        1. [One Off Update](./first-steps/src/one_off_update.rs)
           Directly updates records in the database using a query without loading them first. This is efficient for
           simple field updates or bulk operations.
    6. [Delete Car](./first-steps/src/delete_car.rs)
       Removes car records from the database based on user input. It handles both single record deletion and bulk
       deletion based on criteria.
3. All About Select:
    1. [Fill Users](./all-about-select/src/fill_users.rs)
       Populates the database with random user data for testing purposes. It uses the `rand` crate to generate diverse
       user profiles and inserts them in bulk.
    2. [Run Query DSL](./all-about-select/src/run_query_dsl.rs)
       Demonstrates basic query execution using Diesel's `RunQueryDsl`. It loads all users into a tuple matching the
       table structure.
    3. [Via Query DSL](./all-about-select/src/via_query_dsl.rs)
       Showcases how to construct queries using the `QueryDsl` trait. It selects specific columns, filters by role, and
       orders the results.
    4. [Has Query](./all-about-select/src/has_query.rs)
       Illustrates how to use the `HasQuery` trait to build reusable query components. It defines a custom query for
       fetching user emails based on specific criteria.
    5. [Nested Tuples](./all-about-select/src/nested_tuples.rs)
       Explains how to select data into nested tuples for more complex data structures. This is useful when grouping
       related fields together in the result.
    6. [Without From](./all-about-select/src/without_from.rs)
       Shows how to execute SQL functions directly without selecting from a table. In this example, it retrieves the
       PostgreSQL version.
4. All About Update:
    1. [Update User](./all-about-select/src/update_user.rs)
       Updates a user record using the `UpdateUser` struct defined in [User Model](./orm-module/src/model/user.rs). This
       struct implements `AsChangeset`, allowing for partial updates where only the specified fields (name, email, or
       birthdate) are modified. It also demonstrates how to return the updated record using `.returning()`.

## Troubleshooting

If you have problems installing diesel and postgres in windows try running
> windows `choco install postgresql --params "'/Components:postgresql-client'" -y`