WITH RECURSIVE "account_relationships" ("id", "_type", "metadata", "relationship_class", "target_entity_id")
                   AS (SELECT "entities"."id",
                              "entities"."_type",
                              "entities"."metadata",
                              "relationships"."_class",
                              "relationships"."target_entity_id"
                       FROM ("entities" INNER JOIN "relationships"
                             ON ("entities"."id" = "relationships"."source_entity_id"))
                       WHERE (("entities"."_class" = 'Account') AND ("entities"."id" = 'account_1'))
                       UNION ALL
                       SELECT "entities"."id",
                              "entities"."_type",
                              "entities"."metadata",
                              "relationships"."_class",
                              "relationships"."target_entity_id"
                       FROM (("entities" INNER JOIN "relationships"
                              ON ("entities"."id" = "relationships"."source_entity_id")) INNER JOIN "account_relationships"
                             ON ("account_relationships"."target_entity_id" = "entities"."id")))
SELECT DISTINCT "account_relationships"."id",
                "account_relationships"."_type",
                "account_relationships"."metadata",
                "account_relationships"."relationship_class",
                "entities"."id",
                "entities"."_type",
                "entities"."metadata"
FROM ("account_relationships" INNER JOIN "entities" ON ("account_relationships"."target_entity_id" = "entities"."id"))
WHERE ("entities"."_class" = 'CodeRepo') -- binds: ["Account", "account_1", "CodeRepo"]