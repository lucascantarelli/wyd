#!/usr/bin/env bash
# Gate editorial do site — entrada única.
#
# O CI (`.github/actions/docs-check`) e o `docker compose --profile gate` chamam
# este script. As dependências Python precisam existir antes: o composite instala
# `wyd-docs/requirements.txt`; a imagem `:gate` já as traz.
#
# Uso:
#   bash wyd-docs/gate.sh [<raiz-do-repositorio>] [--json|--details]
#
# O contrato de saída e os subcomandos ficam em `wyd-docs/gate/cli.py`. Este
# arquivo só preserva o caminho estável que o Compose e o CI já usam.
set -euo pipefail

AQUI="$(cd "$(dirname "$0")" && pwd)"
if [[ "${1:-}" == --* ]]; then
  ROOT="$(cd "$AQUI/.." && pwd)"
else
  ROOT="${1:-$(cd "$AQUI/.." && pwd)}"
  shift || true
fi

cd "$AQUI"
exec python -m gate full --root "$ROOT" "$@"
