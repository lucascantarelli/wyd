# WYDPOR-F9 — Mercado: leilão

> Pergunta: "o que a fatia F9 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F9; `docs/specs/requisitos.md` (`RF-POR-025`, `RF-POR-030` (lance e arremate), `RNF-POR-009`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F9 — Mercado: leilão
**Fase:** 6

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F9 do `wyd-portal` cobre anúncio `leilao` com `ends_at`, lances com reserva de crédito, fechamento por job, aviso de arremate e de lance superado e existe para
cumprir `RF-POR-025`, `RF-POR-030` (lance e arremate), `RNF-POR-009` sem antecipar a fatia seguinte.

## Entregável

anúncio `leilao` com `ends_at`, lances com reserva de crédito, fechamento por job, aviso de arremate e de lance superado

## DoD

- [ ] lance menor ou igual ao maior ativo é recusado.
- [ ] lance depois de `ends_at` é recusado.
- [ ] o job roda duas vezes e não cobra duas vezes.
- [ ] perdedor recebe a reserva de volta.
- [ ] vencedor recebe o item.
- [ ] leilão sem lance expira e devolve a custódia ao vendedor.
- [ ] atraso do fechamento tem métrica e alerta.

## Fora de escopo

lance automático (proxy bid) e compra imediata em leilão sem `buyout_price`

## Depende de

F8; `LAC-POR-09` confirmada em 2026-11-23 (BFF fecha, `wyd-ops` agenda/alerta); job idempotente

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

Saída do comando do DoD no PR, com a fatia F9 concluída em `docs/roadmap/roadmap.md`.
