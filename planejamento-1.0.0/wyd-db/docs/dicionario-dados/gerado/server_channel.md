> Pergunta: "o que `game.server_channel` guarda e por que a série não semeia nenhuma linha?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.5; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Topologia de canais por instalação: grupo, canal, host e operação (`max_players`, `maintenance`, `role`)
— uma linha por canal, fundida do legado (`M23`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `group_id` | `SMALLINT` | não | — | grupo/instalação, 0–9; parte da PK |
| `channel` | `SMALLINT` | não | — | 0 = linha de status; 1–10 = jogável |
| `host` | `TEXT` | não | — | não vazio |
| `status_url` | `TEXT` | não | `''` | até 32 caracteres |
| `enabled` | `BOOLEAN` | não | `true` | canal habilitado |
| `max_players` | `INTEGER` | sim | — | capacidade; nulo só em `channel = 0` |
| `maintenance` | `BOOLEAN` | sim | — | nulo só em `channel = 0` |
| `role` | `TEXT` | sim | — | `normal`/`verde`; nulo só em `channel = 0` |
| `note` | `TEXT` | não | `''` | anotação de operação |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_server_channel (group_id, channel)` — a topologia é identificada pelo par; `channel = 0` é a linha
de status da instalação.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** `game.session (group_id, channel) → game.server_channel`, `RESTRICT` — sessão só existe
  em canal cadastrado.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_server_channel_grupo` | grupo fora de 0–9 |
| `ck_server_channel_canal` | canal fora de 0–10 |
| `ck_server_channel_host` | host vazio |
| `ck_server_channel_status_url` | URL de status acima de 32 caracteres |
| `ck_server_channel_jogavel` | `channel = 0` com os três campos de operação, ou canal jogável sem os três |
| `ck_server_channel_max` | capacidade negativa |
| `ck_server_channel_role` | papel fora de `normal`/`verde` |

## Índices

Nenhum além da PK: a leitura é sempre por `(group_id, channel)`.

## Papéis

- **Escreve:** `wyd_game`; na prática, **operação**, não a série — `0007` não semeia canal.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal`.

`host` e `status_url` são dado de instalação: cada ambiente tem os seus, e migration com IP/host fixo
seria mentira sobre o segundo ambiente.

## Migration

Forma (dona): `migrations/0007-game-session.md` — arquivo `0007_game_session.sql`.
