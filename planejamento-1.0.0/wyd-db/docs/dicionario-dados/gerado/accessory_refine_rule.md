> Pergunta: "o que `content.accessory_refine_rule` guarda por receita e passo e por que o material não tem FK?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.22; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Regra base de refino de acessório por `(kind, recipe, step_from)`: chance, desfecho da falha, bônus e
custo em ouro e material. O acessório é posterior ao cliente 7662 (`H`), então receita, transições e
bônus não têm regra oficial confirmada; a tabela é seed-only e read-only em runtime, e o seed `0023`
segue `bloqueada`. Não há override desta tabela em `0022`.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `kind` | `TEXT` | não | — | tipo de acessório; conjunto fechado de seis valores; PK com `recipe` e `step_from` |
| `recipe` | `TEXT` | não | — | nome da receita, 1–96 caracteres após aparar; PK |
| `step_from` | `SMALLINT` | não | — | passo de origem, 0–14; PK |
| `step_to` | `SMALLINT` | não | — | passo de destino, sempre `step_from + 1` |
| `rate_bps` | `INTEGER` | não | — | taxa em pontos-base, 0–10000 |
| `fail_mode` | `TEXT` | não | — | desfecho da falha: `keep`, `reset` ou `destroy` |
| `fail_bonus_item` | `INTEGER` | sim | — | item de bônus opcional; FK `RESTRICT` para `item_catalog` |
| `fail_bonus_bps` | `INTEGER` | não | `0` | chance do bônus, 0–10000 |
| `gold_cost` | `BIGINT` | não | `0` | custo em ouro, na menor unidade |
| `material_item` | `INTEGER` | sim | — | índice do material, 0–32767; **sem FK** |
| `material_name` | `TEXT` | não | `''` | rótulo publicado do material, até 96 caracteres |
| `material_qty` | `SMALLINT` | não | `1` | quantidade exigida, maior que 0 |

Sem `created_at`/`updated_at`, trigger ou coluna de origem: é base de seed e não recebe escrita do
staff em runtime (`migrations/0021_content_rules.md` §7).

## Chave primária

`pk_accessory_refine_rule (kind, recipe, step_from)` — uma regra por tipo, receita e passo de origem.
`step_to` não entra na chave porque é derivado de `step_from`; a mesma receita não repete o passo.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (1 FK):** `fk_accessory_refine_rule_bonus`:
  `fail_bonus_item → content.item_catalog (item_index)`, `ON UPDATE RESTRICT`,
  `ON DELETE RESTRICT`. Item de bônus referenciado não é apagado em silêncio.
- **Sem FK em `material_item`:** decisão 6 de `0021`; o material pode não existir no catálogo 7662, e
  `material_name` é o rótulo publicado, não uma cópia garantida do catálogo.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para `accessory_refine_rule`.

## Unicidade

Só a PK: o trio `(kind, recipe, step_from)` é a identidade da regra; não há segundo único.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_accessory_refine_rule_kind` | `kind` fora de `earring`, `necklace`, `belt`, `amulet`, `ankh` e `spirit_stone` |
| `ck_accessory_refine_rule_recipe` | `recipe` vazia, só com espaço, ou acima de 96 caracteres |
| `ck_accessory_refine_rule_step` | `step_from` fora de 0–14 ou `step_to` diferente de `step_from + 1` |
| `ck_accessory_refine_rule_rate` | `rate_bps` fora de 0–10000 |
| `ck_accessory_refine_rule_fail` | `fail_mode` fora de `keep`, `reset` e `destroy` |
| `ck_accessory_refine_rule_bonus` | `fail_bonus_bps` fora de 0–10000, ou bônus maior que 0 sem `fail_bonus_item` |
| `ck_accessory_refine_rule_gold` | `gold_cost` negativo |
| `ck_accessory_refine_rule_material` | `material_item` preenchido fora de 0–32767 |
| `ck_accessory_refine_rule_material_name` | `material_name` acima de 96 caracteres |
| `ck_accessory_refine_rule_material_qty` | `material_qty` menor ou igual a 0 |

`fail_bonus_bps = 0` com `fail_bonus_item` preenchido passa no `CHECK`: o schema permite o item sem
bônus, e é o runtime que decide não pagar. O inverso — bônus positivo sem item — é recusado.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_accessory_refine_rule_fail_bonus_item (fail_bonus_item) WHERE fail_bonus_item IS NOT NULL` | lado filho da FK nullable: regras que concedem um item de bônus e o `RESTRICT` do catálogo (`P-15`) |

A PK cobre a leitura por tipo/receita/passo; `material_item`, sem FK, não tem índice.

## Armadilha de leitura

`material_item` sem FK é intencional: o material pode ser de conteúdo posterior ao 7662, e
`material_name` é o que o seed publica — não presuma que o índice exista em `content.item_catalog` nem
que o nome venha do catálogo. `step_to` é sempre `step_from + 1`, não um intervalo: a PK só guarda a
origem, e uma regra “de 3 a 7” não cabe nesta tabela. Receitas e desfechos são `H`: o cliente 7662 não
é a fonte do acessório.

## Papéis

- **Escreve:** `wyd_migrator`, no seed (`0023`), que está `bloqueada`; nenhum `GRANT` de escrita em
  runtime (`migrations/0021_content_rules.md` §7).
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0021_content_rules.md` — arquivo `0021_content_rules.sql`.
