import csv
from .model import Album, Track


def read_csv_file(filename):
    with open(filename) as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=',')
        for row in csv_reader:
            album = Album(title=row[2])
            track = Track(title=row[0],
                          len=row[3] if row[3] else None,
                          rating=row[4] if row[4] else None,
                          count=row[5] if row[5] else None)
            yield track, album


if __name__ == '__main__':
    print(list(read_csv_file('../library.csv')))
