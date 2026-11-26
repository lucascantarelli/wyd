#!/usr/bin/env bash
# Papéis de `db/roles.sql` para os alvos de sistema do `wyd-db`.
#
# Uso: db-prepare.sh <nologin|migrator>
# Espera PGHOST, PGPORT, PGUSER, PGPASSWORD e PGDATABASE no ambiente.
set -euo pipefail

MODO="${1:?uso: db-prepare.sh <nologin|migrator>}"
case "$MODO" in
  nologin | migrator) ;;
  *)
    echo "modo inválido: $MODO (esperado nologin ou migrator)" >&2
    exit 2
    ;;
esac

PSQL=(psql -v ON_ERROR_STOP=1 -X -q)

existe() {
  [ "$("${PSQL[@]}" -tAc "SELECT 1 FROM pg_roles WHERE rolname='$1'")" = "1" ]
}

criar() {
  if ! existe "$1"; then
    "${PSQL[@]}" -c "CREATE ROLE $1 NOLOGIN"
  fi
}

for papel in wyd_owner wyd_migrator wyd_game wyd_portal wyd_admin wyd_read; do
  criar "$papel"
done

"${PSQL[@]}" -c "GRANT wyd_owner TO wyd_migrator"
"${PSQL[@]}" -c "GRANT CREATE, CONNECT ON DATABASE $PGDATABASE TO wyd_migrator"

if [ "$MODO" = "migrator" ]; then
  # O preflight do runner recusa outro papel; LOGIN e senha valem só neste
  # container descartável.
  "${PSQL[@]}" -c "ALTER ROLE wyd_migrator LOGIN PASSWORD 'test'"
  "${PSQL[@]}" -c "GRANT CREATE, USAGE ON SCHEMA public TO wyd_migrator"
fi
