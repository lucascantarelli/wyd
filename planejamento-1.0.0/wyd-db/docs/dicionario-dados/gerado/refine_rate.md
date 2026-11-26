> Pergunta: "o que `content.refine_rate` guarda e por que falta de linha não é taxa zero?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.22; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Taxa base de refino por família (`dust` ou `compose`) e passo de 1 a 15, em pontos-base
(`rate_bps`, 0–10000). É tabela **seed-only e read-only em runtime**: o valor entra pelo seed de `0023`
(que segue `bloqueada`) e o servidor só lê. A ausência de linha é ausência de taxa publicada, nunca
`0%`. Alteração por operação é regra de override (`content.refine_rate_override`, `0022`), não `UPDATE`
aqui.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `kind` | `TEXT` | não | — | família do refino, `dust` ou `compose`; PK com `step` |
| `step` | `SMALLINT` | não | — | passo do refino, 1–15 |
| `rate_bps` | `INTEGER` | não | — | taxa em pontos-base, 0–10000 |

Não há `created_at`/`updated_at`, trigger nem coluna de origem: a tabela é base de seed e não recebe
escrita do staff (`migrations/0021_content_rules.md` §7).

## Chave primária

`pk_refine_rate (kind, step)` — a família e o passo identificam a regra; não há `id` próprio
(`P-04`).

## FKs de saída e de entrada

- **Saída:** nenhuma; o schema `content` não aponta para `game`/`portal`.
- **Entrada (1 FK):** `fk_refine_rate_override_base` em `content.refine_rate_override` (`0022`),
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE`. Apagar a base apaga o override correspondente — diferente
  das FKs de catálogo do `0021`, que são `RESTRICT`.

## Unicidade

Só a PK: a mesma família não repete passo, e duas famílias usam o mesmo passo sem colidir.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_refine_rate_kind` | `kind` fora de `('dust', 'compose')` |
| `ck_refine_rate_step` | `step` fora de 1–15 |
| `ck_refine_rate_rate` | `rate_bps` fora de 0–10000 |

`rate_bps = 0` é aceito e significa **0% medido**, distinto de não existir linha. Nada exige que as
duas famílias tenham os 15 passos preenchidos.

## Índices

Nenhum além do índice da PK, que já cobre a busca por `(kind, step)` e a FK do override.

## Armadilha de leitura

A falta de uma linha em `+11` é ausência de taxa, e o runtime não deve inferir 0%: essa distinção é a
razão de a tabela existir em vez de um array de 15 posições. `step` aqui começa em **1**, enquanto
`content.sanc_rate.refine_index` aceita **0** — os dois intervalos não são o mesmo domínio. E o override
de `0022` tem precedência sobre a base; ler só esta tabela ignora a operação.

## Papéis

- **Escreve:** `wyd_migrator`, no seed (`0023`), que está `bloqueada`; nenhum `GRANT` de escrita em
  runtime (`migrations/0021_content_rules.md` §7).
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0021_content_rules.md` — arquivo `0021_content_rules.sql`.
