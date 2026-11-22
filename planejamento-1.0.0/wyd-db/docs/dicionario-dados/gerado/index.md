# wyd-db — referência por tabela

**Estado:** **66 de 97** páginas escritas (2026-11-22). A produção é manual, em lotes (`M26`), sem
gerador: cada página resume a tabela e aponta para a página de `migrations/` dona da forma. Este índice
lista as páginas de `plano-migrations.md` §3 conforme elas existem.

**Páginas de migration não publicadas (`V`, 2026-11-21):** as 26 páginas escritas (de 32 em
`migrations-checklist.md` §2; as outras 6 seguem `bloqueada`) ficam em
`planejamento-1.0.0/wyd-db/migrations/`, o lugar que `convencoes.md` lhes dá. O site monta só
`planejamento-1.0.0/<sistema>/docs/`, então essas páginas ficam de fora — falta publicação, não árvore.
Por isso as páginas de tabela citam a migration dona pelo caminho, em `code`, sem link: a convenção
(*Links e citações*) pede caminho em `code` para alvo fora de `docs/`, e link relativo para lá reprova
no build.

## Lote 1 — identidade (`0001`–`0007`), 22 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `content.content_version` | [content_version](content_version.md) | `0001-bootstrap` |
| `ops.audit_log` | [audit_log](audit_log.md) | `0002-ops-audit` |
| `ops.login_attempt` | [login_attempt](login_attempt.md) | `0002-ops-audit` |
| `ops.config_revision` | [config_revision](config_revision.md) | `0002-ops-audit` |
| `ops.config_change` | [config_change](config_change.md) | `0002-ops-audit` |
| `portal.person` | [person](person.md) | `0003-portal-person` |
| `portal.person_profile` | [person_profile](person_profile.md) | `0003-portal-person` |
| `portal.person_consent` | [person_consent](person_consent.md) | `0003-portal-person` |
| `portal.person_session` | [person_session](person_session.md) | `0004-portal-access` |
| `portal.person_token` | [person_token](person_token.md) | `0004-portal-access` |
| `game.account` | [account](account.md) | `0005-game-account` |
| `game.account_wallet` | [account_wallet](account_wallet.md) | `0005-game-account` |
| `game.account_ledger` | [account_ledger](account_ledger.md) | `0005-game-account` |
| `game.account_moderation` | [account_moderation](account_moderation.md) | `0005-game-account` |
| `game.pin_reset` | [pin_reset](pin_reset.md) | `0005-game-account` |
| `game.character` | [character](character.md) | `0006-game-character` |
| `game.character_status` | [character_status](character_status.md) | `0006-game-character` |
| `game.character_scores` | [character_scores](character_scores.md) | `0006-game-character` |
| `game.character_special` | [character_special](character_special.md) | `0006-game-character` |
| `game.character_short_skill` | [character_short_skill](character_short_skill.md) | `0006-game-character` |
| `game.server_channel` | [server_channel](server_channel.md) | `0007-game-session` |
| `game.session` | [session](session.md) | `0007-game-session` |

## Lote 2 — habilidades, inventário, guilda e economia (`0008`–`0011`), 22 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `game.character_skill` | [character_skill](character_skill.md) | `0008-game-skills` |
| `game.character_learned_skill` | [character_learned_skill](character_learned_skill.md) | `0008-game-skills` |
| `game.character_lineage` | [character_lineage](character_lineage.md) | `0008-game-skills` |
| `game.character_affect` | [character_affect](character_affect.md) | `0008-game-skills` |
| `game.character_chaos` | [character_chaos](character_chaos.md) | `0008-game-skills` |
| `game.character_item` | [character_item](character_item.md) | `0009-game-inventory` |
| `game.vault_item` | [vault_item](vault_item.md) | `0009-game-inventory` |
| `game.refine_pity` | [refine_pity](refine_pity.md) | `0009-game-inventory` |
| `game.guild` | [guild](guild.md) | `0010-game-guild` |
| `game.guild_member` | [guild_member](guild_member.md) | `0010-game-guild` |
| `game.guild_relation` | [guild_relation](guild_relation.md) | `0010-game-guild` |
| `game.guild_tower` | [guild_tower](guild_tower.md) | `0010-game-guild` |
| `game.city` | [city](city.md) | `0011-game-economy` |
| `game.item_price` | [item_price](item_price.md) | `0011-game-economy` |
| `game.personal_shop` | [personal_shop](personal_shop.md) | `0011-game-economy` |
| `game.city_tax_ledger` | [city_tax_ledger](city_tax_ledger.md) | `0011-game-economy` |
| `portal.donate_shop_item` | [donate_shop_item](donate_shop_item.md) | `0011-game-economy` |
| `portal.daily_reward_item` | [daily_reward_item](daily_reward_item.md) | `0011-game-economy` |
| `portal.daily_reward_claim` | [daily_reward_claim](daily_reward_claim.md) | `0011-game-economy` |
| `portal.kingdom_cape_price` | [kingdom_cape_price](kingdom_cape_price.md) | `0011-game-economy` |
| `portal.donate_credit` | [donate_credit](donate_credit.md) | `0011-game-economy` |
| `portal.donate_debit` | [donate_debit](donate_debit.md) | `0011-game-economy` |

