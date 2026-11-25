> Pergunta: "o que `portal.launcher_release` guarda por canal e por que só uma versão pode ficar publicada?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.15; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Versão do launcher por canal: número, URL e hash do manifesto, versão mínima aceita, notas, estado e
publicador. É o ponteiro que o launcher consulta para saber o que baixar — e não uma cópia do binário ou
do manifesto.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `channel` | `TEXT` | não | `'stable'` | `stable` ou `beta` |
| `version` | `TEXT` | não | — | semver `X.Y.Z[-sufixo]` |
| `manifest_url` | `TEXT` | não | — | só `https://` |
| `manifest_sha256` | `CHAR(64)` | não | — | 64 hex minúsculos |
| `min_version` | `TEXT` | sim | — | versão mínima aceita, mesmo formato |
| `notes_md` | `TEXT` | não | `''` | notas, até 20.000 caracteres |
| `status` | `TEXT` | não | `'draft'` | `draft`, `published` ou `revoked` |
| `released_at` | `TIMESTAMPTZ` | sim | — | obrigatório fora de `draft` |
| `revoked_at` | `TIMESTAMPTZ` | sim | — | obrigatório **só** em `revoked` |
| `published_by_person_id` | `BIGINT` | sim | — | publicador; FK `RESTRICT` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbado pelo trigger |

## Chave primária

`pk_launcher_release (id)` — identidade sintética. A chave de negócio é `(channel, version)`, com índice
único próprio; a PK existe para o histórico não depender de texto de versão.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (1 FK):** `fk_launcher_release_publisher`:
  `published_by_person_id → portal.person (id)`, `ON DELETE RESTRICT`, `ON UPDATE NO ACTION` — pessoa que
  publicou não é apagada enquanto o lançamento existir.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o lançamento.
- **Sem FK para o manifesto:** o banco guarda URL e hash, não o arquivo — não há tabela de armazenamento.

## Unicidade

| Único | Garante |
|---|---|
| `ux_launcher_release_versao (channel, version)` | a mesma versão não aparece duas vezes no mesmo canal |
| `ux_launcher_release_vigente (channel) WHERE status = 'published'` | **uma** versão publicada por canal |

O segundo é parcial: `draft` e `revoked` não contam, então o mesmo canal acumula histórico e só uma
`published`. Publicar a próxima exige revogar a vigente na mesma transação — o `UPDATE` solto esbarra em
`23505`.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_launcher_release_canal` | `channel` fora de `stable` e `beta` |
| `ck_launcher_release_versao` | `version` fora de `^[0-9]+[.][0-9]+[.][0-9]+(-[a-z0-9.]+)?$` |
| `ck_launcher_release_min_versao` | `min_version` preenchido fora do mesmo formato |
| `ck_launcher_release_manifesto` | `manifest_url` fora de `^https://` |
| `ck_launcher_release_sha` | `manifest_sha256` fora de 64 hex minúsculos |
| `ck_launcher_release_notas` | `notes_md` acima de 20.000 caracteres |
| `ck_launcher_release_status` | `status` fora de `draft`, `published` e `revoked` |
| `ck_launcher_release_publicacao` | `released_at` nulo com `status` diferente de `draft`, ou preenchido em `draft` |
| `ck_launcher_release_revogacao` | `revoked_at` nulo em `revoked`, ou preenchido fora de `revoked` |
| `ck_launcher_release_autor` | `status` diferente de `draft` sem `published_by_person_id` |

`version` é `TEXT` validada por regex, não tipo semver: comparar `1.10.0` com `1.9.0` é do launcher, e o
schema não ordena versão.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_launcher_release_versao (channel, version)` | evitar repetição de versão no canal |
| `ux_launcher_release_vigente (channel) WHERE status = 'published'` | consulta do launcher: uma publicada por canal |
| `ix_launcher_release_historico (channel, released_at DESC) WHERE status <> 'draft'` | histórico do canal, do lançamento mais recente |

`trg_launcher_release_set_updated_at` mantém `updated_at` no `UPDATE`, via `ops.fn_set_updated_at()`
(`0001`).

## Armadilha de leitura

Status `published` e `revoked` **não convivem** com carimbo trocado: `released_at` é obrigatório nos dois,
`revoked_at` só em `revoked`, e um lançamento revogado mantém `released_at` do dia em que saiu — é o que
responde "qual manifesto estava no ar em tal data". `manifest_sha256` é `CHAR(64)`: a comparação é exata, e
o `CHECK` exige minúsculas. `min_version` nulo significa "sem piso declarado", não "igual à versão".
`channel` é lista fechada de dois valores; um canal novo é migração, não linha.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`
  (`0001-bootstrap.md` §7), pelo BFF que serve o launcher. Nenhum seed cria lançamento.
- **Não leem:** `wyd_read` tem `USAGE` em `portal` mas não recebe `SELECT` por default; `wyd_game` não
  alcança o schema; `wyd_admin` não recebe estas tabelas em `0032`.

## Migration

Forma (dona): `migrations/0030-portal-content.md` — arquivo `0030-portal-content.sql`; DDL em
`ddl-confirmado.md` §5.15.
