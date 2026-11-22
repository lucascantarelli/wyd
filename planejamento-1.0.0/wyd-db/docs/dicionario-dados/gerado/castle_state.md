> Pergunta: "o que `game.castle_state` guarda do castelo e por que a cidade não pode ser apagada com estado vivo?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.28; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Estado corrente do castelo: clã dono, cidade associada, ouro no cofre e dois instantes — última coleta e
última guerra. É **linha única** de estado de jogo, ao lado de `game.kefra_state`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | — | linha única; `CHECK (id = 1)` |
| `owner_guild_id` | `BIGINT` | sim | — | clã dono; `NULL` se apagado |
| `city_code` | `TEXT` | sim | — | cidade associada; FK `RESTRICT` |
| `vault_gold` | `BIGINT` | não | `0` | ouro no cofre, `>= 0` |
| `last_collected_at` | `TIMESTAMPTZ` | sim | — | última coleta |
| `last_war_at` | `TIMESTAMPTZ` | sim | — | última guerra disputada |

Sem identidade e sem carimbos de alteração: a linha é única e os dois instantes registram os fatos que
importam.

## Chave primária

`pk_castle_state (id)` com `ck_castle_state_linha_unica CHECK (id = 1)` — mesmo desenho de
`game.kefra_state`: uma linha, recusada pelo `CHECK` se o `id` não for 1.

## FKs de saída e de entrada

- **Saída, `SET NULL` (1 FK):** `fk_castle_state_guild`: `owner_guild_id → game.guild (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE SET NULL` — apagar o clã mantém o castelo e apaga o dono.
- **Saída, `RESTRICT` (1 FK):** `fk_castle_state_cidade`: `city_code → game.city (code)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — cidade referenciada pelo castelo não é apagada em
  silêncio.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o estado do castelo.

## Unicidade

A PK mais o `CHECK (id = 1)` fecham a singularidade. `city_code` não tem único próprio: com uma linha só,
repetição de cidade é impossível por construção, não por índice.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_castle_state_linha_unica` | `id` diferente de 1 |
| `ck_castle_state_cofre` | `vault_gold` negativo |

Não há `CHECK` que amarre `owner_guild_id` a `city_code`: castelo sem dono e com cidade é estado válido.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_castle_state_guild (owner_guild_id) WHERE owner_guild_id IS NOT NULL` | lado filho da FK nullable (`P-15`); sustenta o `SET NULL` |
| `ix_castle_state_cidade (city_code) WHERE city_code IS NOT NULL` | lado filho da FK nullable (`P-15`); sustenta o `RESTRICT` da cidade |

## Armadilha de leitura

`vault_gold` é **saldo**, não extrato: o cofre não tem ledger como `game.account_wallet`/`account_ledger`
(`0005`), e não há como reconstruir de onde veio o ouro nem quando ele saiu. `owner_guild_id` nulo pode
significar castelo sem dono ou clã apagado; `last_war_at` distingue os casos no uso, não no schema.
`city_code` nulo é castelo sem cidade associada, não cidade "zero". E `last_collected_at` não é
"última vez que o cofre mudou" — a coleta é um dos eventos, e depósito não tem carimbo.

## Papéis

- **Escreve:** `wyd_game` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `game`
  (`0001-bootstrap.md` §7). Nenhum seed cria a linha de estado.
- **Leem:** `wyd_game`; o portal não tem `USAGE` em `game` e depende da view de `0029` (sem permissão
  concedida, decisão de 2026-11-21). `wyd_admin` não recebe esta tabela em `0032`.

## Migration

Forma (dona): `migrations/0028-game-quests-events.md` — arquivo `0028_game_quests_events.sql`.
