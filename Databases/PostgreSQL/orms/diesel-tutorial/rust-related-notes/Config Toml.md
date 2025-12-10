# Cargo's Config Toml

Cargo has a feature that allows you to set environment variables for your project in a `.cargo/config.toml` file. This is useful for setting project-specific configurations without cluttering your shell's configuration files.

## Example: Setting DATABASE_URL

1.  Create a `.cargo` directory in the root of your project.
2.  Inside the `.cargo` directory, create a file named `config.toml`.
3.  Add the following content to `.cargo/config.toml`:

    ```toml
    [env]
    DATABASE_URL = "postgres://myuser:mypassword@localhost/mydb"
    ```

Now, when you run `cargo build` or `cargo run`, the `DATABASE_URL` environment variable will be set to the specified value.

This gives you the flexibility to use a local development database defined in `config.toml` by default, while easily swapping it out for production or testing databases when needed.
