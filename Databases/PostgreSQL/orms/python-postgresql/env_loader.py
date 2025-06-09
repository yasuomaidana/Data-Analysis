import os

def get_variables():
    """
    Retrieves environment variables for PostgreSQL connection.
    Returns a dictionary with the database connection parameters.
    """
    db_user = os.getenv("POSTGRES_USER")
    db_password = os.getenv("POSTGRES_PASSWORD")
    db_host = os.getenv("POSTGRES_HOST")
    db_port = os.getenv("POSTGRES_PORT", "5432")  # Default Postgresql port
    db_name = os.getenv("POSTGRES_DB")

    return {
        "db_user": db_user,
        "db_password": db_password,
        "db_host": db_host,
        "db_port": db_port,
        "db_name": db_name
    }

def get_postgresql_url(ctx):
    db_user = os.getenv("POSTGRES_USER")
    db_password = os.getenv("POSTGRES_PASSWORD")
    db_host = os.getenv("POSTGRES_HOST")
    db_port = os.getenv("POSTGRES_PORT", "5432")  # Default Postgresql port
    db_name = os.getenv("POSTGRES_DB")

    ctx.obj['DATABASE_URL'] = f"postgresql://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"