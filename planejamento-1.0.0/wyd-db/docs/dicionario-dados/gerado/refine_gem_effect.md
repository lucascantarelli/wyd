> Pergunta: "o que `content.refine_gem_effect` guarda por gema e por que `item_index` é a chave?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.22; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Efeito que uma gema de refino aplica, com valor fixo e valor por nível, faixa de refino em que vale e se
aceita item Ancient. A gema é um item do catálogo, e `item_index` é a chave: **uma** regra por gema.
O `effect_code` reusa o `slug` de `content.item_effect` (`0013`) em vez de criar um segundo vocabulário.
Tabela seed-only e read-only em runtime; o seed é o `0023`, que segue `bloqueada`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `item_index` | `INTEGER` | não | — | gema, índice do item no cliente, 0–32767; PK |
| `effect_code` | `TEXT` | não | — | efeito de `content.item_effect`; FK |
| `flat_value` | `SMALLINT` | não | `0` | valor fixo do efeito |
| `per_level_value` | `SMALLINT` | não | `0` | valor somado por nível |
| `min_refine` | `SMALLINT` | não | `10` | refino mínimo em que o efeito vale, 0–15 |
| `max_refine` | `SMALLINT` | não | `15` | refino máximo, entre `min_refine` e 15 |
| `allows_ancient` | `BOOLEAN` | não | `true` | se a regra aceita item Ancient |

Sem `created_at`/`updated_at` e sem trigger: base de seed, sem escrita do staff em runtime
(`migrations/0021-content-rules.md` §7).

## Chave primária

`pk_refine_gem_effect (item_index)` — a gema é a identidade e a regra é 1:1 com ela; um segundo efeito
para a mesma gema seria linha nova em lote novo, não duas linhas aqui.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (2 FKs):** `fk_refine_gem_effect_item` (`item_index → content.item_catalog`) e
  `fk_refine_gem_effect_codigo` (`effect_code → content.item_effect`), ambas `ON UPDATE RESTRICT` e
  `ON DELETE RESTRICT`. Item ou efeito referenciado não é apagado em silêncio.
- **Entrada:** nenhuma tabela da série aponta para `refine_gem_effect`.

## Unicidade

Só a PK — nenhum único adicional; `effect_code` pode se repetir entre gemas diferentes.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_refine_gem_effect_item` | `item_index` fora de 0–32767 |
| `ck_refine_gem_effect_valores` | `flat_value` ou `per_level_value` negativo |
| `ck_refine_gem_effect_faixa` | `min_refine` fora de 0–15, ou `max_refine` menor que `min_refine` ou acima de 15 |

`min_refine = max_refine` é aceito: a regra vale em um único passo. O `CHECK` não exige
`min_refine <= 10`: o default é só ponto de partida do seed.

## Índices

Nenhum além do índice da PK. Não há índice preventivo em `effect_code`: `item_effect` é look-up
somente-leitura e a leitura primária é pela gema (`P-15`, `migrations/0021-content-rules.md` §7).

## Armadilha de leitura

Gema **sem** linha aqui não tem efeito provado — a ausência não significa "sem bônus". `allows_ancient`
default `true` não é decisão de conteúdo: é o default do schema. E `flat_value`/`per_level_value` são
somados pelo runtime com a fórmula de refino, que não está nesta tabela.

## Papéis

- **Escreve:** `wyd_migrator`, no seed (`0023`), que está `bloqueada`; nenhum `GRANT` de escrita em
  runtime (`migrations/0021-content-rules.md` §7).
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0021-content-rules.md` — arquivo `0021-content-rules.sql`.
