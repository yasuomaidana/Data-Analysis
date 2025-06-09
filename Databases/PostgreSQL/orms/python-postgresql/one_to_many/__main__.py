import os

import click

from dotenv import load_dotenv
from .commands import *
from env_loader import get_postgresql_url, get_variables


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

    get_postgresql_url(ctx)

    # This part runs if 'main' is invoked without a subcommand
    if ctx.invoked_subcommand is None:
        variables = get_variables()
        db_user = variables.get("db_user", "Not set")
        db_password = variables.get("db_password", None)
        db_host = variables.get("db_host", "localhost")
        db_port = variables.get("db_port", "5432")
        db_name = variables.get("db_name", "Not set")

        click.echo("--- Environment Configuration (No Subcommand) ---")
        click.echo(f"db_user: {db_user}")
        click.echo(f"db_password: {'********' if db_password else 'Not set'}")
        click.echo(f"db_host: {db_host}")
        click.echo(f"db_port: {db_port}")
        click.echo(f"db_name: {db_name}")
        click.echo("-------------------------------------------------")


main.add_command(show_tracks)
main.add_command(show_albums)
main.add_command(load_csv)
main.add_command(show_album_tracks)

if __name__ == "__main__":
    main()
