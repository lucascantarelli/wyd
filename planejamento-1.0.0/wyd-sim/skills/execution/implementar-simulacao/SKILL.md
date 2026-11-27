---
name: implementar-simulacao
description: Implementa cenário de simulação do WYD reproduzível — seed fixa, entrada de conteúdo versionada e saída comparável — para balancear dano, XP, drop ou economia. Use ao criar cenário novo, ao comparar duas configurações e ao investigar resultado que não se repete.
---

# Implementar simulação

## Quando usar

- Precisa comparar números de balanceamento sem abrir o jogo.
- Um resultado da simulação não se repete.

## Pré-requisitos

- Regra implementada em `wyd-core` (a simulação não reimplementa regra).
- Conteúdo vindo de `content.*` do banco novo, com a versão registrada.

## Passos

1. **Seed explícita**: toda aleatoriedade parte de uma semente informada e registrada na saída.
2. **Entrada versionada**: registre `content_version` e o conjunto de linhas usado.
3. **Cenário nomeado**: parâmetros em struct, não em constante escondida.
4. **Saída comparável**: resumo agregado (média, p95, total) em formato estável para diff.
5. **Custo**: declare iterações e tempo; cenário que demora demais vira amostra declarada.

## Como verificar

- Rodar duas vezes com a mesma seed devolve o mesmo resultado (golden).
- Trocar a seed muda o resultado, mas não a ordem de grandeza.
- `cargo test -p wyd-sim` verde.

## Quando parar e perguntar

- O cenário exige conteúdo que não está medido no cliente.
- O resultado só faz sentido com decisão de produto (taxa, meta de progressão).

## Armadilhas

- **Aleatoriedade sem seed** e conclusão que não se reproduz.
- **Reimplementar a regra** dentro da simulação e divergir do servidor.
- **Usar dado do `deprecated/`** como entrada.
