> Pergunta: "o que `game.character_quest` guarda do progresso e por que a quest não pode ser apagada com progresso vivo?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.28; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Progresso de um personagem numa quest: passo atual, progresso daquele passo e os instantes de início e
conclusão. É estado de jogador, escrito só pelo servidor — não é conteúdo e não é histórico de passos
vencidos.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | personagem dono; PK com `quest_code` |
| `quest_code` | `TEXT` | não | — | quest em progresso; FK `RESTRICT` |
| `step` | `SMALLINT` | não | `1` | passo atual, maior que zero |
| `progress` | `INTEGER` | não | `0` | progresso do passo atual, `>= 0` |
| `started_at` | `TIMESTAMPTZ` | não | `now()` | quando aceitou |
| `completed_at` | `TIMESTAMPTZ` | sim | — | quando concluiu; não anterior a `started_at` |

Sem `updated_at`: a linha muda no jogo e o carimbo de alteração não foi desenhado — `started_at` e
`completed_at` são os dois únicos instantes do registro.

## Chave primária

`pk_character_quest (character_id, quest_code)` — um progresso por personagem e quest. O passo não entra
na chave: a linha é a mesma enquanto a quest avança, e `step` muda por `UPDATE`.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_character_quest_character`: `character_id → game.character (id)`,
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — o progresso é filho possuído do personagem e some com ele.
- **Saída, `RESTRICT` (1 FK):** `fk_character_quest_quest`: `quest_code → content.quest (code)`,
  `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`. Apagar definição de quest com progresso vivo falha alto, em
  vez de apagar progresso de jogador junto (a ficha do legado dizia `CASCADE`).
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o progresso.
- **Sem FK para `content.quest_step`:** o `step` guardado é um número, e o passo é filho da definição;
  uma FK composta obrigaria o progresso a apontar para um passo que pode ser reordenado no seed.

## Unicidade

Só a PK: o personagem tem no máximo um progresso por quest, e repetir a quest usa a mesma linha quando
`content.quest.repeatable` permite.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_quest_step` | `step` menor ou igual a zero |
| `ck_character_quest_progress` | `progress` negativo |
| `ck_character_quest_conclusao` | `completed_at` anterior a `started_at` |

O schema **não** deriva conclusão de `progress`/`step`: chegar ao último passo não preenche
`completed_at`, e `completed_at` preenchido com `step` intermediário passa.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_character_quest_quest (quest_code)` | lado filho da FK para `content.quest`, fora do prefixo da PK (`P-15`); sustenta o `RESTRICT` e a pergunta "quem está nesta quest" |

## Armadilha de leitura

A linha guarda o passo **atual**, não a lista de passos concluídos: passo vencido não deixa histórico, e
"quantos passos o jogador já fez" não é pergunta que esta tabela responda. `step` pode apontar para um
passo que não existe se o seed mudar depois — não há FK, e a coerência é do carregamento. `progress` é do
passo atual, não acumulado da quest. E `completed_at` nulo pode significar "em andamento" ou "concluída
sem carimbo", porque nada no schema amarra os dois.

## Papéis

- **Escreve:** `wyd_game` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `game`
  (`0001_bootstrap.md` §7). Nenhum seed cria progresso: a escrita é do servidor em runtime.
- **Leem:** `wyd_game`; o portal não tem `USAGE` em `game` e depende da view de `0029` (sem permissão
  concedida, decisão de 2026-11-21). `wyd_admin` não recebe esta tabela em `0032`.

## Migration

Forma (dona): `migrations/0028_game_quests_events.md` — arquivo `0028_game_quests_events.sql`.
