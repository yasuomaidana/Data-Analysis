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

## Interpolation

Cargo's `config.toml` supports interpolation of environment variables. You can use `${VAR}` or `$VAR` to embed environment variables within your configuration values.

### Example:

```toml
[env]
DATABASE_URL = "postgres://${DB_USER}:${DB_PASS}@localhost/mydb"
```

In this example, `DB_USER` and `DB_PASS` would be read from the environment and substituted into the `DATABASE_URL` string.

## Profile-Specific Configuration

You can change the configuration for different profiles, like `dev` and `release`, using conditional `cfg` attributes. For environment variables, this is done by targeting different configurations.

The `dev` profile is used by default (e.g., `cargo run`, `cargo build`) and has `debug` assertions enabled. The `release` profile (e.g., `cargo run --release`) is built with optimizations and has `debug` assertions disabled.

### Example: Dev vs. Release `DATABASE_URL`

You can set a different `DATABASE_URL` for development and release builds like this:

```toml
# Default for dev profile (debug assertions enabled)
[target.'cfg(debug)'.env]
DATABASE_URL = "postgres://devuser:devpass@localhost/devdb"

# For release profile (debug assertions disabled)
[target.'cfg(not(debug))'.env]
DATABASE_URL = "postgres://produser:prodpass@localhost/proddb"
```

When you run `cargo run`, it will use the `devdb` connection string. When you run `cargo run --release`, it will use the `proddb` connection string.
