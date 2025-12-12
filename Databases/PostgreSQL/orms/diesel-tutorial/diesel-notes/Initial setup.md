# Diesel initial setup

1. Set the `DATABASE_URL` environmental variable, you can place it using the .env file
    > You can always run all diesel's command using the flag `--database-url <URL>` variable to change the behaviour
2. Run `diesel migration run`, this command applies the changes in the `up.sql` and `diesel migration redo` applies `down.sql` ones.
3. 