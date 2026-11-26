#!/usr/bin/env bash
# Prepara o PostgreSQL 17 local do zero e roda o alvo `sistema_pg`.
#
# Equivale à matriz `pg / schema` de `.github/workflows/ci-db.yml`: papéis
# `nologin` (wyd-ops/scripts/ci/db-prepare.sh) e a série `-- migrate:up`
# (wyd-ops/scripts/ci/apply-series.sh). O container é recriado antes de cada execução —
# `tmpfs`, então o banco nasce vazio e repetir a task não colide com a série já
# aplicada.
#
# O host não tem `psql`: o `apply-series.sh` roda dentro do container, com as
# migrations copiadas para `/tmp`; o teste fala por `docker exec`, como na
# config de launch `Debug: teste wyd-db sistema_pg`.
#
# Uso: wyd-ops/scripts/ci/db-schema-local.sh   (de qualquer cwd)
set -euo pipefail

# Git Bash converte `/tmp/...` em caminho Windows antes de chamar o `docker`;
# sem isto o `docker cp` procura a pasta errada dentro do container.
export MSYS_NO_PATHCONV=1

RAIZ="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$RAIZ"

CONTAINER="wyd-db-1"
COMPOSE=(docker compose --profile db)

# Banco vazio de verdade: remove o container (o tmpfs leva os dados) e sobe de novo.
"${COMPOSE[@]}" rm -sf db >/dev/null 2>&1 || true
"${COMPOSE[@]}" up -d --wait db

# Papéis de `db-prepare.sh` no modo da matriz `schema`.
docker exec -i \
  -e PGHOST=127.0.0.1 -e PGUSER=postgres -e PGPASSWORD=test -e PGDATABASE=wyd \
  "$CONTAINER" bash -s nologin < wyd-ops/scripts/ci/db-prepare.sh

# Série: mesmo script do CI, dentro do container (o host não tem psql).
docker cp wyd-ops/scripts/ci/apply-series.sh "$CONTAINER:/tmp/apply-series.sh"
docker exec "$CONTAINER" mkdir -p /tmp/wyd-root/crates/wyd-db
docker cp crates/wyd-db/migrations "$CONTAINER:/tmp/wyd-root/crates/wyd-db/migrations"
docker exec \
  -e PGHOST=127.0.0.1 -e PGUSER=postgres -e PGPASSWORD=test -e PGDATABASE=wyd \
  "$CONTAINER" bash /tmp/apply-series.sh /tmp/wyd-root

# Alvo, com o mesmo env da config de launch.
WYD_DB_TEST_CONTAINER="$CONTAINER" \
  cargo test --manifest-path crates/wyd-db/Cargo.toml --test sistema_pg -- --test-threads=1
