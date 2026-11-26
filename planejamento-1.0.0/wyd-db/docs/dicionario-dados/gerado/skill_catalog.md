> Pergunta: "o que `content.skill_catalog` guarda de cada skill do cliente e por que o vínculo de classe e linhagem pode ser nulo?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.19; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Catálogo da skill do cliente (154 linhas no seed de `0018`, que segue `bloqueada`): nome dobrado para
busca, vínculos opcionais de classe e linhagem e os números de combate que o servidor lê para executar a
skill. O portal lê para listar por classe e linhagem. `skill_index` é o número que
`game.character_skill` já carrega, e por isso é a chave (`P-04`). Não guarda texto de descrição,
`confidence` nem proveniência: isso é do seed e da documentação.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `skill_index` | `SMALLINT` | não | — | índice da skill no cliente, 0–247; PK |
| `name` | `TEXT` | não | — | nome exibido, 1–64 caracteres após aparar; curadoria do seed |
| `name_fold` | `TEXT` gerada `STORED` | não | — | expressão `content.fn_fold(name)`; chave da busca por prefixo |
| `class_slug` | `TEXT` | sim | — | classe de `content.game_class`; `NULL` = vínculo não provado |
| `lineage_slug` | `TEXT` | sim | — | linhagem de `content.skill_lineage`; `NULL` = vínculo não provado |
| `skill_point` | `SMALLINT` | não | `0` | custo em pontos de skill |
| `mana_spent` | `SMALLINT` | não | `0` | mana consumida |
| `delay_ticks` / `affect_time` | `SMALLINT` | não | `0` | atraso e duração **brutos** do `SkillData.csv`; o runtime converte |
| `range_tiles` | `SMALLINT` | não | `0` | alcance em tiles |
| `target_type` | `SMALLINT` | não | `0` | tipo de alvo, número do cliente |
| `instance_type` / `instance_value` | `SMALLINT` | não | `0` | efeito por instante: tipo e valor |
| `tick_type` / `tick_value` | `SMALLINT` | não | `0` | efeito por tick: tipo e valor |
| `affect_type` / `affect_value` | `SMALLINT` | não | `0` | afeto aplicado: tipo e valor |
| `instance_attribute` / `tick_attribute` | `SMALLINT` | não | `0` | atributo do efeito por instante e por tick |
| `aggressive` | `BOOLEAN` | não | `false` | o CSV traz 0/1; a coluna diz o que o campo é |
| `max_target` | `SMALLINT` | não | `1` | número máximo de alvos |
| `party` | `BOOLEAN` | não | `false` | skill de grupo |
| `affect_resist` | `SMALLINT` | não | `0` | resistência ao afeto |
| `passive` | `BOOLEAN` | não | `false` | skill passiva |
| `enabled` | `BOOLEAN` | não | `true` | aposentadoria sem `DELETE` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbos; `updated_at` pelo trigger |

`name_fold` não declara `NOT NULL`, mas nunca é nula porque `name` é `NOT NULL` e `content.fn_fold`
devolve `NULL` só para entrada nula.

## Chave primária

`pk_skill_catalog (skill_index)` — o número do cliente é a identidade. Nome não é chave e não tem
`UNIQUE`: duas skills podem ter o mesmo nome.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (2 FKs):** `fk_skill_catalog_class` (`class_slug → content.game_class`) e
  `fk_skill_catalog_lineage` (`lineage_slug → content.skill_lineage`), ambas `ON UPDATE RESTRICT` e
  `ON DELETE RESTRICT`. Classe ou linhagem referenciada não é apagada em silêncio.
- **Entrada:** nenhuma tabela da série referencia `skill_catalog`. `game.character_skill.skill_index`
  **não** tem FK de propósito: são 248 índices possíveis contra 154 linhas, e a ausência é decisão, não
  lacuna (`docs/dicionario-dados/dominio-catalogo.md` §"Armadilhas de leitura").
- **Sem FK de `game` para `content`:** o vínculo de progressão com o catálogo seria migração aditiva,
  não está nesta série.

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_skill_catalog_indice` | `skill_index` fora de 0–247 |
| `ck_skill_catalog_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |
| `ck_skill_catalog_nao_negativos` | `skill_point`, `mana_spent`, `delay_ticks`, `affect_time`, `range_tiles` ou `max_target` negativo |

Os tipos `target_type`, `instance_type`, `tick_type`, `affect_type` e os atributos não têm `CHECK` de
faixa: o conjunto de códigos é do cliente e não foi provado no planejamento (`H`).

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_skill_catalog_name_fold (name_fold text_pattern_ops)` | busca por prefixo do nome dobrado (`P-11`) |
| `ix_skill_catalog_afinidade (class_slug, lineage_slug)` | listagem por classe e linhagem, que o portal consome |

Trigger `trg_skill_catalog_set_updated_at` (`BEFORE UPDATE`, por linha).

## Armadilha de leitura

`delay_ticks` e `affect_time` são **brutos**, como vêm do CSV: a conversão é do runtime, e tratar o
número como segundo ou milissegundo é erro de leitura. `class_slug`/`lineage_slug` nulos são ausência de
prova — das 154 skills, 63 têm linhagem medida —, não skill sem classe. `enabled = false` não é lida
por `character_skill`, que não tem FK: aposentar skill não invalida progressão existente.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0018`), que está `bloqueada`.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7). Dono da permissão: `migrations/0015_content_skill.md` §8.
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0015_content_skill.md` — arquivo `0015_content_skill.sql`.
