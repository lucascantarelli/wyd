# `.agents/` — arquivos de IA do projeto

> Pergunta: "onde ficam as skills, os prompts e a memória do agente, e o que é versionado?"
> Público: agentes de IA e quem mantém o repositório
> Dono: `wyd-ops` (organização) + arquitetura
> Base: `AGENTS.md` §7–§10; `planejamento-1.0.0/wyd-ops/docs/arquitetura/ia-e-skills.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). A organização canônica — quais skills existem, quando carregar cada
uma e o que fica fora do git — é de `planejamento-1.0.0/wyd-ops/docs/arquitetura/ia-e-skills.md` §1–§3;
este arquivo só orienta quem abre a pasta.

## Pastas

| Pasta | O que tem | Versionado |
|---|---|---|
| `skills/` | uma pasta por skill, cada uma com `SKILL.md` (`auditar-planejamento`, `planejar-sistema`); o `README.md` de `skills/` explica o formato | sim |
| `prompts/` | prompts de cenário (`auditoria-de-cenario.md`) e o `README.md` de uso | sim |
| `memory/` | memória local do agente (`estado-do-projeto.md`, `prompt-inicial.md`) | não — `.gitignore` |

## Como usar

1. Antes de escrever planejamento, leia a skill que corresponde à tarefa: `planejar-sistema` para abrir
   sistema novo, `auditar-planejamento` para revisar o que existe.
2. Skills da conta (fora do repositório) são carregadas com `cloud_read_skill`; elas não são copiadas
   para cá.
3. `memory/` é rascunho de sessão: nada que precise valer como fonte de verdade mora ali. O fato vai
   para `planejamento-1.0.0/`, e o dono é o documento do sistema.
4. Regras que valem para qualquer agente — invariantes, evidência `V`/`H`/`P`, o que não se escreve fora
   do pedido — estão em `AGENTS.md`, não aqui.

## Limites

- Este arquivo não define skill nova nem decide fronteira de sistema; isso é proposta com aprovação do
  dono (`AGENTS.md` §5.8).
- Nenhum segredo mora em `.agents/`: DSN, token e chave são descritos por tipo e lugar (`AGENTS.md` §5.7).
