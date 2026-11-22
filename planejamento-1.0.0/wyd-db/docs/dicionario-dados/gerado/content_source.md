> Pergunta: "o que `content.content_source` prova sobre cada arquivo importado?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3 e §4
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Proveniência por dataset: qual caminho foi importado, com que hash, tamanho, quantas linhas foram
aproveitadas e que transformação sofreu. Não guarda o dado importado nem o conteúdo do arquivo — guarda
a prova do insumo, para que a carga de `0020` possa ser conferida depois.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `path` | `TEXT` | não | — | caminho do insumo, relativo à raiz |
| `sha256` | `TEXT` | não | — | hash do arquivo importado |
| `size_bytes` | `INTEGER` | não | — | tamanho do arquivo em bytes |
| `row_count` | `INTEGER` | não | — | linhas aproveitadas do arquivo; zero é aceito |
| `loaded_by` | `TEXT` | não | — | quem carregou (script ou migration) |
| `transform` | `TEXT` | não | `''` | transformação aplicada ao insumo |
| `imported_at` | `TIMESTAMPTZ` | não | `now()` | instante da importação; é o carimbo desta tabela |

## Chave primária

`pk_content_source (path)` — o caminho é a identidade do dataset: o mesmo arquivo não pode ter duas
linhas de proveniência concorrentes.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** nenhuma. Nenhuma constraint da série referencia esta tabela (`V`, 2026-11-22: nenhum
  `REFERENCES content.content_source` nas páginas de `migrations/`). As páginas de `0021`, `0024` e
  `0026` dizem que a proveniência fica aqui, mas por convenção de prosa, sem FK.

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_content_source_path` | `path` vazio, só com espaço, ou acima de 512 caracteres |
| `ck_content_source_sha256` | hash que não casa `^[0-9a-f]{64}$` |
| `ck_content_source_tamanho` | `size_bytes` menor ou igual a zero |
| `ck_content_source_linhas` | `row_count` negativo |
| `ck_content_source_autor` | `loaded_by` vazio, só com espaço, ou acima de 64 caracteres |

`row_count = 0` passa: arquivo lido sem linha aproveitada é um resultado registrável, não erro de
carga.

## Índices

Nenhum além da PK. Sem carimbos `created_at`/`updated_at` e sem trigger: `imported_at` é o carimbo, e a
linha não é editada depois da carga.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019-content-world.md` §8.

## Migration

Forma (dona): `migrations/0019-content-world.md` — arquivo `0019_content_world.sql`.
