> Pergunta: "o que `content.level_item` guarda do LevelItem.txt e como classe e nível se combinam?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel
**Sem arquivo no repositório:** `LevelItem.txt` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Item entregue por nível e classe, na ordem do arquivo `LevelItem.txt`. `ordinal` é a ordem depois de
deduplicar o bloco repetido do insumo; não é o nível nem a sequência. Não guarda a entrega ao
personagem — isto é catálogo de conteúdo, não estado de jogador.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `ordinal` | `SMALLINT` | não | — | ordem após deduplicar o bloco repetido do insumo |
| `source_seq` | `SMALLINT` | não | — | sequência no arquivo de origem |
| `source_line` | `INTEGER` | não | — | linha do arquivo de origem |
| `level` | `SMALLINT` | não | — | nível do personagem |
| `class_id` | `SMALLINT` | não | — | classe do personagem; sem FK, ver abaixo |
| `type_id` | `SMALLINT` | não | — | TYPE do arquivo, com significado registrado abaixo |
| `item_index` | `INTEGER` | não | — | item entregue (FK) |
| `extras` | `JSONB` | não | `'[]'` | bloco posicional extra, ainda não decodificado |

`type_id` tem significado registrado no DDL: −1 vale independente de classe, 0 independente de pontos,
1 STR, 2 INT e 3 DEX.

## Chave primária

`pk_level_item (ordinal)` — a ordem é a identidade da linha; o par `(level, class_id, item_index)` não é
chave e pode se repetir em ordinais diferentes.

## FKs de saída e de entrada

- **Saída, `RESTRICT`:** `fk_level_item_catalogo`: `item_index → content.item_catalog (item_index)`
  (`0014`), `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` — item com regra de nível não é apagado em
  silêncio.
- **Entrada:** nenhuma.
- **Sem FK em `class_id`:** `content.game_class` (`0013`) tem `slug`, não o código numérico do arquivo;
  a correspondência código → slug é curadoria do seed (`0017`/`0018`) e é `H`, declarada em
  `migrations/0019_content_world.md` §9.

## Unicidade

Só a PK: nada no banco impede a mesma combinação de nível, classe e item em dois ordinais. A
deduplicação é do seed, não do schema.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_level_item_ordinal` | `ordinal` menor ou igual a zero |
| `ck_level_item_seq` | `source_seq` negativo |
| `ck_level_item_origem` | `source_line` menor ou igual a zero |
| `ck_level_item_nivel` | `level` fora de 1–255 |
| `ck_level_item_classe` | `class_id` fora de 0–3 |
| `ck_level_item_tipo` | `type_id` fora de −1–3 |
| `ck_level_item_item` | `item_index` fora de 0–32767 |
| `ck_level_item_extras` | `extras` que não é array JSON |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_level_item_classe (class_id, level)` | itens de level-up por classe e nível |

Sem carimbos e sem trigger: import puro.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019_content_world.md` §8.

## Migration

Forma (dona): `migrations/0019_content_world.md` — arquivo `0019_content_world.sql`.
