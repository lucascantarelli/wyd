# Verificação de skills e integrações — planejamento

> Pergunta: "como verificar que as skills do agente e as integrações do Freebuff estão documentadas, carregadas e usadas corretamente?"
> Público: Desenvolvimento, arquitetura e agentes
> Dono: `wyd-ops` (organização) + arquitetura
> Base: `AGENTS.md` §5, §7 e §9; `wyd-ops/docs/arquitetura/ia-e-skills.md`; `wyd-ops/docs/github-mapping.md`; `wyd-ops/docs/specs/requisitos.md`; `wyd-ops/auditoria-de-organizacao.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Planejamento; nenhuma skill, integração, código ou teste foi alterado por este documento.

## 1. Escopo e dono

Este documento planeja a verificação de skills e integrações. Ele não implementa skill, não conecta integração, não cria issue e não altera configuração do Freebuff. O dono da organização é `wyd-ops`; a arquitetura é dona da regra de uso.

## 2. Inventário de skills

| Skill | Onde vive | Quando carregar | Verificação |
|---|---|---|---|
| `planejar-sistema` | `.agents/skills/planejar-sistema/SKILL.md` | abrir/fechar sistema, contrato, requisito, roadmap | skill carregada antes de escrever |
| `auditar-planejamento` | `.agents/skills/auditar-planejamento/SKILL.md` | antes de marcar `aceito`, depois de lote, número divergente | achados relatados por severidade |
| `documentation-writer` | conta do usuário (cloud skill) | redação e revisão técnica | carga via `cloud_read_skill` quando aplicável |
| `frontend-design` | conta do usuário (cloud skill) | UI nova ou reformulação visual | carga via `cloud_read_skill` quando aplicável |
| `writing-plans` | conta do usuário (cloud skill) | tarefa multi-etapa antes de tocar código | carga via `cloud_read_skill` quando aplicável |

Regra: o agente carrega a skill relevante **antes** de escrever, não depois. Issue templates têm campo "Skills que o agente deve carregar".

## 3. Matriz de skill-loading por trabalho

| Trabalho | Skill mínima | Quando é obrigatória |
|---|---|---|
| Abrir ou fechar sistema | `planejar-sistema` | antes de escrever ou alterar planejamento |
| Auditar página ou decisão | `auditar-planejamento` | antes de declarar `aceito` |
| Redação de documentação | `documentation-writer` | ao redigir ou revisar docs |
| Decisão de arquitetura ou fronteira | `planejar-sistema` + revisão humana | antes de ADR ou mudança de contrato |
| Trabalho de UI | `frontend-design` | ao criar ou reformular interface |
| Plano multi-etapa | `writing-plans` | antes de implementação |

## 4. Integrações (Freebuff)

| Integração | Estado hoje | Uso planejado |
|---|---|---|
| GitHub | contas do mantenedor listadas no prompt inicial | notificação ou automação futura, com decisão registrada |
| Slack | conta do mantenedor listada no prompt inicial | notificação ou automação futura, com decisão registrada |
| Outras integrações | nenhuma usada no código | descobrir via `search_integration_tools` quando uma frente precisar |

Regra: segredos só via secret do GitHub ou `.env` local ignorado. Nenhum token, DSN, hash, senha ou PIN aparece em documento, log, issue ou chat.

## 5. Verificação mínima

| Item | Como verificar | Dono |
|---|---|---|
| skill carregada antes de escrever | revisão de fluxo e issue template | `wyd-ops` |
| skill documentada | `ia-e-skills.md` e este documento | `wyd-ops` |
| campo de skill-loading nos templates | inspeção dos 4 templates em `.github/ISSUE_TEMPLATE/` | `wyd-ops` |
| integração registrada quando usada | `decisoes-registradas.md` | arquitetura |
| segredo fora do repositório | varredura de segredo | `wyd-ops` |
| cloud skill disponível | `cloud_read_skill` quando aplicável | agente |

## 6. Anti-padrões proibidos

- Escrever antes de carregar a skill relevante.
- Tratar skill como substituto de decisão do dono.
- Conectar integração sem registrar a decisão.
- Colocar token, DSN, hash, senha ou PIN em documento, log, issue ou chat.
- Usar skill cloud sem verificar disponibilidade.
- Criar skill nova sem necessidade real do projeto.

## 7. Cruzamento com auditoria e especificação

| Referência | Relação |
|---|---|
| `wyd-ops/auditoria-de-organizacao.md` §2–§3 | 13 sistemas organizados; `wyd-ops` tem os contratos O1–O5 aprovados; este documento não fecha organização por si só |
| `wyd-ops/docs/arquitetura/ia-e-skills.md` | organização de pastas, skills e integrações |
| `wyd-ops/docs/github-mapping.md` | convenções de issue, branch, PR e label |
| `wyd-ops/docs/specs/requisitos.md` | requisitos de CI, monitoramento, segredos e deploy |
| `AGENTS.md` §5, §7 e §9 | invariantes, segredo e operação |
| `.github/ISSUE_TEMPLATE/*` | campo "Skills que o agente deve carregar" |

## 8. Decisões abertas

1. **Skills adicionais do projeto** — hoje existem 2 skills locais.
   - Opções: A) manter apenas as duas; B) criar skill para auditoria de CI; C) criar skill para captura.
   - Recomendação: A até existir dor real; B/C apenas se repetição justificar.
2. **Integração externa no código** — hoje nenhuma é usada.
   - Opções: A) manter nenhuma; B) notificação via GitHub/Slack; C) integração de monitoramento.
   - Recomendação: A até existir necessidade; B/C com decisão registrada.
3. **Verificação automatizada de skill-loading** — hoje é processo.
   - Opções: A) manter revisão humana; B) checklist no PR; C) teste de template.
   - Recomendação: B primeiro, C se a repetição justificar.

## 9. O que esta página não cobre

- Conteúdo integral das skills: ler cada `SKILL.md`.
- Skills cloud gerenciadas na conta do usuário.
- Implementação de integração.
- Pipeline de CI/CD detalhado: ver `pipeline-ci-cd.md`.