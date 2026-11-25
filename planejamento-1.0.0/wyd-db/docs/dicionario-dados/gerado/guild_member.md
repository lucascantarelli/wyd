> Pergunta: "o que `game.guild_member` guarda e como o banco garante um líder e uma guilda por personagem?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.13; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Vínculo entre personagem e guilda, com papel (`leader`, `sub_leader`, `member`) e data de entrada. Sair
ou ser expulso apaga a linha; não há histórico de vínculo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `guild_id` | `BIGINT` | não | — | PK e FK para `guild` |
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `role` | `TEXT` | não | `'member'` | `leader`, `sub_leader` ou `member` |
| `joined_at` | `TIMESTAMPTZ` | não | `now()` | entrada no grupo |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_guild_member (guild_id, character_id)` — o par é o vínculo; o mesmo personagem não entra duas vezes
na mesma guilda.

## FKs de saída e de entrada

- **Saída:** `fk_guild_member_guild`: `guild_id → game.guild (id)`, `CASCADE`/`NO ACTION`;
  `fk_guild_member_character`: `character_id → game.character (id)`, `CASCADE`/`NO ACTION`.
- **Entrada:** nenhuma.

## Unicidade

- `ux_guild_member_personagem (character_id)` — único **total**: um personagem pertence a uma guilda só;
- `ux_guild_member_lider (guild_id) WHERE role = 'leader'` — único **parcial**: no máximo um líder por
  guilda, inclusive sob concorrência.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_guild_member_papel` | papel fora dos três valores |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_guild_member_personagem (character_id)` | guilda do personagem (e regra de vínculo único) |
| `ux_guild_member_lider (guild_id) WHERE role = 'leader'` | garantir um líder por guilda |

Trigger `trg_guild_member_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0010-game-guild.md` — arquivo `0010-game-guild.sql`.
