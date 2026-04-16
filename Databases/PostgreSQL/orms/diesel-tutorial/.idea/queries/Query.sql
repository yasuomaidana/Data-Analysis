UPDATE cars
SET model = REPLACE(REPLACE(TRIM(model), CHR(13), ''), CHR(10), '')
WHERE model <> REPLACE(REPLACE(TRIM(model), CHR(13), ''), CHR(10), '');


SELECT * FROM cars;