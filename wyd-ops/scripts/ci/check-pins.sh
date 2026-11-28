#!/usr/bin/env bash
# Política de pin do repositório, verificável sem rede:
#   1. toda action de terceiro é referenciada por SHA de 40 hex (tag ou branch não vale);
#   2. toda imagem de terceiro citada em Dockerfile, Compose ou `docker run/pull` de workflow
#      carrega `@sha256:` — tag sozinha aponta para o que o registro quiser servir hoje.
#
# Ficam fora da regra 2 as imagens construídas neste repositório (`wyd/*` e
# `ghcr.io/<dono>/wyd-docs`): a tag delas é o contrato de publicação, não pin de supply chain.
# A descoberta usa `git ls-files`: árvore de fixture (`.build/`), legado (`deprecated/`) e
# `node_modules/` não são código deste repositório e não entram na conta.
#
# Dono: `wyd-ops`. Uso: `bash wyd-ops/scripts/ci/check-pins.sh [raiz]`.
# Roda no job `pinos` de `security.yml` e é barato o bastante para todo PR.
set -uo pipefail

raiz="${1:-.}"
falhas=0

propria='^ghcr\.io/[A-Za-z0-9_.-]+/wyd-docs(:|$)|^wyd/[a-z0-9-]+(:|$)'

echo "== actions por SHA de 40 hex =="
while IFS= read -r ref; do
  [ -n "$ref" ] || continue
  case "$ref" in
    ./*) continue ;;
  esac
  if ! printf '%s' "$ref" | grep -qE '@[0-9a-f]{40}([[:space:]]|$)'; then
    echo "::error::action sem SHA de 40 hex: $ref"
    falhas=1
  fi
done < <(grep -rhoE 'uses:[[:space:]]*[^[:space:]]+' "$raiz/.github" 2>/dev/null | sed -E 's/uses:[[:space:]]*//')

echo "== imagens por digest =="
arquivos=()
if git -C "$raiz" rev-parse --git-dir >/dev/null 2>&1; then
  while IFS= read -r arq; do
    [ -n "$arq" ] && arquivos+=("$raiz/$arq")
  done < <(git -C "$raiz" ls-files -- '*Dockerfile' 'docker-compose*.yml')
else
  while IFS= read -r arq; do
    [ -n "$arq" ] && arquivos+=("$arq")
  done < <(find "$raiz" -maxdepth 4 \( -name '*Dockerfile' -o -name 'docker-compose*.yml' \) 2>/dev/null)
fi

refs=()
for arq in "${arquivos[@]}"; do
  case "$(basename "$arq")" in
    *Dockerfile)
      while IFS= read -r ref; do refs+=("$ref"); done \
        < <(grep -hoE '^[[:space:]]*FROM[[:space:]]+[^[:space:]]+' "$arq" | sed -E 's/^[[:space:]]*FROM[[:space:]]+//')
      ;;
    *)
      while IFS= read -r ref; do refs+=("$ref"); done \
        < <(grep -hoE '^[[:space:]]*image:[[:space:]]*[^[:space:]]+' "$arq" | sed -E 's/^[[:space:]]*image:[[:space:]]*//; s/^"//; s/"$//')
      ;;
  esac
done
# `docker run`/`docker pull` em workflow: o token de imagem mora numa linha de continuação.
while IFS= read -r ref; do refs+=("$ref"); done < <(
  grep -rhoE '^[[:space:]]+[^[:space:]]+/[^[:space:]]+:[^[:space:]]+' --include='*.yml' "$raiz/.github/workflows" 2>/dev/null \
    | sed -E 's/^[[:space:]]+//'
)

for ref in "${refs[@]}"; do
  case "$ref" in
    */*|*:*) ;;
    *) continue ;;  # alias de estágio (`FROM build`) não é imagem de registro
  esac
  if printf '%s' "$ref" | grep -qE "$propria"; then
    continue
  fi
  if ! printf '%s' "$ref" | grep -qE '@sha256:[0-9a-f]{64}$'; then
    echo "::error::imagem sem digest: $ref"
    falhas=1
  fi
done

if [ "$falhas" -ne 0 ]; then
  echo "política de pin violada" >&2
  exit 1
fi
echo "política de pin: OK ($(grep -rhoE 'uses:[[:space:]]*[^[:space:]]+' "$raiz/.github" 2>/dev/null | wc -l | tr -d ' ') referência(s) de action; ${#refs[@]} referência(s) de imagem)"
