# wyd-db — referência por tabela

**Estado:** **85 de 109** páginas escritas (2026-11-22). A produção é manual, em lotes (`M26`), sem
gerador: cada página resume a tabela e aponta para a página de `migrations/` dona da forma. Este índice
lista as páginas de `plano-migrations.md` §3 conforme elas existem.

**Páginas de migration não publicadas (`V`, 2026-11-22):** as 28 páginas escritas (de 34 em
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

## Lote 8 — catálogo de skill e regras de refino (`0015` + `0021`), 5 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `content.skill_catalog` | [skill_catalog](skill_catalog.md) | `0015-content-skill` |
| `content.refine_rate` | [refine_rate](refine_rate.md) | `0021-content-rules` |
| `content.refine_gem_effect` | [refine_gem_effect](refine_gem_effect.md) | `0021-content-rules` |
| `content.sanc_rate` | [sanc_rate](sanc_rate.md) | `0021-content-rules` |
| `content.combine_value` | [combine_value](combine_value.md) | `0021-content-rules` |

O `0015` cria uma tabela só; o lote completou as cinco com quatro das cinco do `0021`, e a quinta
(`accessory_refine_rule`) abre o lote 9.

## Lote 9 — acessório e overrides (`0021` + `0022`), 4 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `content.accessory_refine_rule` | [accessory_refine_rule](accessory_refine_rule.md) | `0021-content-rules` |
| `content.refine_rate_override` | [refine_rate_override](refine_rate_override.md) | `0022-content-overrides` |
| `content.sanc_rate_override` | [sanc_rate_override](sanc_rate_override.md) | `0022-content-overrides` |
| `content.combine_value_override` | [combine_value_override](combine_value_override.md) | `0022-content-overrides` |

O `0021` fecha com a quinta tabela; as três de `0022` são a camada de edição do staff, com `GRANT` de
escrita para `wyd_portal` e reset por `DELETE`.

## Lote 10 — combate (`0025`), 4 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `content.mob_template_stat` | [mob_template_stat](mob_template_stat.md) | `0025-content-combat` |
| `content.mob_template_equip` | [mob_template_equip](mob_template_equip.md) | `0025-content-combat` |
| `content.drop_rate_override` | [drop_rate_override](drop_rate_override.md) | `0025-content-combat` |
| `content.content_rate_override` | [content_rate_override](content_rate_override.md) | `0025-content-combat` |

As duas primeiras são filhas do template, com `CASCADE`; as duas de regra usam item com `RESTRICT` e
família de taxa fechada em `kind`, com `rate_bps` até `1.000.000`.

## Lote 11 — quests e eventos (`0026`), 6 tabelas

| Tabela | Página | Migration |
|---|---|---|
| `content.quest` | [quest](quest.md) | `0026-content-quests-events` |
| `content.quest_step` | [quest_step](quest_step.md) | `0026-content-quests-events` |
| `content.quest_reward` | [quest_reward](quest_reward.md) | `0026-content-quests-events` |
| `content.world_event` | [world_event](world_event.md) | `0026-content-quests-events` |
| `content.event_schedule` | [event_schedule](event_schedule.md) | `0026-content-quests-events` |
| `content.arena` | [arena](arena.md) | `0026-content-quests-events` |

Quest, passo e recompensa formam uma cadeia com `CASCADE` no filho; evento e agenda repetem a relação com
horário em minuto do dia; a arena é definição fechada, sem partida — o estado dela é de `0028`.

## Próximos lotes

Faltam **24 das 109**, por arquivo: `0028` (7 de estado de jogo), `0030` (4 do CMS), `0031` (1,
`person_authenticator`), `0033` (1, `idempotency_record`) e `0034` (11 do mercado) —
7 + 4 + 1 + 1 + 11 = 24. Os seeds `0016`–`0018`, `0020`, `0023` e `0027` não criam tabela e não entram.
Cada lote novo é proposto por arquivo de migration, como o de `0019`.
