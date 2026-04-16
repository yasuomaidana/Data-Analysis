-- B-tree index on the text value of JSON key "name" for equality/sorting queries
CREATE INDEX jtrack_btree ON jtrack USING btree ((body ->> 'name'));

-- GIN index for jsonb existence queries (`?`, `?|`, `?&`) and general jsonb ops.
-- Use `jsonb_ops` to support existence operators (covers more operators).
CREATE INDEX jtrack_gin ON jtrack USING gin (body jsonb_ops);

-- GIN index optimized for containment queries (`@>`).
-- `jsonb_path_ops` is smaller/faster but only supports containment.
CREATE INDEX jtrack_gin_path_ops ON jtrack USING gin (body jsonb_path_ops);