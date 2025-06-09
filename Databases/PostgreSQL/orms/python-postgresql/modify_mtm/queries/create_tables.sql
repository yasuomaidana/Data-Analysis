DROP TABLE IF EXISTS artist CASCADE;
CREATE TABLE artist
(
    id   SERIAL,
    name VARCHAR(128) UNIQUE,
    PRIMARY KEY (id)
);

-- Drop the tracktoartist table if it exists, cascading to dependent objects
DROP TABLE IF EXISTS tracktoartist CASCADE;
-- Create the tracktoartist table to represent the many-to-many relationship between tracks and artists
CREATE TABLE tracktoartist
(
    id        SERIAL,                                           -- Unique identifier for the relationship
    track_id  INTEGER REFERENCES track (id) ON DELETE CASCADE,  -- Foreign key to track table
    artist_id INTEGER REFERENCES artist (id) ON DELETE CASCADE, -- Foreign key to artist table
    PRIMARY KEY (id)
);
-- These lines create foreign keys to the track and artist tables. 
-- The ON DELETE CASCADE clause means that if a referenced track or artist is deleted, 
-- all related rows in tracktoartist will also be automatically deleted. 
-- This helps maintain referential integrity by ensuring there are no orphaned relationships.