from sqlmodel import SQLModel, Field


class Track(SQLModel, table=True):
    __tablename__ = "track"

    id: int = Field(primary_key=True, description="Unique identifier for the track (from source's track_id)")
    title: str = Field(index=True, description="Title of the track (from source's title)")
    len: int | None = Field(index=True, description="Length of the track (from source's length)")
    rating: int | None = Field(index=True, description="Rating of the track (from source's rating)")
    count: int | None = Field(index=True, description="Number of tracks in the album (from source's count)")

    album_id: int = Field(index=True, description="Unique identifier for the album (from source's album_id)",
                          foreign_key="album.id")
