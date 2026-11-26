#!/usr/bin/env bash
# Gate editorial do site — fonte única.
#
# O CI (`.github/actions/docs-check`) e o `docker compose --profile gate` chamam
# este script. As dependências Python precisam existir antes: o composite instala
# `wyd-docs/requirements.txt`; a imagem `:gate` já as traz.
#
# Uso:
#   bash wyd-docs/gate.sh [<raiz-do-repositorio>]
#
# No Compose a árvore de evidência já está copiada na imagem `:gate`; não há bind
# mount. A montagem, o build e a verificação rodam em `wyd-docs/` (a cópia
# gravável da imagem), enquanto a guarda de lacunas lê a evidência em `<raiz>`.
set -euo pipefail

AQUI="$(cd "$(dirname "$0")" && pwd)"
ROOT="${1:-$(cd "$AQUI/.." && pwd)}"

cd "$AQUI"

python verificar-skills.py --root "$ROOT"
python verificar-lacunas.py --root "$ROOT"
python preparar.py
mkdocs build --dir .build/docs --output site
python verificar.py site
python verificar-contagens.py
python verificar-censos.py
