import csv
from .model import *


def model_caster(row: list[str]) -> tuple[Unesco, Iso, Region, State, Category]:
    unesco = Unesco(
        name=row[0],
        description=row[1] if row[1] else None,
        year=int(row[3])
    )
    iso = Iso(name=row[10])
    state = State(name=row[8])
    region = Region(name=row[9])
    category = Category(name=row[7])
    return unesco, iso, region, state, category


def unesco_reader(filename):
    with open(filename) as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=',')
        _header = next(csv_reader)
        for row in csv_reader:
            yield model_caster(row)
        

if __name__ == '__main__':
    for i in unesco_reader('whc-sites-2018-small.csv'):
        print(i)
        break
