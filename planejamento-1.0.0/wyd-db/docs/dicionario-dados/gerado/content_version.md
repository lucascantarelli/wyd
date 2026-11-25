> Pergunta: "o que `content.content_version` guarda e por que ela tem uma linha só?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §3; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Versão única do conteúdo do banco: uma linha que diz qual conjunto de `content.*` está carregado.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | — | a PK; o `CHECK` a fixa em `1` |
| `version` | `INTEGER` | não | — | versão do conteúdo, `>= 1` |
| `note` | `TEXT` | não | `''` | anotação da carga |
| `applied_at` | `TIMESTAMPTZ` | não | `now()` | instante da carga |

## Chave primária

`pk_content_version (id)` — com `ck_content_version_linha_unica (id = 1)`, impede a segunda linha. A
tabela é um singleton, não uma entidade com histórico.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada:** `game.character.content_version → content.content_version (id)` — o personagem guarda o
  molde de conteúdo que o criou (`D-39`), sem `ON DELETE` explícito.

## Unicidade

Só a PK. Não há `slug` nem chave natural: a versão é o próprio número da linha.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_content_version_linha_unica` | `id` diferente de `1` |
| `ck_content_version_versao` | `version` menor que `1` |

## Índices

Nenhum além da PK: a tabela tem uma linha e é lida pela chave.

## Papéis

- **Escreve:** `wyd_migrator` — a versão sobe na carga de conteúdo, não em runtime de jogo.
- **Lê:** `wyd_game`, `wyd_portal` e `wyd_read` (`SELECT` default de `content` em `0001`).
- **Não escreve:** `wyd_game` e `wyd_portal`; não há `GRANT` de escrita em `content` para eles.

## Migration

Forma (dona): `migrations/0001-bootstrap.md` — arquivo `0001-bootstrap.sql`.
