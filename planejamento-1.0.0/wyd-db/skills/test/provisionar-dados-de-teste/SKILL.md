---
name: provisionar-dados-de-teste
description: Cria conta, personagem e fixtures de teste do wyd-db por factory, sem id mágico, sleep nem estado global. Use ao escrever teste de integração que precisa de personagem, clã ou conteúdo, e ao corrigir teste que depende de ordem de execução.
---

# Provisionar dados de teste

## Quando usar

- Teste novo precisa de conta, personagem, clã ou linha de conteúdo.
- Teste existente falha conforme a ordem ou depende de `sleep`.

Não use para semear conteúdo de release: isso é seed da série, com dado medido.

## Pré-requisitos

- `docs/guia-teste/` e `tests/dados-de-teste.md` (dono das fixtures).
- PostgreSQL 17 de teste (`make db-up`) e `cargo test -p wyd-db` rodando.

## Passos

1. **Use a factory existente.** Conta/personagem nascem de função com nome de teste, não de `INSERT`
   com id fixo.
2. **Id só vem da factory** (sequência ou `RETURNING`); número literal em teste é proibido.
3. **Cada teste cria o que usa e limpa o que criou** — transação com rollback ou `TRUNCATE` do schema
   de conteúdo no início, nunca ordem implícita entre testes.
4. **Sem `sleep`.** Sincronize por estado observável (linha existe, revisão mudou), não por tempo.
5. **Sem estado global mutável** entre testes: nada de singleton, `lazy_static` compartilhado ou
   variável de ambiente alterada no meio.
6. **Conteúdo de teste é mínimo**: só a linha que o caso exercita, com `content_source` quando a
   tabela exigir.

## Como verificar

```bash
cargo test -p wyd-db -- --test-threads=1   # precisa passar igual
cargo test -p wyd-db                       # e em paralelo
```

Os dois modos verdes provam que não há dependência de ordem.

## Quando parar e perguntar

- O teste exige conteúdo real (item, skill, NPC) que não existe medido: registre a lacuna; não invente
  linha só para o teste passar.
- A fixture só funciona com dado vindo do `deprecated/`: pare — nível 4 não entra, nem em teste.

## Armadilhas

- **`sleep` como sincronização**, a causa mais comum de teste instável.
- **Fixture que depende de outra fixture** criar antes (ordem implícita).
- **Id mágico** (`6301`, `3751`) copiado de saída de banco.
- **Deixar linha criada no banco de teste** e mascarar a sujeira na execução seguinte.
