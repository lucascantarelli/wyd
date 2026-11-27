---
name: provar-contra-cliente
description: Prova que uma implementação de protocolo do WYD conversa com o cliente 7662 real ou com captura equivalente, sem depender de servidor de produção. Use ao fechar handshake, ao validar pacote novo e ao investigar divergência entre spec e cliente.
---

# Provar contra cliente

## Quando usar

- Implementação de protocolo vai ser dada como pronta.
- Suspeita de divergência entre spec e comportamento do cliente.

## Pré-requisitos

- Cliente 7662 local e captura de referência com `sha256`.
- Servidor de teste em `127.0.0.1`, nunca `0.0.0.0`.

## Passos

1. **Escolha a prova**: captura reproduzida (determinística) ou cliente real apontando para servidor
   local.
2. **Suba o alvo local** e registre a porta e o PID.
3. **Execute o caso**: handshake completo, pacote isolado, ou sequência documentada na spec.
4. **Compare byte a byte** com a captura de referência e registre a diferença quando houver.
5. **Guarde a evidência**: trecho da captura, saída do teste e data — não print solto.

## Como verificar

| Alegação | Prova |
|---|---|
| pacote idêntico | diff de bytes contra a captura |
| handshake completo | log do servidor local com a sequência |
| sem regressão | teste de unidade do crate verde |

## Quando parar e perguntar

- A prova exige rodar o cliente contra servidor externo ou conta real: é teste do dono.
- O cliente muda de comportamento conforme patch: registre a versão antes.

## Armadilhas

- **Declarar "funciona com o cliente"** sem ter rodado o cliente.
- **Usar `0.0.0.0`** e expor o servidor de teste.
- **Guardar captura com dado de jogador** (login, personagem) no repositório.
