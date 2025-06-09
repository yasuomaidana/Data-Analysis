import click
from sqlmodel import create_engine

from env_loader import get_postgresql_url
from ..model import *


@click.command("create-tables")
@click.pass_context  # Decorate 'version_command' to receive context
def create_tables(ctx: click.Context):
    """Create tables in the database."""

    get_postgresql_url(ctx)
    engine = create_engine(ctx.obj['DATABASE_URL'], echo=True)

    tables = [Unesco, Category, Region, Iso, State]
    tables = [i.__table__ for i in tables]
    
    click.echo("Dropping existing tables...")
    SQLModel.metadata.drop_all(engine, tables=tables, checkfirst=True)
    click.echo("Creating new tables...")
    SQLModel.metadata.create_all(engine, tables=tables)
    click.echo("Tables created successfully.")
