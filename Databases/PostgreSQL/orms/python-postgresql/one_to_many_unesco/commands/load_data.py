import click
from click import option
from sqlmodel import create_engine, Session, select

from env_loader import get_postgresql_url
from ..model import *
from ..reader import unesco_reader

def validate_item(item, item_type, session:Session):
    """Validate and add item to the session if it doesn't exist."""
    if item is not None:
        get_statement = select(item_type).where(item_type.name == item.name)
        existing_item = session.exec(get_statement).first()
        if existing_item is None:
            session.add(item)
            session.flush()
            return item
    return existing_item if existing_item else item


@click.command("load-csv")
@click.pass_context  # Decorate 'version_command' to receive context
@option('--file', '-f', type=click.Path(exists=True), default="whc-sites-2018-small.csv")
def load_csv(ctx: click.Context, file):
    """Load albums and tracks from a csv file"""
    click.echo("Loading data...")
    
    get_postgresql_url(ctx)
    engine = create_engine(ctx.obj['DATABASE_URL'], echo=True)
    categories = {}
    isos = {}
    regions = {}
    states = {}
    # unesco = {}
    with Session(engine) as session:
        for (unesco, iso, region, state, category) in unesco_reader(file):
            if category.name not in categories:
                categories[category.name] = validate_item(category, Category, session)
            if iso is not None and iso.name not in isos:
                isos[iso.name] = validate_item(iso, Iso, session)
            if region.name not in regions:
                regions[region.name] = validate_item(region, Region, session)
            if state.name not in states:
                states[state.name] = validate_item(state, State, session)
            
            unesco.iso_id = isos[iso.name].id
            unesco.category_id = categories[category.name].id
            unesco.region_id = regions[region.name].id
            unesco.state_id = states[state.name].id
            if session.exec(select(Unesco).where(Unesco.name == unesco.name)).first() is None:
                session.add(unesco)
            session.commit()