ALTER TABLE IF EXISTS relationships
    DROP CONSTRAINT IF EXISTS check_no_cycles;
ALTER TABLE IF EXISTS relationships
    DROP CONSTRAINT IF EXISTS relationship_source_entity_id_fkey;
ALTER TABLE IF EXISTS relationships
    DROP CONSTRAINT IF EXISTS relationship_target_entity_id_fkey;
DROP INDEX IF EXISTS relationship_class;
DROP TABLE IF EXISTS relationships;
DROP INDEX IF EXISTS entity_type;
DROP INDEX IF EXISTS entity_class;
DROP TABLE IF EXISTS entities;
DROP FUNCTION IF EXISTS has_cycle(text, text);