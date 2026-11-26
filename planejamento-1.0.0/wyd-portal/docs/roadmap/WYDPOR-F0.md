# WYDPOR-F0 — Fundação do portal (auth e sessão)

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-POR-001` a `RF-POR-003`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F0 — Fundação do portal (auth e sessão)
**Fase:** 1


## Contexto

A fatia F0 do `wyd-portal` cobre app Next.js, login, cadastro, recuperação, middleware de sessão, layout dos dois temas e existe para
cumprir `RF-POR-001` a `RF-POR-003` sem antecipar a fatia seguinte.

## Entregável

app Next.js, login, cadastro, recuperação, middleware de sessão, layout dos dois temas

## DoD

- [ ] E2E de cadastro → verificação → login → logout verde.
- [ ] sessão revogada não acessa `/conta`.
- [ ] `npm run build` e `tsc --noEmit` limpos.

## Fora de escopo

área administrativa, WYDpedia, atlas

## Depende de

`0001`–`0004` aplicadas

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F0 concluída em `docs/roadmap/roadmap.md`.
