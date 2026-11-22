> Pergunta: "o que `content.quest_reward` paga por tipo e por que a linha tem `id` e único com NULLS NOT DISTINCT?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.26; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada; o seed `0027` segue `bloqueada`.

## Papel

Recompensa paga por uma quest, uma linha por tipo: item, ouro, EXP, fama ou ponto de habilidade. A
`kind` decide o significado de `amount` e se `item_index` é obrigatório. Filho possuído da quest.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade da linha (`P-03`) |
| `quest_code` | `TEXT` | não | — | quest que paga; FK `CASCADE` |
| `kind` | `TEXT` | não | — | `item`, `gold`, `exp`, `fame` ou `skill_point` |
| `item_index` | `INTEGER` | sim | — | item da recompensa; preenchido **só** em `kind = 'item'` |
| `amount` | `BIGINT` | não | `1` | quantidade, maior que zero |

Sem carimbos e sem coluna de origem: definição de conteúdo, seed-only.

## Chave primária

`pk_quest_reward (id)` — identidade sintética. A ficha do legado usava PK de quatro colunas com
`item_index NOT NULL`, o que contradizia o próprio `CHECK`: ouro e EXP não têm item, e a PK obrigaria
toda recompensa a ter um. A identidade resolve isso sem transformar `item_index` em sentinela.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_quest_reward_quest`: `quest_code → content.quest (code)`,
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — recompensa é linha possuída.
- **Saída, `RESTRICT` (1 FK):** `fk_quest_reward_item`: `item_index → content.item_catalog (item_index)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — item pago por quest não é apagado em silêncio.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para a recompensa.

## Unicidade

`uq_quest_reward UNIQUE NULLS NOT DISTINCT (quest_code, kind, item_index)` — uma recompensa por tipo e
item na mesma quest. **`NULLS NOT DISTINCT` é o que faz o único valer para ouro, EXP, fama e ponto:** sem
ele, duas linhas com `item_index NULL` do mesmo `kind` seriam consideradas distintas e a quest pagaria
ouro duas vezes. Como consequência, dois valores de ouro na mesma quest não cabem em duas linhas — somar
exige uma linha só.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_quest_reward_kind` | `kind` fora de `item`, `gold`, `exp`, `fame` e `skill_point` |
| `ck_quest_reward_item` | `item_index` preenchido com `kind` diferente de `item`, ou `kind = 'item'` sem item |
| `ck_quest_reward_amount` | `amount` menor ou igual a zero |

O `CHECK` é de equivalência (`(kind = 'item') = (item_index IS NOT NULL)`): não existe recompensa de item
sem item nem recompensa de ouro com item.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_quest_reward_item (item_index) WHERE item_index IS NOT NULL` | lado filho da FK nullable, fora da PK (`P-15`); sustenta o `RESTRICT` do catálogo |

O único `uq_quest_reward` cria o próprio índice; a PK cobre a leitura por `id`, que não é a leitura de
negócio.

## Armadilha de leitura

`id` é identidade técnica, não chave de negócio: quem lê usa `(quest_code, kind, item_index)`, e citar
`id` em contrato de API cria acoplamento a um número gerado. `amount` muda de unidade conforme `kind` —
moeda, EXP, fama, ponto ou quantidade de item — e o schema não guarda a unidade: a conversão e a unidade
de EXP/fama são `H`, herdadas do legado. `NULLS NOT DISTINCT` é sensível ao `kind`: `gold` e `exp` com
`item_index NULL` são linhas diferentes e convivem.

## Papéis

- **Escreve:** `wyd_migrator`, no seed de quests (`0027`, `bloqueada`, `migrations-checklist.md` §2);
  nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0026-content-quests-events.md` — arquivo `0026_content_quests_events.sql`.
