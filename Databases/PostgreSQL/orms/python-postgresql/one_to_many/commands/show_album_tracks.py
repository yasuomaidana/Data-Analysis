import click
from click import option
from sqlalchemy import Engine
from sqlmodel import create_engine, Session, select, text

from ..model import Track, Album


def raw_sql(limit: int, engine: Engine):
    limit = limit if limit else 3;
    statement = """SELECT track.title, album.title
                   FROM track
                            JOIN album ON track.album_id = album.id
                   ORDER BY track.title LIMIT :limit;"""
    statement = text(statement)
    with engine.connect() as conn:
        result = conn.execute(statement, {"limit": limit})
        for i, row in enumerate(result):
            click.echo(f"{i}. {row}")


def sqlmodel_read(limit: int, engine: Engine):
    with Session(engine) as session:
        statement = select(Track, Album).join(Album).order_by(Track.title)
        if limit:
            statement = statement.limit(limit)
        result = session.exec(statement)
        for i, (track, album) in enumerate(result):
            click.echo(f"{i}. ({track.title}, {album.title})")


@click.command("show-album-tracks")
@click.pass_context  # Decorate 'version_command' to receive context
@option('--sql', type=bool, default=True, is_flag=True)
@option('--limit', type=int, default=3)
def show_album_tracks(ctx: click.Context, sql: bool, limit: int) -> None:
    """Show album tracks"""
    engine = create_engine(ctx.obj['DATABASE_URL'], echo=True)
    if sql:
        raw_sql(limit, engine)
    else:
        sqlmodel_read(limit, engine)
