# WYDPOR-F7 — Armaria read-only

> Pergunta: "o que a fatia F7 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F7; `docs/specs/requisitos.md` (`RF-POR-019` a `RF-POR-022`, `RNF-POR-008`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F7 — Armaria read-only
**Fase:** 6


## Contexto

A fatia F7 do `wyd-portal` cobre `/personagem/[id]` com cabeçalho, os 16 slots, habilidades, ficha do item, filtro e ordenação e existe para
cumprir `RF-POR-019` a `RF-POR-022`, `RNF-POR-008` sem antecipar a fatia seguinte.

## Entregável

`/personagem/[id]` com cabeçalho, os 16 slots, habilidades, ficha do item, filtro e ordenação

## DoD

- [ ] rota responde sem sessão.
- [ ] personagem apagado devolve `404`.
- [ ] resposta sem `account_id`, e-mail, IP ou ouro.
- [ ] slot vazio e item não catalogado não quebram a página.
- [ ] a ficha exibe habilidades especiais pelo símbolo e, quando houver origem provada, dano e alcance.
- [ ] durabilidade não aparece como número.
- [ ] `stat_source` declara o que não foi medido e nenhum número é inventado.
- [ ] nenhuma escrita do portal em `game`.

## Fora de escopo

edição de perfil, importação manual de dado, simulador de build

## Depende de

F2; views de `wyd-db/armaria-e-mercado.md` §2 no lote; `LAC-POR-08` fechada em 2026-11-22 (ficha publica só o provado; captura em `P-POR-08`)

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F7 concluída em `docs/roadmap/roadmap.md`.
