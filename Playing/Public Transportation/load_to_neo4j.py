import pandas as pd
from neo4j import GraphDatabase
from tqdm import tqdm

## Set up Neo4j connection
uri = "bolt://localhost:7687"
username = "neo4j"
password = "test"
driver = GraphDatabase.driver(uri, auth=(username, password))

# Functions to create nodes and relationships
def create_city(tx, name, coords):
    tx.run("MERGE (c:City {name: $name, coords: $coords})", name=name, coords=coords)

def create_line(tx, name):
    tx.run("MERGE (l:Line {name: $name})", name=name)

def create_station(tx, name, geometry):
    tx.run("MERGE (s:Station {name: $name, geometry: $geometry})", name=name, geometry=geometry)

def link_station_to_city(tx, station_name, city_name):
    tx.run("MATCH (s:Station {name: $station_name}), (c:City {name: $city_name}) MERGE (s)-[:IN_CITY]->(c)", station_name=station_name, city_name=city_name)

def link_station_to_line(tx, station_name, line_name):
    tx.run("MATCH (s:Station {name: $station_name}), (l:Line {name: $line_name}) MERGE (s)-[:ON_LINE]->(l)", station_name=station_name, line_name=line_name)

# Load data from CSV
cities_df = pd.read_csv('./Data/cities.csv')
cities_df.dropna(inplace=True)
lines_df = pd.read_csv('./Data/lines.csv')
lines_df.dropna(inplace=True)
stations_df = pd.read_csv('./Data/stations.csv')
stations_df.dropna(inplace=True)
station_lines_df = pd.read_csv('./Data/station_lines.csv')
station_lines_df.dropna(inplace=True)

# Execute transactions
with driver.session() as session:
    # Create cities
    cities_df.apply(lambda row: session.execute_write(create_city, row['name'], row['coords']), axis=1)

    # Create lines
    lines_df.apply(lambda row: session.execute_write(create_line, row['name']), axis=1)

    # Create stations and link to cities
    for index, station in tqdm(stations_df.iterrows()):
        session.execute_write(create_station, station['name'], station['geometry'])
        # Assuming you can match city by name or another method
        session.execute_write(link_station_to_city, station['name'], "YourCityName")  # Replace with actual city name or ID logic

    # Link stations to lines (assuming line_id matches with line name somehow or using another mapping)
    for index, sl in tqdm(station_lines_df.iterrows()):
        station_name = stations_df.loc[stations_df['id'] == sl['station_id'], 'name'].values[0]
        line_name = lines_df.loc[lines_df['id'] == sl['line_id'], 'name'].values[0]
        session.execute_write(link_station_to_line, station_name, line_name)

driver.close()