#!/usr/bin/env bash
set -x
set -eo pipefail
# Defina o diretório onde os dados do PostgreSQL serão armazenados
PGDATA_DIR="/home/raoni/postgres-db-zero2prod/data"
# Defina o usuário do banco de dados, padrão é 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Defina a senha do banco de dados, padrão é 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Defina o nome do banco de dados, padrão é 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Defina a porta do banco de dados, padrão é '5432'
DB_PORT="${POSTGRES_PORT:=5432}"
# Inicie o PostgreSQL usando Docker
docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_DB=${DB_NAME} \
-p "${DB_PORT}":5432 \
-v "${PGDATA_DIR}":/var/lib/postgresql/data \
-d postgresq
# ^ Aumenta o número máximo de conexões para fins de teste

