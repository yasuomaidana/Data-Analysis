import csv

def read_csv_file(filename):
    with open(filename) as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=',')
        for row in csv_reader:
            yield row
            
if __name__ == '__main__':
    print(list(read_csv_file('../library.csv')))
        
