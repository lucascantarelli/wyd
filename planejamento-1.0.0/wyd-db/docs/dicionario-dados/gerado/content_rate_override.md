> Pergunta: "o que `content.content_rate_override` multiplica por família e por que 10000 não é 100% do teto?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.25; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Multiplicador de taxa por família — `exp`, `drop`, `gold` e `skill_point` — em basis points. O valor
**base** continua em `content.game_setting` (`0019`); esta tabela guarda o override, e a multiplicação em
si é do runtime. Tabela raiz: não aponta para outra tabela da série e não tem chave estrangeira de saída.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `kind` | `TEXT` | não | — | família multiplicada; PK |
| `rate_bps` | `INTEGER` | não | — | multiplicador em pontos-base; `10000` = 100% |

Sem `created_at`/`updated_at` e sem trigger: como as demais tabelas do lote, é base de seed e read-only
em runtime. Autoria de edição, quando ela existir, não mora aqui — `content` não aponta para `portal`, e
o registro de mudança é `ops.config_change` (`0002`).

## Chave primária

`pk_content_rate_override (kind)` — uma linha por família. `kind` é texto com `CHECK`, não enum de
Postgres: a lista fechada é da constraint, e acrescentar família é migração.

## FKs de saída e de entrada

- **Saída:** nenhuma — é tabela raiz, sem FK.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para `content_rate_override`.

## Unicidade

Só a PK: não há duas linhas para a mesma família.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_content_rate_override_kind` | `kind` fora de `exp`, `drop`, `gold` e `skill_point` |
| `ck_content_rate_override_rate` | `rate_bps` fora de 0–1.000.000 |

O teto foi respondido pelo dono em 2026-11-21: **1.000.000** bps, que é 10.000% — margem para
multiplicador alto sem trocar o tipo. `rate_bps = 0` é aceito e significa multiplicador nulo, distinto de
ausência de linha, que significa "sem override".

## Índices

Nenhum além do índice da PK. Sem FK de entrada, não há índice de lado filho a criar.

## Armadilha de leitura

`10000` é 100%, e o teto da coluna é **100×** isso: ler o teto como 100% transforma qualquer
multiplicador acima de 1× em dado "impossível". A ausência de linha não é zero — é herdar o base de
`content.game_setting`, e o runtime decide a ordem entre base e override. As quatro famílias são uma lista
fechada sem semântica por família verificada (`H`): o que cada uma multiplica exatamente é do
`wyd-sim`/`wyd-server`, não desta tabela.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed de origem (`0020`/`0023`, ambos `bloqueada`,
  `migrations-checklist.md` §2); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0025-content-combat.md` — arquivo `0025-content-combat.sql`.
