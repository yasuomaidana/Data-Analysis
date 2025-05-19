We can use Rust and PostgreSQL using an ORM. I chose Diesel, but you can choose whatever you want.

## Preparing 

1. Install `libpq`: `brew install libpq`. Sometimes you'll need to run also `brew link --force libpq`
2. Install the diesel cli `cargo install diesel_cli --no-default-features --features postgres`
> You can read the [documentation](https://diesel.rs/guides/getting-started)

