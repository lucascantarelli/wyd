# WYDPOR-F10 — Mercado: reputação e notificação

> Pergunta: "o que a fatia F10 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F10; `docs/specs/requisitos.md` (`RF-POR-029`, `RF-POR-030`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F10 — Mercado: reputação e notificação
**Fase:** 6


## Contexto

A fatia F10 do `wyd-portal` cobre avaliação do comprador, agregado público do vendedor, central de avisos com contador e existe para
cumprir `RF-POR-029`, `RF-POR-030` sem antecipar a fatia seguinte.

## Entregável

avaliação do comprador, agregado público do vendedor, central de avisos com contador

## DoD

- [ ] uma avaliação por transação e só pelo comprador.
- [ ] agregado confere com as transações concluídas.
- [ ] avaliação oculta não soma.
- [ ] cada evento gera um aviso por destinatário.
- [ ] contador zera ao ler.

## Fora de escopo

comentário com resposta do vendedor e moderação editorial de texto livre

## Depende de

F8 e F9 (transação concluída é a base da avaliação)

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F10 concluída em `docs/roadmap/roadmap.md`.
