# 0020 — seed do mundo

> Pergunta: "o que o `0020-seed-world.sql` escreve, e onde entram as 4 cidades que não têm dataset de
> conteúdo?"
> Público: quem escreve o seed, a curadoria de conteúdo e quem revisa a série
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §4; `pedido-de-curadoria-dos-seeds.md` §1 e §3.7;
> `proposta-decisoes-seeds.md` `Q-M34-3`; `migrations/0019_content_world.md`;
> `migrations/0011_game_economy.md`
> Revisado em: 2026-11-27

**Rascunho.** Esta página não tem linha `**Estado:**` porque é rascunho, como define `docs/convencoes.md`
§Estados: o `.sql` não existe e o seed segue `bloqueada` por dado (`migrations-checklist.md` §2, linha 20).
Ela registra o escopo medido e a nota de `Q-M34-3`; não vale como página `concluída` enquanto o arquivo
não for escrito e conferido contra §5 do checklist.

## 1. O que o seed escreve (`V`, 2026-11-27)

| Bloco | Tabelas | Linhas |
|---|---|---:|
| `content` — mundo | as 12 do `0019` (`npc_template`, `npc_definition`, `npc_shop_item`, `level_item`, `guard_spawn`, `guard_spawn_reward`, `mob_merc`, `game_setting`, `game_matrix`, `region`, `world_item_seed`, `guild_zone`) | 15.142 |
| `content.content_source` | proveniência das fontes, inclusive a de `0017`/`0018` | 12 |
| `content.content_version` | 1 incremento; fecha em **7** | 1 |
| `game.city` | as 4 cidades de `Q-M34-3` | 4 |

As contagens declaradas e medidas, com as divergências nomeadas, estão em `plano-migrations.md` §4; esta
página não as repete.

## 2. Nota das 4 cidades (`Q-M34-3`, 2026-11-27)

A decisão do dono (A em `Q-M34-3`) põe as 4 cidades em `game.city` por este seed — Armia, Azran, Erion e
Noatun (capital), com `tax_rate_bps` 1000. `map_id` fica nulo porque o cliente 7662 não publica o mapa
(`H`, `0011_game_economy.sql`), e `region_id` não é gravado porque o schema novo não modela o vínculo
cidade ↔ região que o legado tinha (`deprecated/crates/wyd-db/migrations/0010_economia.sql`).

A tabela é de `game`, não de `content`: o seed é o primeiro posterior ao `0011` que carrega dado fixo de
mundo, e a página registra o desvio de ordem que `plano-migrations.md` §2 descreve. O `0027` não herda
estas linhas — ele fica com quests, eventos e arenas.

## 3. O que falta para fechar

- a medição da curadoria no cliente 7662, no formato de `pedido-de-curadoria-dos-seeds.md` §4;
- a produção de `0017` e `0020` pela exceção datada de 2026-11-27 (`M34`, script descartável);
- o `.sql` escrito e conferido, com `content_source` (`path`, `sha256`, `size_bytes`, `row_count`);
- a revisão desta página contra os itens de §5 de `migrations-checklist.md`, que hoje não se aplicam a um
  rascunho.
