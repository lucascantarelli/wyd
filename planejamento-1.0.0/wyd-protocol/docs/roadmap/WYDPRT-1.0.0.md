# WYDPRT-1.0.0 — fatia 1.0.0 do fio

> Pergunta: "que issues entram na fatia 1.0.0 e como sei que ela terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-protocol`
> Base: `docs/roadmap/fases.md` §3; `wyd-protocol-1.0.0.md` §4, §9 e §9.1; `lacunas.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** 1.0.0


## Contexto

A numeração `WYDPRT-nnn` de cada issue é aplicada por `wyd-protocol-1.0.0.md` §7 e não é antecipada
aqui; esta página fixa o escopo da fatia 1.0.0 e o que ela prova.

## Entregável

congelamento: catálogo medido do gerador, goldens por família e compatibilidade declarada (`LPRT-09`)

## DoD

- [ ] Todas as issues da fatia 1.0.0 fechadas com evidência e golden datado quando houver byte novo.
- [ ] `cargo test -p wyd-protocol` e o gate do sistema verdes.
- [ ] Nenhuma linha de fio sem prova (`V`) ou hipótese marcada (`H` com o ID da lacuna).

## Fora de escopo

Famílias e versões seguintes; captura em produção (`wyd-ops`); regra de jogo (`wyd-core`/`wyd-sim`).

## Depende de

Fase anterior de `docs/roadmap/fases.md` §3; captura `LPRT-03`/`LPRT-06` quando a fatia exigir byte novo.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `documentation-writer` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída de `cargo test` e `goldens/catalogo.md` atualizado no PR.
