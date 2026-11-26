> Pergunta: "o que `game.character` guarda do personagem e como o soft-delete libera nome e vaga?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.10; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Personagem do jogador: nome, classe, progressão e estado bruto do `STRUCT_MOB`, com soft-delete (`A14`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `account_id` | `BIGINT` | não | — | conta dona; `CASCADE` |
| `slot` | `SMALLINT` | não | — | vaga 0–3 |
| `name` | `TEXT` | não | — | 1–16 caracteres |
| `name_fold` | `TEXT` gerada | sim | `lower(btrim(name))` | `STORED`, sem `NOT NULL`; nunca nula porque `name` não é |
| `class` | `SMALLINT` | não | `0` | 0–5 |
| `tier` | `TEXT` | não | `'mortal'` | seis camadas de progressão |
| `level` | `INTEGER` | não | `1` | 1–400 |
| `exp` | `BIGINT` | não | `0` | experiência |
| `coin` | `BIGINT` | não | `0` | ouro do personagem, não a carteira |
| `magic` | `INTEGER` | não | `0` | poder mágico bruto |
| `learned_skill_mask` | `BIGINT` | não | `0` | máscara de 32 bits |
| `score_bonus` | `INTEGER` | não | `0` | pontos livres |
| `special_bonus` | `INTEGER` | não | `0` | pontos livres |
| `skill_bonus` | `INTEGER` | não | `0` | pontos livres |
| `critical` | `SMALLINT` | não | `0` | 0–255 |
| `save_mana` | `SMALLINT` | não | `0` | 0–255 |
| `recall_x` | `SMALLINT` | sim | — | posição de retorno (`SPX`/`SPY`), nula até a primeira gravação |
| `recall_y` | `SMALLINT` | sim | — | idem |
| `content_version` | `SMALLINT` | não | — | molde de conteúdo que criou o personagem (`D-39`) |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |
| `deleted_at` | `TIMESTAMPTZ` | sim | — | soft-delete (`A14`); a linha fica |

## Chave primária

`pk_character (id)` — o personagem é entidade; nome e vaga são únicos entre vivos, não chaves.

## FKs de saída e de entrada

- **Saída:** `fk_character_account`: `account_id → game.account (id)`, `CASCADE`;
  `fk_character_content_version`: `content_version → content.content_version (id)`, sem `ON DELETE`
  explícito.
- **Entrada, `CASCADE`:** `character_status`, `character_scores`, `character_special`,
  `character_short_skill`, `character_skill`, `character_learned_skill`, `character_lineage`,
  `character_affect`, `character_chaos`, `character_item`, `guild_member`, `personal_shop`,
  `character_quest` e `arena_score`.
- **Entrada, `SET NULL`:** `city_tax_ledger` e `event_participation`.
- **Entrada, `RESTRICT`:** `game.session`, `ops.delivery_queue` e `game.market_escrow`.

## Unicidade

- `ux_character_nome (name_fold) WHERE deleted_at IS NULL` — nome único entre vivos; apagado libera;
- `ux_character_vaga (account_id, slot) WHERE deleted_at IS NULL` — vaga única entre vivos.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_vaga` | vaga fora de 0–3 |
| `ck_character_nome` | nome vazio ou acima de 16 |
| `ck_character_classe` | classe fora de 0–5 |
| `ck_character_tier` | camada fora das seis |
| `ck_character_nivel` | nível fora de 1–400 |
| `ck_character_exp` | experiência negativa |
| `ck_character_moeda` | ouro negativo |
| `ck_character_magia` | magia negativa |
| `ck_character_mascara` | máscara fora de 0–`4294967295` |
| `ck_character_bonus` | qualquer bônus negativo |
| `ck_character_critico` | crítico fora de 0–255 |
| `ck_character_save_mana` | `save_mana` fora de 0–255 |
| `ck_character_exclusao` | exclusão anterior à criação |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_character_nome` (parcial) | nome livre para novo personagem |
| `ux_character_vaga` (parcial) | vaga do personagem |
| `ix_character_account (account_id)` | personagens da conta |
| `ix_character_content_version (content_version)` | lado filho da FK de conteúdo |

Trigger `trg_character_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`; o portal lê por view de `0029` (sem `USAGE` em `game`).
- **Administra:** `wyd_admin` — só `SELECT`, concedido por `0032` para conferir o titular na custódia
  de item.

## Migration

Forma (dona): `migrations/0006_game_character.md` — arquivo `0006_game_character.sql`.
