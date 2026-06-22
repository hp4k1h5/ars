#!/bin/sh
set -eu

echo "Running database migrations..."
diesel migration run --migration-dir /migrations

echo "Starting server..."
exec /app/target/release/server