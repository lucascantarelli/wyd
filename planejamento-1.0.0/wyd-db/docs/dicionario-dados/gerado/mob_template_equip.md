> Pergunta: "o que `content.mob_template_equip` guarda por template e slot e por que o item vem do catálogo?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.25; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Equipamento do template por slot: qual `item_index` o mob veste em cada posição 0–15. O item é
referência ao catálogo (`0014`), não texto livre nem cópia de atributo — o efeito de vestir é do
`wyd-server`, e o banco só guarda o vínculo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `template_name` | `TEXT` | não | — | template que veste; PK com `slot` |
| `slot` | `SMALLINT` | não | — | posição de equipamento, 0–15 |
| `item_index` | `INTEGER` | não | — | item do catálogo, 0–32767 |

Sem carimbos, trigger ou coluna de origem: é import puro do conteúdo, e a proveniência fica em
`content.content_source` (`0019`).

## Chave primária

`pk_mob_template_equip (template_name, slot)` — o template veste no máximo um item por slot. `slot` é a
posição do cliente, não uma ordem de preenchimento.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_mob_template_equip_template`:
  `template_name → content.npc_template (template_name)`, `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — o
  equipamento é filho possuído do template.
- **Saída, `RESTRICT` (1 FK):** `fk_mob_template_equip_item`:
  `item_index → content.item_catalog (item_index)`, `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — item
  equipado por mob não é apagado em silêncio.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para `mob_template_equip`.

## Unicidade

Só a PK. O mesmo `item_index` em dois slots do mesmo template **é permitido**: a identidade é
`(template_name, slot)`, e nada no schema proíbe equipar o mesmo item duas vezes no mesmo mob — essa é
uma leitura do runtime, não uma garantia do banco.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_mob_template_equip_slot` | `slot` fora de 0–15 |
| `ck_mob_template_equip_item` | `item_index` fora de 0–32767 |

O domínio 0–32767 é o do catálogo (`0014`), não a faixa 0–6499 citada na ficha do legado: a faixa do
cliente é do loader, e o banco aceita o domínio que o catálogo já usa.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_mob_template_equip_item (item_index)` | lado filho da FK do item, que não é prefixo da PK (`P-15`); sustenta o `RESTRICT` do catálogo e a pergunta "quais mobs vestem este item" |

## Armadilha de leitura

`slot` é a posição do cliente (0–15), não o número do item nem uma sequência contígua: slot 7 vazio é
ausência de linha, não linha com `NULL`. O item não é copiado do catálogo — mudar o catálogo muda o que o
mob veste, e não existe foto do item nesta tabela. E o `RESTRICT` do item vale para apagar: aposentar item
é `content.item_catalog.enabled = false`, não `DELETE`.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed de origem (`0020`/`0023`, ambos `bloqueada`,
  `migrations-checklist.md` §2); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0025_content_combat.md` — arquivo `0025_content_combat.sql`.
