# Skills — roteador

Uma skill por tipo de tarefa. Escolha **uma** principal por tarefa; se precisar de duas, a segunda é
verificação, não reescrita.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| auditar documento, contagem, ADR ou afirmação sem prova | [`auditar-planejamento`](auditar-planejamento/SKILL.md) | revisão de código Rust (ainda não há código) |
| abrir o planejamento de um sistema novo | [`planejar-sistema`](planejar-sistema/SKILL.md) | escrever página dentro de sistema já aberto |
| reescrever `AGENTS.md`/memória | este README + `AGENTS.md` §10 | — |

## Fronteiras

- **Skill** = como executar, com passos e critério de parada.
- **Prompt** (`.agents/prompts/`) = o que pedir a um agente, com entrada e saída.
- **Memória** (`.agents/memory/`) = estado local, não versionado.
- **Planejamento** (`planejamento-1.0.0/<sistema>/`) = conteúdo oficial do sistema.

Skills específicas de um sistema vivem no próprio sistema (`planejamento-1.0.0/<sistema>/skills/`),
como o plano diretor do `wyd-db` prevê. Não crie skill genérica nem duplique o que já existe no
sistema.

## Regra de escrita

Toda skill tem: **quando usar**, **pré-requisitos**, **passos**, **como verificar** e **quando parar
e perguntar**. Passo que não pode ser verificado não entra. Exemplo tem que ser real do repositório,
não inventado.
