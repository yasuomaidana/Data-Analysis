import csv
import os

import click
from sqlmodel import Session, create_engine, select

from env_loader import get_postgresql_url
from ..model import Album, Track, Artist, TrackToArtist


def read_csv_file(filename):
    with open(filename) as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=',')
        for row in csv_reader:
            album_ = Album(title=row[2])
            track_ = Track(title=row[0],
                          len=row[3] if row[3] else None,
                          rating=row[4] if row[4] else None,
                          count=row[5] if row[5] else None)
            artist_ = Artist(name=row[1])
            yield track_, album_, artist_


def get_or_create_artist(stored_artists: dict[str, Artist], to_search: Artist, session: Session) -> Artist:
    """Get or create an artist in the database."""
    if to_search.name not in stored_artists:
        find_artist_statement = select(Artist).where(Artist.name == to_search.name)
        find_artist_statement = session.exec(find_artist_statement).first()
        if find_artist_statement is not None:
            to_search = find_artist_statement
        else:
            session.add(to_search)
            session.flush()
        stored_artists[to_search.name] = to_search
    return stored_artists[to_search.name]


def get_or_create_album(stored_albums: dict[str, Album], to_search: Album, session: Session) -> Album:
    """Get or create an album in the database."""
    if to_search.title not in stored_albums:
        find_album_statement = select(Album).where(Album.title == to_search.title)
        find_album_statement = session.exec(find_album_statement).first()
        if find_album_statement is not None:
            to_search = find_album_statement
        else:
            session.add(to_search)
            session.flush()
        stored_albums[to_search.title] = to_search
    return stored_albums[to_search.title]
        

@click.command("update-info")
@click.pass_context
@click.option('--file', '-f', type=click.Path(exists=True), default="library.csv",
              help="Path to the CSV file containing track, album, and artist information.")
def update_info(ctx, file):
    """Update track, album, and artist information from a CSV file."""
    click.echo("Updating information...")
    get_postgresql_url(ctx)
    engine = ctx.obj['DATABASE_URL']
    engine = create_engine(engine)
    albums = {}
    artists = {}
    with Session(engine) as session:
        for track, album, artist in read_csv_file(file):
            find_track_statement = select(Track).where(Track.title == track.title)
            track_:Track = session.exec(find_track_statement).first()
            # if track is not None:
            #     click.echo(f"Track '{track.title}' already exists in the database.")
                
            # album_ = get_or_create_album(albums, album, session)
            artist_ = get_or_create_artist(artists, artist, session)
            find_track_artist_statement = select(TrackToArtist).where(TrackToArtist.track_id == track_.id, 
                                                                  TrackToArtist.artist_id == artist_.id)
            track_artist: TrackToArtist = session.exec(find_track_artist_statement).first()
            if track_artist is None:
                track_artist = TrackToArtist(track_id=track_.id, artist_id=artist_.id)
                session.add(track_artist)
                session.flush()
            else:
                click.echo(f"Track '{track.title}' already has artist '{artist.name}' associated.")

        session.commit()

        click.echo("Finished updating information.")
    print(len(albums))
    print(len(artists))
if __name__ == "__main__":
    fn = "../../library.csv"
    if os.path.exists(fn):
        for track, album, artist in read_csv_file(fn):
            print(f"Track: {track.title}, Album: {album.title}, Artist: {artist.name}")
            break
