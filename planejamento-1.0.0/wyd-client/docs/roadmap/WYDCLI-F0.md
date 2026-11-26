# WYDCLI-F0 — Inventário e captura

> Pergunta: "o que a fatia F0 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-client`
> Base: `docs/roadmap/roadmap.md` fatia F0; `docs/specs/requisitos.md` (`RF-CLI-001`, `RF-CLI-002`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F0 — Inventário e captura
**Fase:** 1


## Contexto

A fatia F0 do `wyd-client` cobre inventário item a item (`docs/specs/inventario-da-arvore.md`), pacote do pedido (`docs/specs/pedido-ao-titular.md`) e existe para
cumprir `RF-CLI-001`, `RF-CLI-002` sem antecipar a fatia seguinte.

## Entregável

inventário item a item (`docs/specs/inventario-da-arvore.md`), pacote do pedido (`docs/specs/pedido-ao-titular.md`)

## DoD

- [ ] inventário com hash/tamanho por arquivo.
- [ ] pedido enviado e protocolado.

## Fora de escopo

patches

## Depende de

A1 autorizada; pedido ao titular

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
