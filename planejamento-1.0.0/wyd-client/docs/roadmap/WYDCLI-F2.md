# WYDCLI-F2 — Fechamento

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-client`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-CLI-005`, `RF-CLI-006`, `RF-CLI-007`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F2 — Fechamento
**Fase:** 3

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F2 do `wyd-client` cobre lacunas fechadas, documentação final, suíte e existe para
cumprir `RF-CLI-005`, `RF-CLI-006`, `RF-CLI-007` sem antecipar a fatia seguinte.

## Entregável

lacunas fechadas, documentação final, suíte

## DoD

- [ ] `LCLI-02`/`LCLI-03`/`LCLI-05` resolvidas ou com dono+prazo.
- [ ] cobertura ≥ 85% onde aplicável.

## Fora de escopo

distribuição pública

## Depende de

F0–F1; CI real

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

Saída do comando do DoD no PR, com a fatia F2 concluída em `docs/roadmap/roadmap.md`.
