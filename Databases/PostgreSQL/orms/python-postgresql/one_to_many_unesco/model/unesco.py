from sqlmodel import SQLModel, Field


class Unesco(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)

    name: str = Field(index=True, description="Name of the UNESCO site")
    description: str | None = Field(default=None, description="Description of the UNESCO site")
    year: int = Field(description="Year of the UNESCO site")

    category_id: int = Field(foreign_key="category.id")
    state_id: int = Field(foreign_key="state.id")
    region_id: int = Field(foreign_key="region.id")
    iso_id: int = Field(foreign_key="iso.id")


class Category(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    name: str = Field(index=True, description="Name of the category", unique=True)


class State(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    name: str = Field(index=True, unique=True, description="Name of the state")


class Region(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    name: str = Field(index=True, unique=True, description="Name of the region")


class Iso(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    name: str = Field(index=True, unique=True, description="Name of the ISO code")
