> Pergunta: "o que `content.quest_step` guarda de cada passo e por que o alvo não tem FK?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.26; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada; o seed `0027` segue `bloqueada`.

## Papel

Passo de quest, possuído pela quest: ordem, objetivo, alvo, quantidade e — só quando o objetivo é
`reach` — mapa e coordenada. O `step` é ordinal dentro da quest, não uma entidade com identidade própria.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `quest_code` | `TEXT` | não | — | quest dona; PK com `step` |
| `step` | `SMALLINT` | não | — | ordem do passo, maior que zero |
| `objective` | `TEXT` | não | — | verbo do passo, oito valores fechados |
| `target_id` | `INTEGER` | sim | — | alvo polimórfico (mob, item, mapa); **sem FK** |
| `count` | `INTEGER` | não | `1` | quantidade necessária, maior que zero |
| `map_id` | `SMALLINT` | sim | — | mapa do passo; FK `RESTRICT` |
| `pos_x` | `SMALLINT` | sim | — | coordenada X, 0–32767 |
| `pos_y` | `SMALLINT` | sim | — | coordenada Y, 0–32767 |
| `description` | `TEXT` | não | `''` | texto de autoria/portal |

Sem carimbos: definição de conteúdo, seed-only.

## Chave primária

`pk_quest_step (quest_code, step)` — um passo por posição na quest. `step` não é contíguo nem
sequencialmente gerado: o schema aceita os passos 1, 5 e 9 da mesma quest, e a ordem de leitura é
crescente por `step`.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_quest_step_quest`: `quest_code → content.quest (code)`,
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — o passo é filho possuído.
- **Saída, `RESTRICT` (1 FK):** `fk_quest_step_mapa`: `map_id → content.map_definition (map_id)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — mapa usado por passo não é apagado em silêncio.
- **Entrada:** nenhuma. O progresso em `game.character_quest` (`0028`) guarda o número do passo, sem FK
  para esta tabela.
- **Sem FK em `target_id`:** o alvo é polimórfico — mob, item ou mapa, conforme o objetivo (`H`, o
  mapeamento por verbo não foi reverificado) — e uma FK única não serviria a três catálogos.

## Unicidade

Só a PK. O mesmo `target_id` pode aparecer em passos diferentes e em quests diferentes — repetir o alvo
não é erro de dado.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_quest_step_step` | `step` menor ou igual a zero |
| `ck_quest_step_objetivo` | `objective` fora de `kill`, `collect`, `talk`, `reach`, `refine`, `compose`, `use_item` e `escort` |
| `ck_quest_step_alvo` | `target_id` negativo |
| `ck_quest_step_count` | `count` menor ou igual a zero |
| `ck_quest_step_reach_posicao` | `objective = 'reach'` sem `map_id`, `pos_x` ou `pos_y` |
| `ck_quest_step_posicao` | `pos_x` ou `pos_y` fora de 0–32767, quando preenchidos |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_quest_step_mapa (map_id) WHERE map_id IS NOT NULL` | lado filho da FK nullable, fora da PK (`P-15`); sustenta o `RESTRICT` de `map_definition` |

## Armadilha de leitura

`target_id` sem FK é intencional: o mesmo número significa mob, item ou mapa conforme o `objective`, e
nenhum catálogo único o valida. `pos_x`/`pos_y` podem existir sem `map_id` — o `CHECK` só exige posição
completa em `reach`, não proíbe coordenada avulsa nos outros verbos. `step` é ordem, não contagem: uma
quest com passos 1 e 3 tem dois passos, e "passo 2" não existe. E a faixa de coordenada é 0–32767, não o
teto 4095 do loader do cliente.

## Papéis

- **Escreve:** `wyd_migrator`, no seed de quests (`0027`, `bloqueada`, `migrations-checklist.md` §2);
  nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0026-content-quests-events.md` — arquivo `0026_content_quests_events.sql`.
