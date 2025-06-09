from sqlmodel import SQLModel, Field


class Album(SQLModel, table=True):
    __tablename__ = "album"

    # This 'id' column corresponds to 'album_id INTEGER' from your source data.
    # It's the primary key for the 'album' table. We assume it's an integer
    # provided with the source data that uniquely identifies an album.
    id: int = Field(primary_key=True, description="Unique identifier for the album (from source's album_id)")

    # This 'name' column corresponds to 'album TEXT' from your source data.
    title: str = Field(index=True, description="Title of the album (from source's album text)")
