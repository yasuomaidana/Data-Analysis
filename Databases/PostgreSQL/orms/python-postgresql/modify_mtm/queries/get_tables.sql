SELECT t.table_schema,
       t.table_name,
       c.column_name,
       c.data_type
FROM information_schema.tables t
         JOIN
     information_schema.columns c
     ON t.table_schema = c.table_schema
         AND t.table_name = c.table_name
WHERE t.table_type = 'BASE TABLE' AND t.table_schema = 'public'
ORDER BY t.table_schema,
         t.table_name,
         c.ordinal_position;