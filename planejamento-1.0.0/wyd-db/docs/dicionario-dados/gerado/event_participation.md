> Pergunta: "o que `game.event_participation` registra de cada participação e por que personagem e clã podem ficar nulos?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.28; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Histórico de participação em evento de mundo: quem participou, por qual clã, com que resultado, quantos
pontos e quando. É registro de ocorrido, não "quem está participando agora" — presença ao vivo é do
runtime, não do banco.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade da linha (`P-03`) |
| `event_code` | `TEXT` | não | — | evento disputado; FK `RESTRICT` |
| `character_id` | `BIGINT` | sim | — | personagem; `NULL` se apagado |
| `guild_id` | `BIGINT` | sim | — | clã do participante; `NULL` se apagado |
| `result` | `TEXT` | não | — | `win`, `lose`, `draw` ou `participated` |
| `score` | `BIGINT` | não | `0` | pontuação, `>= 0` |
| `at` | `TIMESTAMPTZ` | não | `now()` | momento do registro |

Sem `created_at`/`updated_at`: `at` é o carimbo, e a linha não é editada depois de gravada.

## Chave primária

`pk_event_participation (id)` — identidade sintética. O histórico não tem chave natural: o mesmo
personagem pode participar do mesmo evento várias vezes, e `(evento, personagem, instante)` seria frágil
como identidade.

## FKs de saída e de entrada

- **Saída, `RESTRICT` (1 FK):** `fk_event_participation_event`:
  `event_code → content.world_event (code)`, `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — apagar evento
  com histórico falha alto, em vez de apagar histórico de jogador (a ficha dizia `CASCADE`).
- **Saída, `SET NULL` (2 FKs):** `fk_event_participation_character` (`character_id → game.character`) e
  `fk_event_participation_guild` (`guild_id → game.guild`), `ON UPDATE RESTRICT`, `ON DELETE SET NULL` —
  apagar personagem ou clã preserva o fato de que houve participação.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o histórico.

## Unicidade

Só a PK: não há único por evento, personagem ou instante — duas participações iguais em instantes
diferentes são registros legítimos, e o schema não impede duplicata no mesmo segundo.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_event_participation_result` | `result` fora de `win`, `lose`, `draw` e `participated` |
| `ck_event_participation_score` | `score` negativo |

`character_id` e `guild_id` não exigem pelo menos um preenchido: uma linha com os dois nulos é aceita e
significa participação sem dono identificado — por exemplo, depois de o personagem ser apagado.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_event_participation_event (event_code, at DESC)` | histórico de um evento, do mais recente para o mais antigo |
| `ix_event_participation_character (character_id) WHERE character_id IS NOT NULL` | lado filho da FK nullable (`P-15`); histórico de um jogador |
| `ix_event_participation_guild (guild_id) WHERE guild_id IS NOT NULL` | lado filho da FK nullable (`P-15`); histórico de um clã |

## Armadilha de leitura

`character_id` nulo **não** quer dizer participação anônima: é o rastro de um personagem apagado, porque a
FK é `SET NULL`. `result = 'participated'` é resultado sem vitória nem derrota; `win`/`lose`/`draw` são
desfechos, e a diferença entre "participou e perdeu" e "só apareceu" está no valor, não em coluna à parte.
`score` é da participação, com unidade e escala não verificadas (`H`). E o histórico não prova que o
evento aconteceu: prova que alguém foi registrado nele.

## Papéis

- **Escreve:** `wyd_game` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `game`
  (`0001_bootstrap.md` §7). Nenhum seed grava histórico.
- **Leem:** `wyd_game`; o portal não tem `USAGE` em `game` e depende da view de `0029` (sem permissão
  concedida, decisão de 2026-11-21). `wyd_admin` não recebe esta tabela em `0032`.

## Migration

Forma (dona): `migrations/0028_game_quests_events.md` — arquivo `0028_game_quests_events.sql`.
