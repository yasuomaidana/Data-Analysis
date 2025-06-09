-- removes all rows whose id is not the minimum for their title, 
-- effectively dropping duplicates based on the title column.
DELETE
FROM track
WHERE id NOT IN (
    -- finds the minimum id for each unique title.
    SELECT MIN(id)
    FROM track
    GROUP BY title);