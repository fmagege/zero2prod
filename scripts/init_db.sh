#!/usr/bin/env bash
# This script is based on the following tutorial:
# Zero To Production In Rust
# Luca Palmieri
# https://itunes.apple.com/WebObjects/MZStore.woa/wa/viewBook?id=0

# Check if sqlx is installed
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    exit 1
fi

# Check if psql is installed
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

# set -x enables a mode of the shell where all executed commands are printed to the terminal
set -x

# set -e instructs bash to immediately exit if any command [1] has a non-zero exit status
# set -o pipefail sets the exit status of a pipeline to that of the rightmost command to exit with a non-zero status,
set -eo pipefail

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL

# if we do not have a running postgres instance then start one
if ! podman ps -a | grep -q postgres; 
then
    podman run \
        -d \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        --name postgres \
        postgres -N 1000
        # ^ Increased maximum number of connections for testing purposes”

    # Keep pinging Postgres until it's ready to accept commands
    # export PGPASSWORD="${DB_PASSWORD}"
    # until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    #     >&2 echo "Postgres is still unavailable - sleeping"
    #     sleep 1
    # done

    # Keep pinging postgres until it's ready to accept commands
    until podman exec $(podman ps -q) pg_isready --timeout=0 --dbname=${DB_NAME} > /dev/null 2>&1; do
        echo "Waiting for postgres server to be up and running"
        sleep 1
    done

    >&2 echo "Postgres is up and running on port ${DB_PORT}!"

    # Create the database
    >&2 echo "Creating database ${DB_NAME}!"
    sqlx database create

else
    >&2 echo "Postgres is already running on port ${DB_PORT}!"
fi

# Run the migrations
>&2 echo "Running migrations for ${DB_NAME}!"
sqlx migrate run

>&2 echo "Database ${DB_NAME} has been migrated and is ready to use!"

# unset -x disables the mode of the shell where all executed commands are printed to the terminal
set +x



