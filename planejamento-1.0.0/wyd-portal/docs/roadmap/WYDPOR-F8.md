# WYDPOR-F8 — Mercado: carteira e venda direta

> Pergunta: "o que a fatia F8 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F8; `docs/specs/requisitos.md` (`RF-POR-023`, `RF-POR-026` a `RF-POR-028`, `RF-POR-031`, `RNF-POR-009`, `RNF-POR-010`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F8 — Mercado: carteira e venda direta
**Fase:** 6

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F8 do `wyd-portal` cobre `/mercado`, `/mercado/[id]`, `/mercado/minhas`, `/mercado/carteira`; anúncio, compra, taxa e extrato e existe para
cumprir `RF-POR-023`, `RF-POR-026` a `RF-POR-028`, `RF-POR-031`, `RNF-POR-009`, `RNF-POR-010` sem antecipar a fatia seguinte.

## Entregável

`/mercado`, `/mercado/[id]`, `/mercado/minhas`, `/mercado/carteira`; anúncio, compra, taxa e extrato

## DoD

- [ ] item sai do inventário antes de o anúncio ficar `ativo`.
- [ ] compra entrega o item e liquida com `taxa + líquido = bruto`.
- [ ] repetir a requisição não duplica anúncio nem cobrança.
- [ ] comprar da própria conta é recusado.
- [ ] saldo confere com a soma do ledger.
- [ ] teste de carga com 10 mil anúncios mede `p95` dentro do `RNF-POR-009`.

## Fora de escopo

leilão (F9), reputação e avisos (F10), disputa e antifraude (F11)

## Depende de

F7; lote do mercado (`armaria-e-mercado.md` §3, `LAC-ADM-07`); contrato de custódia do `wyd-admin` (`contrato-c4.md` §4.9–§4.13, `ADM-10`; `LAC-POR-07` fechada em 2026-11-22)

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

Saída do comando do DoD no PR, com a fatia F8 concluída em `docs/roadmap/roadmap.md`.
