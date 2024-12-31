# Use the official PostgreSQL image as a base image
FROM postgres:latest

# Set environment variables (optional, as they are defined in docker-compose)
ENV POSTGRES_USER=youruser
ENV POSTGRES_PASSWORD=yourpassword
ENV POSTGRES_DB=local-postgres

# (Optional) Add custom initialization scripts
# COPY init.sql /docker-entrypoint-initdb.d/

# Expose the default PostgreSQL port
EXPOSE 5432
