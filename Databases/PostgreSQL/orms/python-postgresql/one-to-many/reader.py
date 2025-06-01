import csv
from model import Album, Track


def read_csv_file(filename):
    with open(filename) as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=',')
        for row in csv_reader:
            album = Album(name=row[2])
            track = Track(title=row[0],
                          len=row[3],
                          rating=row[4],
                          count=row[5])
            yield track, album


if __name__ == '__main__':
    print(list(read_csv_file('../library.csv')))
