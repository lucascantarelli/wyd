> Pergunta: "o que `game.account_moderation` guarda e por que bloqueio disciplinar não é coluna de `account`?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.2; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Estado de bloqueio disciplinar da conta, com motivo, autor e instante — separado do ciclo de vida.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | PK e FK para `account` |
| `blocked` | `BOOLEAN` | não | `false` | estado do bloqueio |
| `reason` | `TEXT` | não | `''` | exigido quando bloqueada |
| `blocked_by` | `BIGINT` | sim | — | pessoa autora; FK `RESTRICT` |
| `blocked_at` | `TIMESTAMPTZ` | sim | — | carimbo do bloqueio |

## Chave primária

`pk_account_moderation (account_id)` — uma linha por conta, 1:1.

## FKs de saída e de entrada

- **Saída:** `fk_account_moderation_account`: `account_id → game.account (id)`, `RESTRICT`;
  `fk_account_moderation_autor`: `blocked_by → portal.person (id)`, `RESTRICT`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_account_moderation_motivo` | bloqueada com motivo vazio |
| `ck_account_moderation_instante` | `blocked` e `blocked_at` em desacordo |
| `ck_account_moderation_autor` | bloqueada sem autor |

Desbloquear força `blocked_at` nulo; `reason` e `blocked_by` podem permanecer — ver `H` na página de
[moderação](../dominio-moderacao.md).

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_account_moderation_autor (blocked_by)` | lado filho da FK |
| `ix_account_moderation_bloqueadas (account_id) WHERE blocked` | contas bloqueadas |

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (o portal chega ao bloqueio por API, não por SQL).
- **Administra:** `wyd_admin` — `SELECT, INSERT, UPDATE` concedidos por `0032`; `DELETE` fica fora.

Dono do assunto no dicionário: [moderação](../dominio-moderacao.md).

## Migration

Forma (dona): `migrations/0005_game_account.md` — arquivo `0005_game_account.sql`.
