> Pergunta: "o que `game.session` guarda de uma conexão de jogo e como o schema impede duas sessões vivas da mesma conta?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.5; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Sessão de jogo: conta, personagem (depois da seleção), canal e ciclo de vida com motivo de liberação.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `account_id` | `BIGINT` | não | — | conta conectada |
| `character_id` | `BIGINT` | sim | — | nulo até a seleção de personagem |
| `group_id` | `SMALLINT` | não | — | grupo do canal |
| `channel` | `SMALLINT` | não | — | 1–10 |
| `client_version` | `INTEGER` | não | — | inteiro do handshake, não semver |
| `ip` | `INET` | sim | — | auditoria de multibox (`B5`) |
| `opened_at` | `TIMESTAMPTZ` | não | `now()` | abertura |
| `renewed_at` | `TIMESTAMPTZ` | não | `now()` | último sinal de vida |
| `released_at` | `TIMESTAMPTZ` | sim | — | liberação; nulo = viva |
| `released_reason` | `TEXT` | sim | — | motivo operacional (`A14`) |

## Chave primária

`pk_session (id)` — cada conexão é uma linha; liberar não apaga.

## FKs de saída e de entrada

- **Saída:** `fk_session_account`: `account_id → game.account (id)`, `RESTRICT`;
  `fk_session_character`: `character_id → game.character (id)`, `RESTRICT`;
  `fk_session_canal`: `(group_id, channel) → game.server_channel (group_id, channel)`, `RESTRICT`.
- **Entrada:** nenhuma.

## Unicidade

- `ux_session_viva_por_conta (account_id) WHERE released_at IS NULL` — a segunda conexão da mesma
  conta falha aqui, nunca só no código;
- `ux_session_viva_por_personagem (character_id) WHERE released_at IS NULL AND character_id IS NOT NULL`
  — uma sessão viva por personagem, quando já houver seleção.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_session_canal` | canal fora de 1–10 |
| `ck_session_client_version` | versão do cliente não positiva |
| `ck_session_sinal` | renovação anterior à abertura |
| `ck_session_liberacao` | liberação anterior à abertura |
| `ck_session_motivo` | liberada sem motivo, viva com motivo, ou motivo fora dos quatro |

Motivos aceitos: `disconnect`, `reaper`, `character_deleted`, `admin`.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_session_viva_por_conta` (parcial) | unicidade da sessão viva |
| `ux_session_viva_por_personagem` (parcial) | unicidade por personagem |
| `ix_session_canal (group_id, channel, opened_at DESC)` | ocupação por canal |
| `ix_session_reaper (renewed_at) WHERE released_at IS NULL` | reaper de 60 s / stale 180 s |
| `ix_session_ip (ip, opened_at DESC) WHERE ip IS NOT NULL` | multibox por IP/hora |

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`; o portal lê presença por `game.v_live_session` (`0029`).
- **Não lê:** `wyd_portal` direto (sem `USAGE` em `game`).
- **Administra:** `wyd_admin` — `SELECT, UPDATE` concedidos por `0032`; sem `INSERT`/`DELETE`.

Retenção: 365 dias após `released_at` (`M22`).

## Migration

Forma (dona): `migrations/0007-game-session.md` — arquivo `0007_game_session.sql`.
