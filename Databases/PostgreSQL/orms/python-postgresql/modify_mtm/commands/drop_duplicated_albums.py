import os

import click
from sqlalchemy import CursorResult
from sqlmodel import Session, create_engine, text

from env_loader import get_postgresql_url


@click.command("drop-duplicated-albums")
@click.pass_context
def drop_duplicated_albums(ctx):
    """Drop duplicated albums from the database."""
    click.echo("Dropping duplicated albums...")

    get_postgresql_url(ctx)
    engine = ctx.obj['DATABASE_URL']
    with Session(create_engine(engine)) as session:
        with open(os.path.join(os.path.dirname(__file__), "..", "queries", "drop_duplicates.sql")) as f:
            sql = f.read()
        result:CursorResult = session.exec(text(sql))
        session.commit()
    click.echo(f"{result.rowcount} duplicated albums dropped.")
    click.echo("Duplicated albums dropped successfully.")