WITH RECURSIVE AccountRelationships AS (
         -- Anchor member (non-recursive):
         -- This is the initial seed of the recursion. It selects entities joined with relationships
         -- where the entity is the source (e.id = r.source_entity_id).
         SELECT
             *,
             r._class AS relationship_class
         FROM entities e
         JOIN relationships r ON e.id = r.source_entity_id

         -- Recursive member (place after the anchor, combined with UNION ALL):
         -- If you need recursive expansion, add a recursive SELECT here that references
         -- AccountRelationships. Example template:
         -- UNION ALL
         -- SELECT e2.*, r2._class AS relationship_class
         -- FROM entities e2
         -- JOIN relationships r2 ON e2.id = r2.source_entity_id
         -- JOIN AccountRelationships ar ON ar.target_entity_id = e2.id
     )
     -- Final result:
     -- Selects rows produced by the CTE and joins to the target entities to return the final output.
     SELECT ar.id,
            ar._type,
            ar.metadata,
            ar.relationship_class,
            e.*
     FROM AccountRelationships ar
     JOIN entities e ON ar.target_entity_id = e.id;