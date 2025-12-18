-- This file should undo anything in `up.sql`
ALTER TABLE cars
    DROP CONSTRAINT IF EXISTS cars_model_year_unique;
ALTER TABLE cars
    ADD CONSTRAINT car_model_key UNIQUE (model);