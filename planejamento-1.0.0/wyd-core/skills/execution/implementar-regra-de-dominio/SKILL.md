---
name: implementar-regra-de-dominio
description: Implementa regra de domínio do WYD em código puro — sem I/O, sem banco, sem tokio — com teste de tabela e invariante nomeada. Use ao modelar dano, XP, drop, refino, cooldown ou qualquer cálculo de jogo.
---

# Implementar regra de domínio

## Quando usar

- Cálculo ou invariante de jogo entra no escopo, sem persistência.
- Regra existente divergiu do valor esperado.

## Pré-requisitos

- Requisito numerado (`RF-*`/`RN-*`) ou página de domínio que fixe o valor esperado.
- Tipos de entrada vindos do `wyd-protocol`, não reinventados.

## Passos

1. **Função pura**: entrada por struct tipada, saída por resultado explícito; nada de `&mut` global.
2. **Sem I/O**: nenhum `sqlx`, `reqwest`, `tokio` ou `std::fs` no módulo.
3. **Invariante nomeada**: cada regra tem nome (`dano_nao_excede_hp_max`) e teste que a viola.
4. **Tabela de caso**: valores de fronteira (0, negativo, máximo) com resultado esperado.
5. **Determinismo**: se houver sorteio, a fonte de aleatoriedade é injetada e semeada no teste.

## Como verificar

```bash
cargo test --manifest-path crates/wyd-core/Cargo.toml
cargo clippy --manifest-path crates/wyd-core/Cargo.toml --all-targets -- -D warnings
```

Teste de fronteira e teste de tabela verdes, sem `unwrap()` em caminho de produção.

## Quando parar e perguntar

- A regra não está escrita em requisito nem em página: é requisito, não implementação.
- O valor depende de dado do cliente que ninguém mediu.

## Armadilhas

- **Chamar banco de dentro da regra** para "simplificar".
- **Constante mágica** sem fonte e sem nome.
- **Sorteio não semeado**, que deixa o teste instável.
