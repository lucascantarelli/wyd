> Pergunta: "o que `game.kefra_state` guarda na linha única e por que o clã pode virar nulo?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.28; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Estado corrente do Kefra: última morte, próximo nascimento, contador de mortes e o clã que matou por
último. É **linha única** de estado de jogo — não é catálogo de aparições e não guarda histórico.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | — | linha única; `CHECK (id = 1)` |
| `last_kill_at` | `TIMESTAMPTZ` | sim | — | última morte do Kefra |
| `next_spawn_at` | `TIMESTAMPTZ` | sim | — | próximo nascimento |
| `kill_count` | `INTEGER` | não | `0` | total de mortes, `>= 0` |
| `last_guild_id` | `BIGINT` | sim | — | clã da última morte; `NULL` se apagado |

Sem identidade e sem carimbos: a linha é única e o instante de cada fato já está nas duas colunas de
tempo. `last_kill_at` e `next_spawn_at` podem ser nulos ao mesmo tempo — o schema não exige que o
primeiro preenchimento traga o segundo.

## Chave primária

`pk_kefra_state (id)` — PK de uma linha só, com `ck_kefra_state_linha_unica CHECK (id = 1)`. É o mesmo
desenho de `content.content_version` (`0001`): sem `id` gerado, para o banco recusar a segunda linha em
vez de deixar o runtime escolher qual vale.

## FKs de saída e de entrada

- **Saída, `SET NULL` (1 FK):** `fk_kefra_state_guild`: `last_guild_id → game.guild (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE SET NULL` — apagar o clã preserva o estado e apaga só a atribuição.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o estado do Kefra.

## Unicidade

A PK mais o `CHECK (id = 1)` fecham a singularidade: não existe segundo registro, e `id = 2` é recusado
pelo `CHECK`, não pela PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_kefra_state_linha_unica` | `id` diferente de 1 |
| `ck_kefra_state_kill_count` | `kill_count` negativo |

`last_kill_at` e `next_spawn_at` não têm `CHECK` de ordem: o schema aceita `next_spawn_at` anterior a
`last_kill_at`, e essa coerência é do runtime.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_kefra_state_guild (last_guild_id) WHERE last_guild_id IS NOT NULL` | lado filho da FK nullable (`P-15`); sustenta o `SET NULL` |

## Armadilha de leitura

`id = 1` é valor mágico **documentado**, não identidade de negócio: quem lê o estado não filtra por `id`
de jogador. `last_guild_id` nulo pode significar "Kefra nunca morreu" ou "o clã que matou foi apagado" —
as duas leituras convivem, e `last_kill_at` é o que distingue. `kill_count` é contador acumulado, não "mortes
nesta semana": não há janela nem histórico, e reset de contador não é registrado. E `next_spawn_at` não é
calculado pelo banco.

## Papéis

- **Escreve:** `wyd_game` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `game`
  (`0001-bootstrap.md` §7). Nenhum seed cria a linha de estado.
- **Leem:** `wyd_game`; o portal não tem `USAGE` em `game` e depende da view de `0029` (sem permissão
  concedida, decisão de 2026-11-21). `wyd_admin` não recebe esta tabela em `0032`.

## Migration

Forma (dona): `migrations/0028-game-quests-events.md` — arquivo `0028-game-quests-events.sql`.
