> Pergunta: "o que `content.drop_rate_override` guarda do drop e por que `weight` não é porcentagem?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.25; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Regra de drop por template e item: **peso** relativo e faixa de pilha. O roll é do `wyd-server` — o banco
guarda o insumo, comparado em 1..1.000.000, e não uma probabilidade já calculada. É tabela de conteúdo,
seed-only e read-only em runtime.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `template_name` | `TEXT` | não | — | template que dropa; PK com `item_index` |
| `item_index` | `INTEGER` | não | — | item dropado, do catálogo (`0014`) |
| `weight` | `BIGINT` | não | — | peso relativo, maior que zero |
| `min_qty` | `SMALLINT` | não | `1` | pilha mínima |
| `max_qty` | `SMALLINT` | não | `1` | pilha máxima, `>= min_qty` |

Sem carimbos e sem proveniência por linha; `content_source` é de `0019`, e observação livre não vira
coluna — `notes` da ficha do legado ficou fora (`ddl-confirmado.md` §5.25).

## Chave primária

`pk_drop_rate_override (template_name, item_index)` — um peso por item e template. O mesmo item em dois
templates são duas linhas independentes; não há tabela de drop com `id` próprio.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_drop_rate_override_template`:
  `template_name → content.npc_template (template_name)`, `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — sem
  template não há drop.
- **Saída, `RESTRICT` (1 FK):** `fk_drop_rate_override_item`:
  `item_index → content.item_catalog (item_index)`, `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`. Decisão
  do dono em 2026-11-21, contra o `CASCADE` da ficha: o catálogo é conteúdo e a regra é de combate, e
  item apagado não leva regra de drop em silêncio.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para `drop_rate_override`.

## Unicidade

Só a PK: a mesma dupla template × item não repete peso.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_drop_rate_override_peso` | `weight` menor ou igual a zero |
| `ck_drop_rate_override_qtd` | `min_qty` menor ou igual a zero, ou `max_qty` menor que `min_qty` |
| `ck_drop_rate_override_item` | `item_index` fora de 0–32767 |

`weight` **não** tem teto: o limite de 1.000.000 é a amplitude do roll, não uma faixa da coluna. Um peso
maior que 1.000.000 é aceito pelo schema e é leitura do runtime.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_drop_rate_override_item (item_index)` | lado filho da FK do item, fora do prefixo da PK (`P-15`); sustenta o `RESTRICT` do catálogo e a pergunta "quem dropa este item" |

## Armadilha de leitura

`weight` não é percentual: 0–100 aqui seria erro de leitura, porque o servidor compara o peso ao roll
1..1.000.000. O peso é **relativo aos outros itens do mesmo template** — somar pesos de templates
diferentes não produz chance. `min_qty`/`max_qty` são `SMALLINT`, então a pilha tem teto de tipo
(32767), não de regra. E o multiplicador de drop de `content_rate_override` incide no runtime; o valor
gravado aqui é o base.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed de origem (`0020`/`0023`, ambos `bloqueada`,
  `migrations-checklist.md` §2); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0025_content_combat.md` — arquivo `0025_content_combat.sql`.
