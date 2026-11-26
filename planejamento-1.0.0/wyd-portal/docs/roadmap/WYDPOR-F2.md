# WYDPOR-F2 — Portal público: home, notícias e rankings

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-POR-005`, `RF-POR-006`, `RF-POR-007`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F2 — Portal público: home, notícias e rankings
**Fase:** 3


## Contexto

A fatia F2 do `wyd-portal` cobre home, `/noticias*`, `/rankings`, `/download` e existe para
cumprir `RF-POR-005`, `RF-POR-006`, `RF-POR-007` sem antecipar a fatia seguinte.

## Entregável

home, `/noticias*`, `/rankings`, `/download`

## DoD

- [ ] home com banco vazio mostra `0` e hora da medição.
- [ ] ranking exclui `deleted_at`.
- [ ] rascunho de notícia não aparece.

## Fora de escopo

WYDpedia e atlas

## Depende de

F0; views de `0029`; lote das tabelas `portal.news`/`banner` para notícias

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F2 concluída em `docs/roadmap/roadmap.md`.
