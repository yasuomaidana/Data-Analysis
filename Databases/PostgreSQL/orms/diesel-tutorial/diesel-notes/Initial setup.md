# Diesel initial setup

1. Set the `DATABASE_URL` environmental variable, you can place it in a `.env` file.
   > You can always run all diesel's command using the flag `--database-url <URL>` to override the .env file.
2. Run `diesel migration run` to apply pending migrations. This command applies the changes in the `up.sql` files.
3. To revert the last migration, run `diesel migration redo`. This will run the `down.sql` file for the last applied migration.

## Generating migrations and updating the schema

Diesel primarily follows a SQL-first approach for migrations. You write SQL to define your schema changes, and Diesel updates your Rust schema definition (`schema.rs`) from the database.

### From SQL to Schema (the standard way)

This is the recommended workflow for managing your database schema with Diesel.

1.  **Generate a new migration:**
    Create a new migration directory with `up.sql` and `down.sql` files.
    ```bash
    diesel migration generate <migration_name>
    ```
    Replace `<migration_name>` with a descriptive name for your migration (e.g., `create_posts`).

2.  **Write your migration:**
    -   Edit the generated `up.sql` file to include the SQL for your schema changes (e.g., `CREATE TABLE posts ...`).
    -   Edit the `down.sql` file to include the SQL to revert the changes (e.g., `DROP TABLE posts`).

3.  **Apply the migration and update schema.rs:**
    Run the migration to apply the changes to your database. Diesel will automatically reflect these changes in your `src/schema.rs` file.
    ```bash
    diesel migration run
    ```
    Your `schema.rs` is now up-to-date with your database schema. You should not edit this file manually.

### From Schema to SQL (Advanced)

While the standard workflow is SQL-first, Diesel provides a `--diff-schema` flag to generate a migration by comparing your current database schema against a temporary, modified schema definition. This is useful for generating migrations from changes made to your Rust structs.

**This approach should be used with caution**, as it bypasses the standard SQL-first workflow.

1.  **Define your desired schema in a temporary file:**
    Create a new Rust file (e.g., `src/schema/car_schema.rs`) and define the desired table structure using Diesel's `table!` macro. This file will not be part of your final build.

2.  **Generate the migration:**
    Use the `diesel migration generate` command with the `--diff-schema` flag, pointing to your temporary schema file.
    ```bash
    diesel migration generate modify_car --diff-schema=.\\src\\schema\\car_schema.rs
    ```
    Diesel will compare the schema in `car_schema.rs` with the schema of your actual database and generate the `up.sql` and `down.sql` files with the necessary changes.

3.  **Review and run the migration:**
    Always review the generated SQL files to ensure they are correct. Then, run the migration as usual:
    ```bash
    diesel migration run
    ```
    This will apply the changes and update your main `src/schema.rs` file.
