import pandas as pd
from neo4j import GraphDatabase
from tqdm import tqdm

# Set up Neo4j connection
uri = "bolt://localhost:7687"
username = "neo4j"
password = "test"
driver = GraphDatabase.driver(uri, auth=(username, password))

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
cities_df = pd.read_csv('Data/cities.csv').dropna(subset=['name', 'coords'])
lines_df = pd.read_csv('Data/lines.csv').dropna(subset=['name'])
stations_df = pd.read_csv('Data/stations.csv').dropna(subset=['name', 'geometry'])
station_lines_df = pd.read_csv('Data/station_lines.csv')  # Assuming no specific columns to check for nulls here, adjust if necessary

with driver.session() as session:


    # Link stations to cities - this part needs city name logic
    # For demonstration, assuming 'city_id' in stations_df relates to 'id' in cities_df
    for station in tqdm(stations_df.itertuples(), total=len(stations_df)):
        city = cities_df[cities_df['id'] == station.city_id]  # Match by ID or name as appropriate
        if not city.empty:
            city_name = city['name'].iloc[0]
            session.execute_write(link_station_to_city, station.name, city_name)

    # Link stations to lines
    for sl in tqdm(station_lines_df.itertuples(), total=len(station_lines_df)):
        station_name = stations_df.loc[stations_df['id'] == sl.station_id, 'name']
        if not station_name.empty:
            station_name = station_name.values[0]
            line_name = lines_df.loc[lines_df['id'] == sl.line_id, 'name'].values[0]
            session.execute_write(link_station_to_line, station_name, line_name)

driver.close()
print("Data import completed.")