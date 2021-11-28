#!/bin/bash
set -e
SCHEMA="${GRAPHILE_WORKER_SCHEMA:-graphile_worker}"

psql -v ON_ERROR_STOP=1 -v GRAPHILE_WORKER_SCHEMA="$SCHEMA"  --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
  create schema if not exists :GRAPHILE_WORKER_SCHEMA;
  create extension if not exists "uuid-ossp" with schema public;
  create extension if not exists "pgcrypto" with schema public;
  create table :GRAPHILE_WORKER_SCHEMA.migrations(
    id int primary key,
    ts timestamptz default now() not null
  );
EOSQL

cp -r migrations/ /tmp/migrations

for file in /tmp/migrations/*; do
  sed -i "s/:GRAPHILE_WORKER_SCHEMA/${SCHEMA}/" "${file}"
  psql -v ON_ERROR_STOP=1 -v GRAPHILE_WORKER_SCHEMA="$SCHEMA"  --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" -a -f "${file}"
done

rm -rf /tmp/migrations
