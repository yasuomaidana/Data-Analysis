import os
from collections import defaultdict

from sqlalchemy import text
from sqlmodel import Session, create_engine

from env_loader import get_postgresql_url
import click


@click.command("get-table-info")
@click.pass_context
@click.option('--tables', '-t', multiple=True, type=str,
              help="Name(s) of the table(s) to get information about. Can be used multiple times.")
def get_table_info(ctx, tables):
    """Get information about the database tables."""
    get_postgresql_url(ctx)
    engine = ctx.obj['DATABASE_URL']
    with Session(create_engine(engine)) as session:
        with open(os.path.join(os.path.dirname(__file__), "..", "queries", "get_tables.sql")) as f:
            sql = f.read()
        result = session.exec(text(sql))
    table_info = defaultdict(list)
    for (_, table_name, *columns) in result:
        table_info[table_name].append(columns)
    if tables:
        table_info = {key: value for key, value in table_info.items() if key in tables}
        if not table_info:
            click.echo("No matching tables found.")
            return
        missing_tables = set(tables) - set(table_info.keys())
        if missing_tables:
            click.echo(f"Warning: The following tables were not found: {', '.join(missing_tables)}")
            click.echo("-" * 40)
    for table_name, columns in table_info.items():
        click.echo(f"Table: {table_name}")
        if columns:
            for column_name, column_type in columns:
                click.echo(f"  {column_name}: {column_type}")
        else:
            click.echo("No columns found.")
        click.echo("-" * 40)
