import os

import click
from dotenv import load_dotenv

from modify_mtm.commands import *


@click.group(invoke_without_command=True)  # Allow main to run if no subcommand, and to set up context
@click.option('--env-file', '-e-f', default=".env",
              help='Environment file path.',
              type=click.Path())
@click.pass_context  # Ensure 'main' receives the context object
def main(ctx, env_file):  # Add 'ctx' as the first parameter
    """
    A CLI tool that modifies an SQL database.
    This CLI tool is the second part of the one-to-many tool.
    Actually, mtm stands for many-to-many.
    Here, I'll modify the database to handle many-to-many relationships.
    The tool will create the necessary tables and drop duplicated albums.
    It will also update the database with the new tables and relationships.
    """
    # Initialize ctx.obj if it's not already (important for groups)
    ctx.ensure_object(dict)

    env_path = os.path.abspath(env_file)
    if os.path.exists(env_path):
        load_dotenv(dotenv_path=env_path)
    else:
        click.echo(f"Warning: Environment file '{env_path}' not found. Using system environment variables if set.",
                   err=True)

    if ctx.invoked_subcommand is None:
        click.echo(ctx.get_help())

main.add_command(get_table_info)
main.add_command(drop_duplicated_albums)
main.add_command(create_tables)
main.add_command(update_info)
if __name__ == "__main__":
    main()
