## Simple join
```postgresql
SELECT make.name, model.name
    FROM model
    JOIN make ON model.make_id = make.id
    ORDER BY make.name LIMIT 5;
```
