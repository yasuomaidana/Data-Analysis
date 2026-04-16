-- This file should undo anything in `up.sql`
ALTER TABLE IF EXISTS "cars"
    RENAME TO "car";