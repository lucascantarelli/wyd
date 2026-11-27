---
name: revisar-schema
description: Revisa DDL de uma migration do wyd-db contra o padrão do banco — tipo, FK, CHECK, índice, permissão e nome — antes do merge. Use ao abrir PR que cria ou altera tabela, ao revisar PR de outra pessoa, ou quando o gate acusa constraint sem teste.
---

# Revisar schema

## Quando usar

- PR cria/altera tabela, coluna, constraint, índice ou `GRANT`.
- O gate aponta DDL sem prova.

Não use para revisar texto de planejamento: isso é `review/revisar-documentacao`.

## Pré-requisitos

- `planejamento-1.0.0/wyd-db/docs/arquitetura/padroes-postgres.md` (critério de tipo, índice e
  armadilha).
- `planejamento-1.0.0/wyd-db/docs/arquitetura/integridade-e-constraints.md` (política de FK, CHECK e
  unicidade).
- O bloco `planejamento-1.0.0/wyd-db/ddl-confirmado.md` §5.x do arquivo em revisão.

## Passos

1. **Liste o que o arquivo cria** (`CREATE TABLE`, `CONSTRAINT`, `INDEX`, `GRANT`) e compare com o
   bloco confirmado: divergência é defeito, mesmo que "melhor".
2. **Tipo**: cada coluna usa um tipo da lista fechada e o tipo diz o domínio (`SMALLINT` para faixa
   curta, `INTEGER`/`BIGINT` conforme cardinalidade, `TEXT` sem `VARCHAR(n)`).
3. **Chave**: chave natural onde o consumidor já carrega o código; `id` sintético só onde não há.
4. **FK**: `ON DELETE` explícito e coerente com a política (CASCADE onde o filho não vive sem o pai,
   RESTRICT onde apagar esconderia histórico).
5. **CHECK**: toda invariante que o banco pode garantir sozinho está em CHECK, não em comentário.
6. **Índice**: só com motivo — invariante (unicidade) ou consulta nomeada; índice "preventivo" não
   entra.
7. **Permissão**: `content` é leitura para os três papéis de runtime; escrita só `wyd_migrator`.
8. **Nome**: `pk_`, `fk_`, `ck_`, `uq_`, `ix_` com a tabela no nome; tabela e coluna em `snake_case`
   sem acento.
9. **Ciclo**: o `down` reverte o `up` na ordem inversa, sem sobra.

## Como verificar

```bash
make db-up
# aplica a série até o arquivo e depois roda o down dele
cargo test --manifest-path crates/wyd-db/Cargo.toml
make gate-fast
```

O ciclo `up → down → up` tem de fechar sem `ERROR`; o runner tem de aceitar a ordem da série.

## Quando parar e perguntar

- A revisão conclui que a constraint precisa afrouxar para caber dado real: **decisão do dono**, com
  a página e o `ddl-confirmado.md` alterados juntos.
- A mudança cria índice/coluna fora do bloco confirmado.
- Aparece regra de negócio que o banco não consegue garantir: isso é `specs/`, não DDL.

## Armadilhas

- **Ajustar o schema ao dataset herdado** em vez de medir o dado novo.
- **CHECK que só existe no comentário** e volta como defeito em produção.
- **`down` vazio** (aceito no `up`, reprovado no ciclo).
- **Constraint sem teste que a prove** — o par constraint × prova é o que fecha a revisão.