## Lote 3 — mundo do conteúdo (`0019`), 13 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `content.content_source` | [content_source](content_source.md) | `0019-content-world` |
| `content.npc_template` | [npc_template](npc_template.md) | `0019-content-world` |
| `content.npc_definition` | [npc_definition](npc_definition.md) | `0019-content-world` |
| `content.npc_shop_item` | [npc_shop_item](npc_shop_item.md) | `0019-content-world` |
| `content.level_item` | [level_item](level_item.md) | `0019-content-world` |
| `content.guard_spawn` | [guard_spawn](guard_spawn.md) | `0019-content-world` |
| `content.guard_spawn_reward` | [guard_spawn_reward](guard_spawn_reward.md) | `0019-content-world` |
| `content.mob_merc` | [mob_merc](mob_merc.md) | `0019-content-world` |
| `content.game_setting` | [game_setting](game_setting.md) | `0019-content-world` |
| `content.game_matrix` | [game_matrix](game_matrix.md) | `0019-content-world` |
| `content.region` | [region](region.md) | `0019-content-world` |
| `content.world_item_seed` | [world_item_seed](world_item_seed.md) | `0019-content-world` |
| `content.guild_zone` | [guild_zone](guild_zone.md) | `0019-content-world` |

As 11 páginas restantes fecharam o arquivo em 2026-11-22; a forma das 13 está em
`0019-content-world.md`.

## Lote 4 — entrega assíncrona (`0012`), 1 tabela

| Tabela | Página | Migration |
|---|---|---|
| `ops.delivery_queue` | [delivery_queue](delivery_queue.md) | `0012-ops-delivery` |

## Lote 5 — catálogo de item (`0014`), 1 tabela

| Tabela | Página | Migration |
|---|---|---|
| `content.item_catalog` | [item_catalog](item_catalog.md) | `0014-content-item` |

## Lote 6 — mapas (`0024`), 2 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `content.map_definition` | [map_definition](map_definition.md) | `0024-content-maps` |
| `content.teleport_route` | [teleport_route](teleport_route.md) | `0024-content-maps` |

## Lote 7 — taxonomias (`0013`), 5 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `content.game_system` | [game_system](game_system.md) | `0013-content-taxonomy` |
| `content.game_class` | [game_class](game_class.md) | `0013-content-taxonomy` |
| `content.skill_lineage` | [skill_lineage](skill_lineage.md) | `0013-content-taxonomy` |
| `content.item_kind` | [item_kind](item_kind.md) | `0013-content-taxonomy` |
| `content.item_effect` | [item_effect](item_effect.md) | `0013-content-taxonomy` |

As cinco fecharam em 2026-11-22; a forma delas está em `migrations/0013-content-taxonomy.md`, e a
função `content.fn_fold` do mesmo arquivo não tem página própria (não é tabela).

## Próximos lotes

Faltam **31 das 97**: `0015` (catálogo de skill) — os seeds `0016`–`0018` não
criam tabela —, `0021` e `0022` (regras e sobrescritas), `0025` e `0026` (combate e quests/eventos) e
`0028`–`0031` (estado de jogo, views e os dois adendos do portal). Cada lote novo é
proposto por arquivo de migration, como o de `0019`.
