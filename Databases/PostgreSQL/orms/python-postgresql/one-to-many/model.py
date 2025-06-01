from sqlmodel import SQLModel, Field, Relationship


class Album(SQLModel, table=True):
    __tablename__ = "album"

    # This 'id' column corresponds to 'album_id INTEGER' from your source data.
    # It's the primary key for the 'album' table. We assume it's an integer
    # provided with the source data that uniquely identifies an album.
    id: int = Field(primary_key=True, description="Unique identifier for the album (from source's album_id)")

    # This 'name' column corresponds to 'album TEXT' from your source data.
    name: str = Field(index=True, description="Name of the album (from source's album text)")

class Track(SQLModel, table=True):
    __tablename__ = "track"
    
    id: int = Field(primary_key=True, description="Unique identifier for the track (from source's track_id)")
    title: str = Field(index=True, description="Title of the track (from source's title)")
    len: int = Field(index=True, description="Length of the track (from source's length)")
    rating: int = Field(index=True, description="Rating of the track (from source's rating)")
    count: int = Field(index=True, description="Number of tracks in the album (from source's count)")
    
    album_id: int = Field(index=True, description="Unique identifier for the album (from source's album_id)")
    