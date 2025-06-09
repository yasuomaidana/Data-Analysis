import os

import click
from sqlmodel import Session, create_engine, text

from env_loader import get_postgresql_url


@click.command("create-tables")
@click.pass_context
def create_tables(ctx):
    """Creates artist and tracktoartist table"""
    get_postgresql_url(ctx)
    engine = ctx.obj['DATABASE_URL']
    with Session(create_engine(engine)) as session:
        with open(os.path.join(os.path.dirname(__file__), "..", "queries", "create_tables.sql")) as f:
            sql = f.read()
        session.exec(text(sql))
        session.commit()
