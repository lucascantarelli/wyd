# WYDPOR-F8 — Mercado: carteira e venda direta

> Pergunta: "o que a fatia F8 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F8; `docs/specs/requisitos.md` (`RF-POR-023`, `RF-POR-026` a `RF-POR-028`, `RF-POR-031`, `RNF-POR-009`, `RNF-POR-010`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F8 — Mercado: carteira e venda direta
**Fase:** 6


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

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F8 concluída em `docs/roadmap/roadmap.md`.
