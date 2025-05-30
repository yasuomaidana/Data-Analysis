from sqlmodel import SQLModel, Field, Relationship


class HeroTeamLink(SQLModel, table=True):
    


class Album(SQLModel, table=True):
    __tablename__ = "album"

    # This 'id' column corresponds to 'album_id INTEGER' from your source data.
    # It's the primary key for the 'album' table. We assume it's an integer
    # provided with the source data that uniquely identifies an album.
    id: int = Field(primary_key=True, description="Unique identifier for the album (from source's album_id)")

    # This 'name' column corresponds to 'album TEXT' from your source data.
    name: str = Field(index=True, description="Name of the album (from source's album text)")

    # Relationship: Defines that an Album can have multiple Track records.
    # 'back_populates="album"' links this to the 'album: Optional["Album"]' field in the Track model.
    tracks: list["Track"] = Relationship(back_populates="albums")