WITH RECURSIVE AccountRelationships AS (SELECT e.*,
                                               r._class AS relationship_class,
                                               r.target_entity_id
                                        FROM entities e
                                                 JOIN relationships r ON e.id = r.source_entity_id
                                        WHERE e._class = 'Account'
                                          AND e.id = 'account_1')

SELECT ar.id,
       ar._type,
       ar.metadata,
       ar.relationship_class,
       e.*
FROM AccountRelationships ar
         JOIN
     entities e ON ar.target_entity_id = e.id
WHERE e._class = 'CodeRepo'