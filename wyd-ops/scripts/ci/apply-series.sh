#!/usr/bin/env bash
# Aplica o bloco `-- migrate:up` de cada migration, em ordem de nome.
#
# Uso: apply-series.sh [<raiz-do-repositorio>]
# Espera PGHOST, PGPORT, PGUSER, PGPASSWORD e PGDATABASE no ambiente.
#
# A separação é textual de propósito: o alvo `sistema_pg` prova o schema, não o
# parser do runner. O `awk` remove CRLF antes de comparar os marcadores.
set -euo pipefail

ROOT="${1:-$(cd "$(dirname "$0")/../../.." && pwd)}"
SERIE="$ROOT/crates/wyd-db/migrations"

if [ ! -d "$SERIE" ]; then
  echo "série não encontrada em $SERIE" >&2
  exit 2
fi

for arquivo in "$SERIE"/*.sql; do
  awk '
    { linha = $0; sub(sprintf("%c", 13), "", linha) }
    linha == "-- migrate:up"   { if (up) exit 4; up = 1; dentro = 1; next }
    linha == "-- migrate:down" { if (!up || down) exit 5; down = 1; dentro = 0; next }
    dentro { print linha }
    END { if (!up || !down) exit 6 }
  ' "$arquivo" | psql -v ON_ERROR_STOP=1 -X -q
done
