# WYDPOR-F3 — WYDpedia e atlas

> Pergunta: "o que a fatia F3 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F3; `docs/specs/requisitos.md` (`RF-POR-008`, `RF-POR-009`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F3 — WYDpedia e atlas
**Fase:** 3


## Contexto

A fatia F3 do `wyd-portal` cobre `/wydpedia*`, `/atlas*`, busca e tooltip e existe para
cumprir `RF-POR-008`, `RF-POR-009` sem antecipar a fatia seguinte.

## Entregável

`/wydpedia*`, `/atlas*`, busca e tooltip

## DoD

- [ ] 404 para item inexistente.
- [ ] busca ignora caixa.
- [ ] mapa sem pins mostra estado vazio.
- [ ] link cruzado mínimo por ficha.

## Fora de escopo

simulador de build e calculadora de dano

## Depende de

F2; `0013`–`0019`, `0024` e seus seeds

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F3 concluída em `docs/roadmap/roadmap.md`.
