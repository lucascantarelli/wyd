# Skills do `wyd-server` — roteador

Runtime do jogo: laço de tick, filas, sessões e o par `/health` `/ready`. Regra de ouro: **sem I/O no
tick** — a ponte com o banco é a fila de entrega do `wyd-db`.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| implementar/alterar o laço de tick ou o shutdown | [`execution/implementar-tick`](execution/implementar-tick/SKILL.md) | a mudança é de regra pura (`wyd-core`) |
