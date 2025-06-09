from sqlmodel import SQLModel, Field, Relationship
from typing import Optional

class TrackToArtist(SQLModel, table=True):
    __tablename__ = "tracktoartist"

    id: Optional[int] = Field(default=None, primary_key=True)
    track_id: Optional[int] = Field(default=None, foreign_key="track.id")
    artist_id: Optional[int] = Field(default=None, foreign_key="artist.id")