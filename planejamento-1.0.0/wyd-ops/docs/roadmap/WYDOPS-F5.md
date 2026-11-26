# WYDOPS-F5 — Gate e fechamento

> Pergunta: "o que a fatia F5 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-ops`
> Base: `docs/roadmap/roadmap.md` fatia F5; `docs/specs/requisitos.md` (`RF-OPS-007` e todos); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F5 — Gate e fechamento
**Fase:** 4


## Contexto

A fatia F5 do `wyd-ops` cobre gate verde, censos coincidem, cobertura e existe para
cumprir `RF-OPS-007` e todos sem antecipar a fatia seguinte.

## Entregável

gate verde, censos coincidem, cobertura

## DoD

- [ ] `docker compose --profile gate run --rm gate` verde.
- [ ] cobertura ≥ 85% onde aplicável.

## Fora de escopo

migrations

## Depende de

F0–F4; CI real

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F5 concluída em `docs/roadmap/roadmap.md`.
