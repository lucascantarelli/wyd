# Skills do `wyd-launcher` — roteador

Launcher em Tauri (Windows-only): verifica versão, aplica patch e abre o cliente. O que se versiona é
o manifesto medido, não o binário do cliente.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| atualizar manifesto de patch ou a verificação de integridade | [`execution/atualizar-manifesto`](execution/atualizar-manifesto/SKILL.md) | a mudança é do cliente em si (`wyd-client`) |
