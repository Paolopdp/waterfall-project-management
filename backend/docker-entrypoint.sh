#!/bin/bash
set -e

# Attendi che il database sia pronto
echo "Waiting for postgres..."
until pg_isready -h postgres -p 5432 -U postgres; do
    sleep 1
done
echo "PostgreSQL started"

# Esegui le migrazioni
echo "Running migrations..."
sqlx migrate run

# Avvia l'applicazione
echo "Starting application..."
cargo run

exec "$@"
