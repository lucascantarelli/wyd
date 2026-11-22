> Pergunta: "o que `content.game_matrix` guarda e por que cada linha é um array em vez de colunas nomeadas?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Bloco numérico do `gameconfig.txt` na ordem do arquivo: uma linha por posição `(seção, bloco, linha)`, sem
nome de coluna para cada número.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `section` | `TEXT` | não | — | seção do arquivo, 1–64 caracteres após aparar; PK composta |
| `block_index` | `SMALLINT` | não | — | bloco dentro da seção, `>= 0` |
| `line_index` | `SMALLINT` | não | — | linha dentro do bloco, `>= 0` |
| `row_values` | `SMALLINT[]` | não | — | valores na ordem do arquivo, sem nome por posição |
| `source_line` | `INTEGER` | não | — | linha do arquivo de origem, `> 0` |

## Chave primária

`pk_game_matrix (section, block_index, line_index)` — a posição é a chave; não há id de matriz nem de
bloco.

## FKs de saída e de entrada

- **Saída:** nenhuma. `section` aqui não referencia `content.game_setting`: seção de matriz e seção de
  parâmetro são conjuntos independentes, sem tabela de seção e sem FK entre as duas.
- **Entrada:** nenhuma.

## Unicidade

Só a PK: uma linha por posição, e o mesmo array pode se repetir em posições diferentes.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_game_matrix_secao` | `section` vazia, só com espaço, ou acima de 64 caracteres |
| `ck_game_matrix_bloco` | `block_index` negativo |
| `ck_game_matrix_linha` | `line_index` negativo |
| `ck_game_matrix_valores` | array sem nenhuma posição, ou com `NULL` em alguma posição |
| `ck_game_matrix_origem` | `source_line` menor ou igual a zero |

O piso de tamanho dentro de `ck_game_matrix_valores` é `array_length(row_values, 1) >= 1`, e ele **não**
recusa o array vazio: com `'{}'` o predicado avalia nulo, e `CHECK` só recusa `false`.

## Índices

Nenhum além da PK — a leitura é sempre pela posição. Sem `created_at`/`updated_at` e sem trigger: é
tabela de import puro.

## Papéis

- **Escreve:** `wyd_migrator` — carga e seed (`0020`).
- **Lê:** `wyd_game`, `wyd_portal` e `wyd_read` (`SELECT` default de `content` em `0001`).
- **Não escreve em runtime:** sem `GRANT` de escrita em `content` para papel de runtime.

## Migration

Forma (dona): `migrations/0019-content-world.md` — arquivo `0019_content_world.sql`.
