CREATE TABLE entities
(
    id       TEXT PRIMARY KEY,
    _type    TEXT NOT NULL,
    _class   TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::JSONB
);

CREATE INDEX entity_class ON entities (_class);
CREATE INDEX entity_type ON entities (_type);

CREATE TABLE relationships
(
    source_entity_id TEXT NOT NULL REFERENCES entities (id),
    target_entity_id TEXT NOT NULL REFERENCES entities (id),
    _class           TEXT,

    PRIMARY KEY (source_entity_id, target_entity_id),
    CHECK ( source_entity_id != target_entity_id )
);

CREATE INDEX relationship_class ON relationships (_class);


CREATE OR REPLACE FUNCTION has_cycle(input_source_node_id text, input_target_node_id text)
    RETURNS BOOLEAN
    LANGUAGE plpgsql AS
$$
DECLARE
    rec RECORD;
BEGIN
    FOR rec IN
        WITH RECURSIVE traversed AS (SELECT ARRAY [input_source_node_id] AS path,
                                            input_target_node_id         AS target_node_id
                                     UNION ALL
                                     SELECT traversed.path || relationships.source_entity_id,
                                            relationships.target_entity_id
                                     FROM traversed
                                              JOIN relationships
                                                   ON relationships.source_entity_id = traversed.target_node_id)
        SELECT *
        FROM traversed
        LOOP
            IF rec.target_node_id = ANY (rec.path) THEN
                RETURN TRUE; -- Early return, stop looking when first cycle is detected
            END IF;
        END LOOP;
    RETURN FALSE;
END;
$$;

ALTER TABLE relationships
    ADD CONSTRAINT check_no_cycles CHECK (NOT has_cycle(source_entity_id, target_entity_id));

