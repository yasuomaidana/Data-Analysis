import csv
# from collections import defaultdict

from .model import *


def model_caster(row: list[str]) -> tuple[Unesco, Iso, Region, State, Category]:
    unesco = Unesco(
        name=row[0],
        description=row[1] if row[1] else None,
        year=int(row[3])
    )
    iso = Iso(name=row[10]) if row[10] else None
    state = State(name=row[8])
    region = Region(name=row[9])
    category = Category(name=row[7])
    return unesco, iso, region, state, category


def unesco_reader(filename):
    with open(filename) as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=',')
        _headers = next(csv_reader)
        # print(headers)
        # columns = ["name", "description", "year", "category", "state", "region", "iso"]
        # print(columns)
        # 
        # headers = {header: i for i, header in enumerate(headers)}
        # nullable = defaultdict(int)
        for row in csv_reader:
        #     for col in columns:
        #         if not row[headers[col]]:
        #             print(row)
        #             print(row[headers[col]])
        #             nullable[col] += 1
        # print(nullable)        
            yield model_caster(row)


if __name__ == '__main__':
    # unesco_reader('whc-sites-2018-small.csv')
    for i in unesco_reader('whc-sites-2018-small.csv'):
        if i[1] is None:
            for j in i:
                print(j)
                print("-" * 10)
        else:
            for j in i:
                if j.name is None:
                    print(j)
                    print("*" * 10)
    #     print(i)
    #     break
