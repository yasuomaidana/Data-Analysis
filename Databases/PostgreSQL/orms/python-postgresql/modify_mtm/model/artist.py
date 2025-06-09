from sqlmodel import SQLModel, Field


class Artist(SQLModel, table=True):
    id: int = Field(default=None, primary_key=True)
    name: str = Field(max_length=128, unique=True)
