import click
from click import option
from sqlmodel import Session, select, create_engine

from ..model import Track


@click.command("show-tracks")
@click.pass_context  # Decorate 'version_command' to receive context
@option('--limit','-l', type=int)
def show_tracks(ctx: click.Context, limit: int):
    """Shows all/limit tracks."""
    click.echo("Showing tracks...")
    engine = create_engine(ctx.obj['DATABASE_URL'], echo=True)
    with Session(engine) as session:
        statement = select(Track).order_by(Track.title, Track.rating)
        if limit:
            statement = statement.limit(limit)
        tracks = session.exec(statement).all()
        for track in tracks:
            click.echo(f"{track.id}:{track.name} ({track.rating})")