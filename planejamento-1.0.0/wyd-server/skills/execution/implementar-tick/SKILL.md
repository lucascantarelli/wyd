---
name: implementar-tick
description: Implementa ou altera o laço de tick do servidor WYD respeitando a regra de sem I/O no tick, com fila de entrega para o banco e shutdown que fecha save e filas. Use ao ligar o loop principal, ao corrigir travamento de tick e ao implementar /health ou /ready.
---

# Implementar tick

## Quando usar

- O laço principal, o agendamento de sistemas ou o shutdown entram no escopo.
- Tick está travando ou perdendo prazo.

## Pré-requisitos

- `docs/arquitetura/fronteira-hot-cold.md` do `wyd-db` (sem I/O no tick).
- `wyd-core` e `wyd-net` compilando.

## Passos

1. **Tick sem I/O**: nada de `await` de banco ou de rede dentro do passo; o que precisa persistir vai
   para a fila.
2. **Orçamento de tempo**: meça a duração do passo e registre o pior caso; passo que estoura é defeito.
3. **Fila de entrega**: enfileirar é síncrono e limitado; a drenagem roda fora do tick.
4. **Shutdown ordenado**: parar de aceitar conexão, drenar fila, salvar e só então sair.
5. **`/health` e `/ready`**: `health` diz que o processo vive; `ready` diz que aceita tráfego — não
   confunda os dois.
6. **Escuta em `127.0.0.1`** em desenvolvimento.

## Como verificar

- Teste de tick com fila falsa: passo não bloqueia e enfileira o esperado.
- Shutdown no meio da fila não perde evento já aceito.
- `cargo test --manifest-path crates/wyd-server/Cargo.toml` verde; `GET /health` e `/ready` respondem 200
  no processo de teste.

## Quando parar e perguntar

- A solução exige I/O dentro do tick: é decisão de arquitetura, não ajuste local.
- O prazo de tick depende de capacidade de máquina (decisão de operação).

## Armadilhas

- **`await` de banco no tick** e "funciona na minha máquina".
- **`/ready` respondendo antes de a fila estar pronta.**
- **Shutdown abrupto** que perde save em lote.
