---
name: implementar-bot
description: Implementa robô de carga do WYD contra ambiente de teste, com conta isolada, limite de taxa e relatório de resultado. Use ao criar cenário de estresse, medir capacidade e reproduzir carga que derrubou o servidor.
---

# Implementar bot

## Quando usar

- Cenário de carga novo ou ajuste de comportamento de bot.
- Investigar queda de servidor sob carga.

## Pré-requisitos

- Servidor de teste em `127.0.0.1` e contas de teste que não sejam de jogador real.
- `wyd-protocol` com o handshake documentado.

## Passos

1. **Alvo explícito**: endereço e porta do ambiente de teste; produção é proibida por padrão.
2. **Conta isolada**: credencial de teste vinda de configuração local, nunca de arquivo versionado.
3. **Limite de taxa**: cada bot respeita o limite declarado; carga cresce por passo, não de uma vez.
4. **Comportamento nomeado**: "andar", "atacar", "comprar" — cada um com métrica própria.
5. **Relatório**: conexões, erros, latência p50/p95 e duração, em formato estável para diff.
6. **Parada limpa**: encerrar bots e fechar sessão ao fim, sem deixar conexão pendurada.

## Como verificar

- Rodada curta contra servidor local gera relatório completo.
- Aumentar bots aumenta carga de forma monotônica até o limite declarado.
- `cargo test -p wyd-bots` verde.

## Quando parar e perguntar

- A carga precisa rodar contra ambiente compartilhado ou produção.
- O resultado depende de capacidade de máquina que você não controla.

## Armadilhas

- **Apontar bot para produção.**
- **Credencial de jogador real** em configuração.
- **Sem limite de taxa**, transformando teste em ataque.
