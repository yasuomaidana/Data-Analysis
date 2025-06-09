import click
from click import option
from sqlmodel import Session, select, create_engine
from ..model import Album

@click.command("show-albums")
@click.pass_context  # Decorate 'version_command' to receive context
@option('--limit','-l', type=int)
def show_albums(ctx: click.Context, limit: int):
    """Shows all/limit albums."""
    click.echo("Showing albums...")
    engine = create_engine(ctx.obj['DATABASE_URL'], echo=True)
    with Session(engine) as session:
        statement = select(Album).order_by(Album.title)
        if limit:
            statement = statement.limit(limit)
        albums = session.exec(statement).all()
        for album in albums:
            click.echo(f"id: {album.id} title: {album.title}")