#!/usr/bin/env bash
# Fecha o marco da versão publicada e abre o próximo quando a release é final
# (`vX.Y.Z`, sem `-rc`). O candidato não fecha nada: `-rc.N` é pré-release.
#
# A política de versão é `planejamento-1.0.0/wyd-ops/docs/devops/index.md` §13.
set -euo pipefail

tag="${1:?uso: fechar-milestone.sh vX.Y.Z[-rc.N]}"
repo="${GITHUB_REPOSITORY:?GITHUB_REPOSITORY ausente}"

if ! printf '%s' "$tag" | grep -Eq '^v[0-9]+\.[0-9]+\.[0-9]+(-rc\.[0-9]+)?$'; then
  echo "::error::tag '$tag' fora da política vX.Y.Z ou vX.Y.Z-rc.N"
  exit 1
fi

if printf '%s' "$tag" | grep -q -- '-rc\.'; then
  echo "tag '$tag' é candidato: nenhum marco é fechado"
  exit 0
fi

versao="${tag#v}"
maior="${versao%%.*}"
resto="${versao#*.}"
menor="${resto%%.*}"
patch="${versao##*.}"

# O marco pode estar nomeado com ou sem o `v`.
numero=$(gh api --paginate "repos/${repo}/milestones?state=open&per_page=100" \
  --jq "[.[] | select(.title == \"${versao}\" or .title == \"v${versao}\")][0].number // empty")

if [ -z "${numero:-}" ]; then
  echo "::warning::nenhum milestone aberto com título '${versao}' ou 'v${versao}'; nada a fechar"
  exit 0
fi

abertas=$(gh api "repos/${repo}/milestones/${numero}" --jq '[.open_issues, .closed_issues] | @tsv')
abertas_total=$(printf '%s' "$abertas" | cut -f1)
if [ "${abertas_total:-0}" -gt 0 ]; then
  echo "::warning::milestone ${versao} tem ${abertas_total} issue(s) aberta(s); fechando mesmo assim (a tag é a fonte da versão)"
fi

gh api --method PATCH "repos/${repo}/milestones/${numero}" \
  --field state=closed --field due_on="$(date --utc +%Y-%m-%dT%H:%M:%SZ)" >/dev/null
echo "milestone ${versao} fechado (nº ${numero})"

# Próximo marco: a política é `0.y.z` até o 1.0.0, então o próximo é o minor seguinte.
proximo="${maior}.$((menor + 1)).0"
existe=$(gh api --paginate "repos/${repo}/milestones?state=open&per_page=100" \
  --jq "[.[] | select(.title == \"${proximo}\")] | length")
if [ "${existe:-0}" -eq 0 ]; then
  gh api --method POST "repos/${repo}/milestones" \
    --field title="$proximo" \
    --field description="Próximo marco depois de ${versao} (política §13: SemVer; ${proximo} é o minor seguinte)." >/dev/null
  echo "milestone ${proximo} aberto"
else
  echo "milestone ${proximo} já existia"
fi
