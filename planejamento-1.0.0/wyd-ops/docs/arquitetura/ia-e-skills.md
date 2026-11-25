# IA, skills e integrações — organização

> Pergunta: "onde vivem os arquivos de IA, skills e integrações e como o agente deve usá-los?"
> Público: Desenvolvimento e agentes
> Dono: `wyd-ops` (organização) + arquitetura
> Base: `AGENTS.md`; `.agents/README.md`; `.agents/skills/*/SKILL.md`; `.freebuff/project-id`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25).

## 1. Pastas

| Pasta | Conteúdo | Versionado? |
|---|---|---|
| `.agents/skills/` | skills do projeto (`auditar-planejamento`, `planejar-sistema`), cada uma com `SKILL.md` | sim |
| `.agents/prompts/` | prompts de cenário (`auditoria-de-cenario.md`, `README.md`) | sim |
| `.agents/memory/` | memória do agente (`estado-do-projeto.md`, `prompt-inicial.md`) | não (`.gitignore`) |
| `.freebuff/` | metadados locais (`project-id`); ignorado pelo git | não |
| `.claude/` | (ausente) — sem skills locais do Claude Code neste repo | n/a |

## 2. Skills disponíveis e quando usar

| Skill | Quando carregar | Arquivo |
|---|---|---|
| `planejar-sistema` | abrir sistema novo ou decidir fronteira/contrato | `.agents/skills/planejar-sistema/SKILL.md` |
| `auditar-planejamento` | revisar página/sistema antes de aceitar; detectar afirmação sem evidência, ADR inventado, cópia do legado | `.agents/skills/auditar-planejamento/SKILL.md` |
| cloud skills (`documentation-writer`, `frontend-design`, `writing-plans`, …) | conforme o tipo de trabalho; carregar via `cloud_read_skill` quando aplicável | conta do usuário |

Regra: o agente deve carregar a skill relevante **antes** de escrever, não depois. Issue templates agora têm campo "Skills que o agente deve carregar".

## 3. Integrações (Freebuff)

Nenhuma integração externa (GitHub/Slack) é usada no código hoje; as contas listadas no prompt
inicial são do próprio mantenedor. Se uma frente precisar (ex.: notificação), usar `search_integration_tools`
e registrar a decisão em `decisoes-registradas.md`. Segredos só via secret do GitHub ou `.env` local
ignorado (AGENTS.md §7, §9).

## 4. O que esta página não cobre

- Conteúdo das skills (ler o `SKILL.md` de cada uma).
- Skills cloud: gerenciadas na conta, não no repo.