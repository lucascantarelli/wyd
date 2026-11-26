> Pergunta: "o que `portal.person_authenticator` guarda do segundo fator e por que o segredo é cifrado, não hash?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.20; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Segundo fator TOTP do staff (`admin`/`moderator`), em credencial separada de `portal.person`. Guarda o
segredo **cifrado**, o estado de confirmação, o último passo aceito e a revogação. O jogo não lê esta
tabela, e `member` fica fora da `1.0.0` (`A20`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `person_id` | `BIGINT` | não | — | pessoa dona; FK `CASCADE` |
| `kind` | `TEXT` | não | `'totp'` | mecanismo; só `totp` na `1.0.0` |
| `secret_encrypted` | `BYTEA` | sim | — | segredo cifrado, 32–512 bytes; `NULL` só em revogado |
| `confirmed_at` | `TIMESTAMPTZ` | sim | — | `NULL` = inscrição pendente |
| `last_used_step` | `BIGINT` | sim | — | último passo de 30 s aceito; só depois de confirmado |
| `revoked_at` | `TIMESTAMPTZ` | sim | — | revogação; não anterior à criação nem à confirmação |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

`secret_encrypted` é a **única coluna de segredo reversível da série**: o BFF precisa decifrar o segredo
para calcular o código, e por isso `hash` não serve (`padrao-banco.md` §3/§4). A chave de cifra mora no
ambiente do BFF, nunca no banco nem na documentação.

## Chave primária

`pk_person_authenticator (id)` — identidade sintética, porque revogar **não** apaga a linha: o histórico
"este fator existiu e foi revogado em tal data" é o que responde a incidente de staff.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_person_authenticator_person`: `person_id → portal.person (id)`,
  `ON DELETE CASCADE`, `ON UPDATE NO ACTION` — linha possuída (`P-15`). Como fechar pessoa é soft-delete
  com anonimização (`A12`), o `CASCADE` só age em expurgo manual.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o autenticador.

## Unicidade

`ux_person_authenticator_ativo UNIQUE (person_id, kind) WHERE revoked_at IS NULL` — **um** autenticador
ativo por pessoa e tipo. É parcial: linha revogada não ocupa a vaga, então revogar libera o próximo
cadastro, e o histórico de revogados fica na tabela.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_person_authenticator_kind` | `kind` fora de `totp` |
| `ck_person_authenticator_segredo` | ativo sem segredo, ou revogado com segredo |
| `ck_person_authenticator_segredo_tamanho` | `secret_encrypted` fora de 32–512 bytes |
| `ck_person_authenticator_confirmacao` | `confirmed_at` anterior a `created_at` |
| `ck_person_authenticator_passo` | `last_used_step` negativo, ou preenchido sem confirmação |
| `ck_person_authenticator_revogacao` | `revoked_at` anterior a `created_at` |
| `ck_person_authenticator_revogacao_confirmada` | `revoked_at` anterior a `confirmed_at` |

`ck_person_authenticator_segredo` amarra as duas pontas: ativo **tem** segredo e revogado **não guarda**
segredo morto. Revogar sem zerar `secret_encrypted` falha em `23514`.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_person_authenticator_ativo (person_id, kind) WHERE revoked_at IS NULL` | garante um ativo por pessoa e tipo e faz a busca do fator no login |
| `ix_person_authenticator_person (person_id)` | lado filho da FK (`P-15`); achar as linhas ao fechar/apagar pessoa |

`trg_person_authenticator_set_updated_at` mantém `updated_at` no `UPDATE`, via `ops.fn_set_updated_at()`
(`0001`).

## Armadilha de leitura

`confirmed_at` nulo é inscrição **pendente**, e o fator não vale no login até ser confirmado — metadado
lido pela tela de segurança não é prova de que o segundo fator está ativo. `secret_encrypted` nunca é lido
para exibição: a tela `/admin/seguranca` mostra só `kind`, `confirmed_at`, `last_used_step` e
`revoked_at`, e o valor decifrado existe apenas na memória do BFF durante a validação. Revogar é `UPDATE`
com `revoked_at` e `secret_encrypted = NULL`, **nunca** `DELETE`. E `kind` só aceita `totp`: WebAuthn seria
decisão nova, não linha nova de `CHECK`.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`
  (`0001_bootstrap.md` §7), pelo BFF do portal. Nenhum seed cria fator.
- **Não leem:** `wyd_read` tem `USAGE` em `portal` mas não recebe `SELECT` por default; `wyd_game` não
  alcança o schema; `wyd_admin` não lê `portal` e não participa do segundo fator.

## Migration

Forma (dona): `migrations/0031_portal_mfa.md` — arquivo `0031_portal_mfa.sql`; DDL em
`ddl-confirmado.md` §5.20.
