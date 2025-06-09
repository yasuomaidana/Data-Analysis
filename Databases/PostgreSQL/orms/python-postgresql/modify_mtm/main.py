import os

import click
from dotenv import load_dotenv

from modify_mtm.commands.drop_duplicated_albums import drop_duplicated_albums
from modify_mtm.commands.get_table_info import get_table_info


@click.group(invoke_without_command=True)  # Allow main to run if no subcommand, and to set up context
@click.option('--env-file', '-e-f', default=".env",
              help='Environment file path.',
              type=click.Path())
@click.pass_context  # Ensure 'main' receives the context object
def main(ctx, env_file):  # Add 'ctx' as the first parameter
    """
    A CLI tool that modifies an SQL database.
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
if __name__ == "__main__":
    main()
