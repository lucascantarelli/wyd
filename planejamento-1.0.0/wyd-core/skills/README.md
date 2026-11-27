# Skills do `wyd-core` — roteador

Regra de domínio pura: sem I/O, sem banco, sem rede. Recebe tipo do `wyd-protocol`, devolve decisão e
evento. Tudo aqui é determinístico e testável em memória.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| implementar regra de jogo, cálculo ou invariante | [`execution/implementar-regra-de-dominio`](execution/implementar-regra-de-dominio/SKILL.md) | a mudança é de transporte (`wyd-net`) ou de persistência (`wyd-db`) |
