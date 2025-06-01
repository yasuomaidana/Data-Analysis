import os

import click

from dotenv import load_dotenv
from show_tracks import show_tracks


@click.group(invoke_without_command=True)  # Allow main to run if no subcommand, and to set up context
@click.option('--env-file', '-e-f', default=".env",
              help='Environment file path.',
              type=click.Path())
@click.pass_context  # Ensure 'main' receives the context object
def main(ctx, env_file):  # Add 'ctx' as the first parameter
    """
    A CLI tool that loads environment variables and can display its version.
    The environment variables are loaded when the CLI is invoked.
    If no subcommand is given, it will print the loaded environment.
    """
    # Initialize ctx.obj if it's not already (important for groups)
    ctx.ensure_object(dict)

    env_path = os.path.abspath(env_file)
    if os.path.exists(env_path):
        load_dotenv(dotenv_path=env_path)
    else:
        click.echo(f"Warning: Environment file '{env_path}' not found. Using system environment variables if set.",
                   err=True)

    db_user = os.getenv("POSTGRES_USER")
    db_password = os.getenv("POSTGRES_PASSWORD")
    db_host = os.getenv("POSTGRES_HOST")
    db_port = os.getenv("POSTGRES_PORT", "5432")  # Default Postgresql port
    db_name = os.getenv("POSTGRES_DB")

    ctx.obj['DATABASE_URL'] = f"postgresql://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"

    # This part runs if 'main' is invoked without a subcommand
    if ctx.invoked_subcommand is None:
        click.echo("--- Environment Configuration (No Subcommand) ---")
        click.echo(f"db_user: {db_user}")
        click.echo(f"db_password: {'********' if db_password else 'Not set'}")
        click.echo(f"db_host: {db_host}")
        click.echo(f"db_port: {db_port}")
        click.echo(f"db_name: {db_name}")
        click.echo("-------------------------------------------------")


main.add_command(show_tracks)

if __name__ == "__main__":
    main()
