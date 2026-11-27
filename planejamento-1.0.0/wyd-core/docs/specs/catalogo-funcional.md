# Catálogo funcional — sistemas de jogo

> Pergunta: "quais sistemas de jogo (guerras, mobs, NPCs, canais, skills, classes, quests, armas, armaduras, lojas, mercado, crafting, magias, poções, pets, montarias) estão mapeados e quem é o dono?"
> Público: Produto, arquitetura e desenvolvimento
> Dono: `wyd-core` (modelo) + `wyd-db` (catálogo/dicionário)
> Base: `planejamento-1.0.0/wyd-db/docs/dicionario-dados/dominio-catalogo.md`; `wyd-core-1.0.0.md`; `wyd-portal/docs/arquitetura/...`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Catálogo é dono do `wyd-db`; esta página é o mapa de fronteira.

## 1. Dono por área

| Área | Dono do modelo | Dono do catálogo/dado | Estado |
|---|---|---|---|
| Guerra / PvP / canal | `wyd-core` | `wyd-db` (tabelas `game.*`) | proposto |
| Mobs / NPCs / IA | `wyd-core` | `wyd-db` | proposto |
| Classes / skills / feitiços | `wyd-core`/`wyd-sim` | `wyd-db` | proposto |
| Quests | `wyd-core` | `wyd-db` | proposto |
| Armas / armaduras / itens | `wyd-core` | `wyd-db` (domínio-catalogo) | proposto |
| Lojas / mercado / crafting | `wyd-core`/`wyd-portal` | `wyd-db` | proposto |
| Poções / buffs | `wyd-core`/`wyd-sim` | `wyd-db` | proposto |
| Pets / companheiros / montarias | `wyd-core` | `wyd-db` | proposto |
| Canais / chat | `wyd-net`/`wyd-server` | `wyd-db` | proposto |

## 2. Fonte de verdade

- Catálogo de itens: `planejamento-1.0.0/wyd-db/docs/dicionario-dados/dominio-catalogo.md`.
- Fórmulas/curvas: `wyd-sim` (determinísticas, com `stat_source`).
- Captura do cliente 7662: `wyd-client` (`LCLI-02`/`LCLI-03`).

## 3. O que esta página não cobre

- Números de balanceamento (dono: `wyd-sim`, com fixture capturada).
- Conteúdo seedado (depende de curadoria, `pedido-de-curadoria-dos-seeds.md`).
