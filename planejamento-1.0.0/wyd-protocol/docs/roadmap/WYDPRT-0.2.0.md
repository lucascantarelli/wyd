# WYDPRT-0.2.0 — fatia 0.2.0 do fio

> Pergunta: "que issues entram na fatia 0.2.0 e como sei que ela terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-protocol`
> Base: `docs/roadmap/fases.md` §3; `wyd-protocol-1.0.0.md` §4, §9 e §9.1; `lacunas.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** 0.2.0

**Dono do valor:** escopo, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.

## Contexto

A numeração `WYDPRT-nnn` de cada issue é aplicada por `wyd-protocol-1.0.0.md` §7 e não é antecipada
aqui; esta página fixa o escopo da fatia 0.2.0 e o que ela prova.

## Entregável

famílias `autenticacao` e `personagem`, mais o que fecha `LPRT-06` antes delas

## DoD

- [ ] Todas as issues da fatia 0.2.0 fechadas com evidência e golden datado quando houver byte novo.
- [ ] `cargo test -p wyd-protocol` e o gate do sistema verdes.
- [ ] Nenhuma linha de fio sem prova (`V`) ou hipótese marcada (`H` com o ID da lacuna).

## Fora de escopo

Famílias e versões seguintes; captura em produção (`wyd-ops`); regra de jogo (`wyd-core`/`wyd-sim`).

## Depende de

Fase anterior de `docs/roadmap/fases.md` §3; captura `LPRT-03`/`LPRT-06` quando a fatia exigir byte novo.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída de `cargo test` e `goldens/catalogo.md` atualizado no PR.
