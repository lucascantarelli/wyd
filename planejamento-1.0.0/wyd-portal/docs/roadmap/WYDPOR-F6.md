# WYDPOR-F6 — CMS e launcher

> Pergunta: "o que a fatia F6 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F6; `docs/specs/requisitos.md` (`RF-POR-006` (edição)); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F6 — CMS e launcher
**Fase:** 5


## Contexto

A fatia F6 do `wyd-portal` cobre `/admin/conteudo/noticias`, `/admin/conteudo/wydpedia`, `/admin/launcher` e existe para
cumprir `RF-POR-006` (edição) sem antecipar a fatia seguinte.

## Entregável

`/admin/conteudo/noticias`, `/admin/conteudo/wydpedia`, `/admin/launcher`

## DoD

- [ ] publicar/despublicar notícia reflete no público em até um ciclo de revalidação.
- [ ] versão de launcher só publica manifesto íntegro.

## Fora de escopo

geração de patch e upload de binário (`wyd-ops`)

## Depende de

— (`0030` aprovado em 2026-11-20: `news`, `banner`, `guide`, `launcher_release`)

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F6 concluída em `docs/roadmap/roadmap.md`.
