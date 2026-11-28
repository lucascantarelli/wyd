#!/usr/bin/env bash
# Sentinela de imagem-base: compara o digest pinado em Dockerfile/Compose com o digest
# que a tag aponta hoje no registro. O `check-pins.sh` prova que existe pin; ele não prova
# que o pin ainda é o da tag — digest pinado envelhece em silêncio, e o Dependabot não
# alerta CVE em imagem de Dockerfile.
#
# Depende de rede e de `docker buildx imagetools`, por isso roda **só** no `images.yml`
# (agendado e manual), nunca no gate de PR: o check de PR continua offline e determinístico.
# Cada tag é resolvida com até três tentativas para não confundir instabilidade de registro
# com imagem velha. Exit 1 quando alguma tag andou, e o `notificar falha` do workflow avisa.
#
# Dono: `wyd-ops`. Uso: `bash wyd-ops/scripts/ci/check-image-freshness.sh [raiz]`.
set -uo pipefail

raiz="${1:-.}"
falhas=0
conferidas=0
propria='^ghcr\.io/[A-Za-z0-9_.-]+/wyd-docs(:|$)|^wyd/[a-z0-9-]+(:|$)'

resolver() {
  local repo_tag="$1" tentativa digest
  for tentativa in 1 2 3; do
    digest=$(docker buildx imagetools inspect "$repo_tag" 2>/dev/null | awk '/^Digest:/{print $2; exit}')
    if [ -n "$digest" ]; then
      printf '%s\n' "$digest"
      return 0
    fi
    sleep 5
  done
  return 1
}

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
        < <(grep -hoE '^[[:space:]]*FROM[[:space:]]+[^[:space:]]+@sha256:[0-9a-f]{64}' "$arq" \
              | sed -E 's/^[[:space:]]*FROM[[:space:]]+//')
      ;;
    *)
      while IFS= read -r ref; do refs+=("$ref"); done \
        < <(grep -hoE '^[[:space:]]*image:[[:space:]]*[^[:space:]]+@sha256:[0-9a-f]{64}' "$arq" \
              | sed -E 's/^[[:space:]]*image:[[:space:]]*//; s/^"//; s/"$//')
      ;;
  esac
done

mapfile -t unicas < <(printf '%s\n' "${refs[@]:-}" | sed '/^$/d' | sort -u)

echo "== frescor das imagens pinadas =="
for ref in "${unicas[@]}"; do
  if printf '%s' "$ref" | grep -qE "$propria"; then
    continue
  fi
  repo_tag="${ref%@*}"
  pin="${ref#*@}"
  if ! atual=$(resolver "$repo_tag"); then
    echo "::error::não resolvi $repo_tag no registro (rede ou tag removida)"
    falhas=1
    continue
  fi
  conferidas=$((conferidas + 1))
  if [ "$pin" != "$atual" ]; then
    echo "::error::$repo_tag está pinado em $pin, mas a tag aponta para $atual — atualizar o digest"
    falhas=1
  else
    echo "  ok  $repo_tag"
  fi
done

if [ "$falhas" -ne 0 ]; then
  echo "imagem base fora de política" >&2
  exit 1
fi
echo "frescor: OK ($conferidas imagem(ns) conferida(s))"
