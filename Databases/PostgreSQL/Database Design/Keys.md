## Three Kinds of Keys
- **Primary key** - generally an integer auto-increment field
- **Logical key** - what the outside world uses for lookup
- **Foreign key** - generally an integer key pointing to a row in another table
## Primary Key Rules
Best practices:
- Never use your logical key as the primary key.
- Logical keys can and do change, albeit slowly.
- Relationships that are based on matching string fields are less efficient than relationships that are based on integers.
## Foreign Keys
A foreign key is a **column** in a table that contains a key **that references** the primary key of **another table**.
When all primary keys are integers, then all foreign keys are integers. This is good - very good.