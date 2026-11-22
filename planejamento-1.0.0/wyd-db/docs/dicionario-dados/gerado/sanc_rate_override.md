> Pergunta: "o que a edição do staff sobre `content.sanc_rate` guarda e por que ela não repete o domínio do pó?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.23; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Substituição editável da célula de `content.sanc_rate` para a mesma chave `(powder, refine_index)`.
Uma linha aqui vale no lugar da base; apagar a linha é o reset. É a camada de edição da área
administrativa do portal para a taxa de SANC, separada do seed.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `powder` | `TEXT` | não | — | espelha `sanc_rate.powder`; PK com `refine_index` |
| `refine_index` | `SMALLINT` | não | — | espelha `sanc_rate.refine_index`; PK; faixa vem da FK à base |
| `rate_bps` | `INTEGER` | não | — | chance editada em pontos-base, 0–10000 |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbo da linha; sem trigger neste lote |

`updated_at` só recebe `now()` no `INSERT`; o lote não cria trigger `BEFORE UPDATE`. Autoria e valor
anterior ficam em `ops.config_change`.

## Chave primária

`pk_sanc_rate_override (powder, refine_index)` — exatamente a PK da base; uma edição por célula.

## FKs de saída e de entrada

- **Saída (1 FK):** `fk_sanc_rate_override_base`:
  `(powder, refine_index) → content.sanc_rate (powder, refine_index)`, `ON UPDATE RESTRICT`,
  `ON DELETE CASCADE`. Sem célula base não há override; apagar a base leva a edição junto.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o override.

## Unicidade

Só a PK: o mesmo pó não repete índice.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_sanc_rate_override_rate` | `rate_bps` fora de 0–10000 |

`powder`/`refine_index` não repetem `CHECK` de domínio: a FK composta garante que a célula existe na
base, e a base já limita `PO`/`PL`/`AMAGO` e 0–15. O valor, que a FK não limita, tem `CHECK` próprio.

## Índices

Nenhum além do índice da PK: ela cobre a FK composta e a leitura por `(powder, refine_index)`. Não há
índice em `updated_at`.

## Armadilha de leitura

O domínio é **0–15**, diferente de `refine_rate.step` (1–15): copiar um número entre as duas tabelas
pelo nome “passo” erra a linha. O override só existe se a célula base existir; com o seed `0023`
`bloqueada`, a edição fica impossível até a base ser semeada. E `rate_bps` não é `percent`: a conversão
de % para bps é do contrato de leitura, não uma coluna.

## Papéis

- **Escreve:** `wyd_portal`, com `SELECT, INSERT, UPDATE, DELETE` concedidos por `0022` (`M21`);
  `DELETE` é o reset para a base.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7); o `GRANT` de `0022` cobre `wyd_portal`.
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0022-content-overrides.md` — arquivo `0022_content_overrides.sql`.
