# wyd-db — referência por tabela

**Estado:** **44 de 97** páginas escritas (2026-11-21). A produção é manual, em lotes (`M26`), sem
gerador: cada página resume a tabela e aponta para a página de `migrations/` dona da forma. Este índice
lista as páginas de `plano-migrations.md` §3 conforme elas existem.

**Páginas de migration não publicadas (`V`, 2026-11-21):** as 25 páginas escritas (de 31 em
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

## Próximos lotes

`0012`–`0018` (entrega, taxonomia, catálogo e os três seeds), `0019`–`0027` (mundo, regras, mapas,
combate, quests e eventos) e `0028`–`0031` (estado de jogo, views e os dois adendos do portal).
