import click
from click import option
from sqlmodel import create_engine, Session, select

from ..model import Album, Track
from ..reader import read_csv_file


@click.command("load-csv")
@click.pass_context  # Decorate 'version_command' to receive context
@option('--file', '-f', type=click.Path(exists=True), default="library.csv")
def load_csv(ctx: click.Context, file):
    """Load albums and tracks from a csv file"""
    click.echo("Loading data...")
    engine = create_engine(ctx.obj['DATABASE_URL'], echo=True)
    with Session(engine) as session:
        album_cache = {}
        for track, album in read_csv_file(file):
            if album.title not in album_cache:
                find_album_statement = select(Album).where(Album.title == album.title)
                stored_album = session.exec(find_album_statement).first()
                if stored_album is None:
                    session.add(album)
                    session.flush()
                album_cache[album.title] = album
            else:
                album = album_cache[album.title]
            track.album_id = album.id
            find_track_statement = select(Track).where(Track.title == track.title, Track.album_id == album.id)
            if session.exec(find_track_statement).first() is None:
                session.add(track)
                session.flush()
            session.commit()
    click.echo("Finished")
