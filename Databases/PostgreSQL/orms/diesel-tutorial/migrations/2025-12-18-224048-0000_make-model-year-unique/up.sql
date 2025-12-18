ALTER TABLE cars
    DROP CONSTRAINT IF EXISTS car_model_key;
ALTER TABLE cars
    ADD CONSTRAINT cars_model_year_unique UNIQUE (model, year);