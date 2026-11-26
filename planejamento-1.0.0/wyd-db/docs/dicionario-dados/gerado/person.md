> Pergunta: "o que `portal.person` guarda, o que o schema garante sobre status e por que o e-mail fechado é anonimizado?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.1; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Identidade da pessoa no portal: e-mail, senha, papel e estado — a raiz do grafo de identidade.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `email` | `TEXT` | não | — | até 254 caracteres |
| `email_fold` | `TEXT` gerada | sim | `lower(btrim(email))` | `STORED`, sem `NOT NULL`; nunca nula porque `email` não é |
| `password_hash` | `TEXT` | não | — | só `$argon2id$…` |
| `role` | `TEXT` | não | `'member'` | papel **do portal**, não de jogo |
| `status` | `TEXT` | não | `'active'` | `pending`, `active`, `suspended`, `closed` |
| `email_verified_at` | `TIMESTAMPTZ` | sim | — | confirmação do endereço |
| `failed_login_count` | `SMALLINT` | não | `0` | contador de falhas |
| `locked_until` | `TIMESTAMPTZ` | sim | — | bloqueio temporário por falha |
| `password_changed_at` | `TIMESTAMPTZ` | sim | — | nulo = nunca trocada (`M4`) |
| `last_login_at` | `TIMESTAMPTZ` | sim | — | último login |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |
| `closed_at` | `TIMESTAMPTZ` | sim | — | fechamento da pessoa |
| `suspended_reason` | `TEXT` | sim | — | motivo, exigido na suspensão (`B4`) |
| `suspended_by` | `BIGINT` | sim | — | autor, exigido na suspensão |
| `suspended_at` | `TIMESTAMPTZ` | sim | — | instante da suspensão |

## Chave primária

`pk_person (id)` — identidade interna; o e-mail não é PK porque muda e é dobrado.

## FKs de saída e de entrada

- **Saída:** `fk_person_suspended_by`: `suspended_by → portal.person (id)`, `RESTRICT` — suspensa
  exige autor, e pessoa não é apagada.
- **Entrada:** `person_profile`, `person_consent`, `person_session`, `person_token` e
  `person_authenticator` (`CASCADE`); `game.account` e `game.account_moderation.blocked_by`
  (`RESTRICT`); `ops.delivery_queue` (`RESTRICT`); `news` (autor e quem publicou), `banner`, `guide`
  (autor e revisor) e `launcher_release` (`RESTRICT`).

## Unicidade

`ux_person_email (email_fold)` — **total**, sem filtro de status e sem janela de reuso (`A12`): o
endereço de uma pessoa fechada continua ocupado pelo tombstone.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_person_email_len` | e-mail acima de 254 caracteres |
| `ck_person_email_forma` | endereço fora do formato `local@dominio.tld` |
| `ck_person_role` | papel fora de `member`/`moderator`/`admin` |
| `ck_person_status` | estado fora dos quatro |
| `ck_person_senha_hasheada` | hash que não começa com `$argon2id$` |
| `ck_person_falhas` | contador negativo |
| `ck_person_senha_trocada` | troca anterior à criação |
| `ck_person_fechamento_coerente` | `closed` sem `closed_at`, ou o inverso |
| `ck_person_fechamento_anonimo` | pessoa fechada com e-mail diferente de `closed-<id>@removido.invalid` |
| `ck_person_suspensao_coerente` | suspensão sem carimbo, ou carimbo fora de suspensão |
| `ck_person_suspensao_motivo` | suspensão sem motivo não vazio ou sem autor |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_person_email (email_fold)` | login e cadastro por e-mail |
| `ix_person_nao_ativas (status) WHERE status <> 'active'` | fila de moderação/limpeza |
| `ix_person_suspended_by (suspended_by)` | lado filho da FK |

Trigger `trg_person_set_updated_at` mantém `updated_at`.

## Papéis

- **Escreve:** `wyd_portal` (CRUD por default privilege de `portal`).
- **Lê:** `wyd_portal`; o jogo lê identidade só pela view `portal.v_person_identity` (`0029`).
- **Não lê:** `wyd_read` tem `USAGE` em `portal`, mas não `SELECT` por default.

## Migration

Forma (dona): `migrations/0003_portal_person.md` — arquivo `0003_portal_person.sql`.
