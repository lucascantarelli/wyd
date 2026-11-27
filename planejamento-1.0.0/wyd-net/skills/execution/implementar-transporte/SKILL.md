---
name: implementar-transporte
description: Implementa ou ajusta o transporte do servidor WYD — aceitação de conexão, enquadramento de pacote, backpressure, limite e reconexão — sem lógica de domínio nem I/O de banco. Use ao ligar o handshake, ao corrigir sessão que trava e ao tratar desconexão.
---

# Implementar transporte

## Quando usar

- Handshake, enquadramento, timeout ou reconexão entram no escopo.
- Conexão cai e o servidor não se recupera.

## Pré-requisitos

- Spec do pacote em `wyd-protocol/docs/` com opcode e layout.
- `wyd-net` compilando; Tokio disponível no crate.

## Passos

1. **Enquadramento primeiro**: leia o cabeçalho, valide tamanho contra o máximo declarado, rejeite o
   resto — pacote malformado não chega ao domínio.
2. **Sessão**: estado explícito (conectado, autenticado, fechando); transição só por evento nomeado.
3. **Backpressure**: fila com limite; ao encher, decida entre bloquear leitura ou derrubar a sessão —
   e registre a escolha.
4. **Timeout e reconexão**: prazo explícito, sem laço infinito de retry.
5. **Sem domínio**: nada de regra de jogo aqui; evento segue para `wyd-core`/`wyd-server`.
6. **Endereço de escuta** `127.0.0.1` em desenvolvimento.

## Como verificar

- Teste com servidor falso e cliente falso: pacote válido, pacote truncado, pacote acima do limite.
- Queda de conexão no meio do pacote não vaza tarefa nem memória.
- `cargo test --manifest-path crates/wyd-net/Cargo.toml` verde (não há workspace na raiz).

## Quando parar e perguntar

- A mudança exige alterar o protocolo (é `wyd-protocol`, com captura que prove).
- O limite/política de reconexão é decisão de operação.

## Armadilhas

- **Somar regra de jogo** ao handler de rede.
- **Confiar no tamanho do cabeçalho** sem validar contra o máximo.
- **Retry infinito** mascarando servidor fora do ar.
