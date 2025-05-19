# Use the official PostgreSQL image from Docker Hub
# You can specify a version like: postgres:16 or postgres:latest
FROM postgres:latest

# Set environment variables for the database
# Replace 'your_password' with a strong password
ENV POSTGRES_PASSWORD=123
# Optionally, you can also set a specific user and database name
# ENV POSTGRES_USER=myuser
# ENV POSTGRES_DB=mydb

# The PostgreSQL server listens on port 5432 by default
EXPOSE 5432

# (Optional) You can add custom initialization scripts
# These scripts will be executed when the container starts for the first time
# For example, to create an additional database or user
# COPY ./init-user-db.sh /docker-entrypoint-initdb.d/

# (Optional) Set the default command.
# This is usually handled by the base image, but can be explicit.
# CMD ["postgres"]