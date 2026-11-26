> Pergunta: "o que a edição do staff sobre `content.combine_value` guarda e por que ela troca a linha inteira?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.23; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Substituição editável da linha de `content.combine_value` para a mesma chave `(family, key)`. A
substituição é **por linha inteira**: a edição pode trocar a unidade entre taxa e moeda, desde que
continue com exatamente uma preenchida. Apagar a linha é o reset para a base.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `family` | `TEXT` | não | — | espelha `combine_value.family`; PK com `key` |
| `key` | `TEXT` | não | — | espelha `combine_value.key`; PK |
| `rate_bps` | `INTEGER` | sim | — | taxa editada em pontos-base, 0–10000; uma das duas unidades |
| `coin` | `BIGINT` | sim | — | custo editado em moeda, na menor unidade; a outra unidade |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbo da linha; sem trigger neste lote |

`updated_at` só recebe `now()` no `INSERT`; o lote não cria trigger `BEFORE UPDATE`. Autoria e valor
anterior ficam em `ops.config_change`.

## Chave primária

`pk_combine_value_override (family, key)` — exatamente a PK da base; uma edição por família/chave.

## FKs de saída e de entrada

- **Saída (1 FK):** `fk_combine_value_override_base`:
  `(family, key) → content.combine_value (family, key)`, `ON UPDATE RESTRICT`,
  `ON DELETE CASCADE`. Sem linha base não há override; apagar a base leva a edição junto.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o override.

## Unicidade

Só a PK: a mesma família não repete chave.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_combine_value_override_one_unit` | as duas colunas de valor preenchidas, nenhuma preenchida, `rate_bps` fora de 0–10000 ou `coin` negativo |

A FK garante que `(family, key)` existe na base, mas **não** limita o valor: por isso o `CHECK` de
unidade é repetido aqui, e não herdado.

## Índices

Nenhum além do índice da PK: ela cobre a FK composta e a leitura por `(family, key)`. Não há índice em
`updated_at`.

## Armadilha de leitura

A edição substitui a linha inteira, não coluna a coluna. Uma base com `rate_bps` pode virar override
com `coin`, e quem lê precisa escolher a linha do override **ou** a base, nunca misturar as duas. O
`CHECK` permite `rate_bps = 0`, que é 0% medido, distinto de ausência de linha. E `family`/`key` não são
enums: a lista de famílias é `H` e não existe tabela de família nesta série.

## Papéis

- **Escreve:** `wyd_portal`, com `SELECT, INSERT, UPDATE, DELETE` concedidos por `0022` (`M21`);
  `DELETE` é o reset para a base.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7); o `GRANT` de `0022` cobre `wyd_portal`.
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0022_content_overrides.md` — arquivo `0022_content_overrides.sql`.
