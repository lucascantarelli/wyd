> Pergunta: "o que `content.guild_zone` guarda da zona de guilda e por que o estado de guerra não está aqui?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Definição da zona de guilda por par `(clã do cliente, zona)`: condição de vitória e imposto da cidade.
É **só definição** — `charge_guild`, `challenge_guild`, `challenge_money` e `tax_vault` são estado
mutável de guerra e ficam no lote de `game` (`0028`): conteúdo muda por deploy, não por disputa.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `clan` | `SMALLINT` | não | — | código de clã do cliente, não FK para `game.guild` |
| `zone` | `SMALLINT` | não | — | identificador da zona |
| `victory` | `SMALLINT` | não | `0` | condição de vitória da disputa |
| `city_tax` | `SMALLINT` | não | `0` | imposto da cidade associada |
| `enabled` | `BOOLEAN` | não | `true` | aposentadoria sem `DELETE` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbos; `updated_at` pelo trigger |

## Chave primária

`pk_guild_zone (clan, zone)` — o par é a chave: uma definição por clã e zona.

## FKs de saída e de entrada

- **Saída:** nenhuma. `clan` é o **código de clã do cliente**, não FK para `game.guild`: guilda é
  estado de jogador e conteúdo não aponta para `game` (`plano-migrations.md` §3.1).
- **Entrada:** nenhuma — inclusive `castle_state` (`0028`), que guarda o estado de guerra (cofre,
  `vault_gold`, e `last_war_at`) sem FK para cá.

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_guild_zone_clan` | `clan` negativo |
| `ck_guild_zone_zona` | `zone` negativo |
| `ck_guild_zone_vitoria` | `victory` negativo |
| `ck_guild_zone_imposto` | `city_tax` negativo |

Nenhum `CHECK` relaciona `city_tax` a `game.city`: a ligação é por código no runtime, e não há
constraint que prove que o imposto pertence a uma cidade existente.

## Índices

Nenhum além da PK — a leitura é sempre pelo par. Trigger `trg_guild_zone_set_updated_at`.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019-content-world.md` §8.

## Migration

Forma (dona): `migrations/0019-content-world.md` — arquivo `0019_content_world.sql`.
