# Skills do `wyd-net` — roteador

Camada de transporte entre cliente e servidor. O `wyd-net` **não** decide regra de jogo nem toca
banco: recebe, valida tamanho, enfileira e entrega.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| implementar/acomodar transporte, sessão e reconexão | [`execution/implementar-transporte`](execution/implementar-transporte/SKILL.md) | a mudança é de regra de domínio (`wyd-core`) |
