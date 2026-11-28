# Verificação de skills e integrações — planejamento

> Pergunta: "como verificar que as skills do agente e as integrações do Freebuff estão documentadas, carregadas e usadas corretamente?"
> Público: Desenvolvimento, arquitetura e agentes
> Dono: `wyd-ops` (organização) + arquitetura
> Base: `AGENTS.md` §5, §7 e §9; `wyd-ops/docs/arquitetura/ia-e-skills.md`; `wyd-ops/docs/github-mapping.md`; `wyd-ops/docs/specs/requisitos.md`; `wyd-ops/auditoria-de-organizacao.md`
> Revisado em: 2026-11-28

**Estado:** proposto (2026-11-26; contrato de skills verificado; guia de uso em 2026-11-28).
Planejamento; nenhuma skill ou integração de produção foi alterada. A verificação automatizada do
contrato está em `wyd-docs/tests/test_skill_contract.py`; o passo a passo de carga, uso e registro
está em `ia-e-skills.md` §4.

## 1. Escopo e dono

Este documento planeja a verificação de skills e integrações. Ele não implementa skill, não conecta integração e não altera configuração do Freebuff. O dono da organização é `wyd-ops`; a arquitetura é dona da regra de uso. A lista fechada e o dono do valor são de `template-de-issue.md` §3; esta página apenas inventaria onde cada skill existe e como é verificada.

## 2. Inventário de skills

As doze skills da lista fechada, com a mesma grafia do template:

| Skill | Onde vive | Quando carregar | Verificação |
|---|---|---|---|
| `auditar-planejamento` | `.agents/skills/auditar-planejamento/SKILL.md` | antes de marcar `aceito`, depois de lote, número divergente | skill carregada antes de escrever; achados por severidade |
| `documentation-writer` | conta do usuário (cloud skill) | redação e revisão técnica | carga via `cloud_read_skill` quando aplicável |
| `escrever-migration` | `planejamento-1.0.0/wyd-db/skills/execution/escrever-migration/SKILL.md` + cloud skill | arquivo da série `0001`–`0034` e a página dele | skill carregada antes de escrever; série aplicada do zero |
| `finishing-a-development-branch` | conta do usuário (cloud skill) | fechar branch de implementação | carga via `cloud_read_skill`; checklist de merge |
| `frontend-design` | conta do usuário (cloud skill) | UI nova ou reformulação visual | carga via `cloud_read_skill` quando aplicável |
| `github-actions-docs` | conta do usuário (cloud skill) | workflow, action, permissão, cache ou OIDC | carga via `cloud_read_skill`; workflow revisado contra a doc oficial |
| `implementar-issue` | `planejamento-1.0.0/wyd-db/skills/execution/implementar-issue/SKILL.md` + cloud skill | executar issue com DoD acordado de ponta a ponta | skill carregada antes de escrever; PR com checks verdes |
| `planejar-sistema` | `.agents/skills/planejar-sistema/SKILL.md` | abrir/fechar sistema, contrato, requisito, roadmap | skill carregada antes de escrever |
| `python-testing-patterns` | conta do usuário (cloud skill) | teste Python, fixture, mock ou cobertura | carga via `cloud_read_skill`; teste e fixture revisados |
| `revisar-documentacao` | `planejamento-1.0.0/wyd-db/skills/review/revisar-documentacao/SKILL.md` + cloud skill | revisar página contra cabeçalho, marcas e o que reprova | skill carregada antes de escrever; página revisada |
| `skill-creator` | conta do usuário (cloud skill) | criar, revisar ou medir skill | carga via `cloud_read_skill`; skill avaliada antes de entrar na lista |
| `writing-plans` | conta do usuário (cloud skill) | tarefa multi-etapa antes de tocar código | carga via `cloud_read_skill`; plano com tarefas e verificação |

Regra: o agente carrega a skill relevante **antes** de escrever, não depois. Issue templates têm campo
"Skills que o agente deve carregar". Como carregar cada origem (disco ou cloud) e o que registrar no
PR: `ia-e-skills.md` §4.1 e §4.4.

## 3. Matriz de skill-loading por trabalho

| Trabalho | Skill mínima | Quando é obrigatória |
|---|---|---|
| Abrir ou fechar sistema | `planejar-sistema` | antes de escrever ou alterar planejamento |
| Auditar página ou decisão | `auditar-planejamento` | antes de declarar `aceito` |
| Redação de documentação | `documentation-writer` | ao redigir ou revisar docs |
| Decisão de arquitetura ou fronteira | `planejar-sistema` + revisão humana | antes de ADR ou mudança de contrato |
| Criação ou revisão de skill | `skill-creator` | antes de alterar `.agents/skills/` |
| Trabalho de UI | `frontend-design` | ao criar ou reformular interface |
| Workflow ou action | `github-actions-docs` | antes de editar `.github/workflows` ou `.github/actions` |
| Teste Python, fixture ou gate | `python-testing-patterns` | antes de escrever teste ou fixture |
| Arquivo da série de migrations | `escrever-migration` | antes de tocar `crates/wyd-db/migrations/` |
| Execução de issue aprovada | `implementar-issue` | antes de abrir a branch da issue |
| Revisão de página do `wyd-db` | `revisar-documentacao` | antes de declarar a página revisada |
| Plano multi-etapa | `writing-plans` | antes de implementação |
| Fechamento de branch | `finishing-a-development-branch` | antes de merge ou limpeza da branch |

## 4. Integrações (Freebuff)

| Integração | Estado hoje | Uso planejado |
|---|---|---|
| GitHub | contas do mantenedor listadas no prompt inicial | notificação ou automação futura, com decisão registrada |
| Slack | conta do mantenedor listada no prompt inicial | notificação ou automação futura, com decisão registrada |
| Outras integrações | nenhuma usada no código | descobrir via `search_integration_tools` quando uma frente precisar |

Regra: segredos só via secret do GitHub ou `.env` local ignorado. Nenhum token, DSN, hash, senha ou PIN aparece em documento, log, issue ou chat.

### 4.1 Servidores MCP

Servidor MCP não é integração: é ferramenta do agente, fora do repositório, e não entra no CI.

| Servidor | Estado hoje | Uso planejado |
|---|---|---|
| `sequential-thinking` | disponível no cliente; nada configurado no repositório | decompor tarefa multi-etapa antes de decidir |
| `server-memory` | disponível no cliente; grafo local do mantenedor | rascunho de contexto entre sessões, nunca fonte de verdade |

Quem não tem esses servidores roda `make gate` do mesmo jeito: nada no projeto depende deles.
Inventário e regra de uso em `ia-e-skills.md` §3; configuração do cliente, exemplo de chamada de cada
servidor e formato do registro no PR em `ia-e-skills.md` §4.2–§4.4.

## 5. Verificação mínima

| Item | Como verificar | Dono |
|---|---|---|
| skill carregada antes de escrever | revisão de fluxo e issue template; passo a passo em `ia-e-skills.md` §4.1 | `wyd-ops` |
| skill documentada | `ia-e-skills.md` e este documento | `wyd-ops` |
| campo de skill-loading nos templates | gate `skills`: igualdade entre `template-de-issue.md` §3 e o campo `skill_principal` dos 4 formulários de `.github/ISSUE_TEMPLATE/` | `wyd-ops` |
| integração registrada quando usada | `decisoes-registradas.md` | arquitetura |
| segredo fora do repositório | varredura de segredo | `wyd-ops` |
| cloud skill disponível | `cloud_read_skill` quando aplicável | agente |
| servidor MCP disponível | `search_mcp_tools` antes de usar; inventário em `ia-e-skills.md` §3 | `wyd-ops` |

## 6. Anti-padrões proibidos

- Escrever antes de carregar a skill relevante.
- Tratar skill como substituto de decisão do dono.
- Conectar integração sem registrar a decisão.
- Colocar token, DSN, hash, senha ou PIN em documento, log, issue ou chat.
- Usar skill cloud sem verificar disponibilidade.
- Criar skill nova sem necessidade real do projeto.
- Tratar saída de MCP como fonte de verdade sem documento dono.

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
3. **Verificação automatizada de skill-loading** — **C implementado em 2026-11-26** para o contrato verificável: o gate `skills` lê a lista da §3 do template, compara com o campo `skill_principal` dos quatro formulários nas duas direções e usa esses nomes para validar as páginas; `wyd-docs/tests/test_skill_contract.py` cobre o contrato e as skills em disco; o carregamento em si continua sendo ato do agente e é cobrado pelo checklist do PR.
   - Opções: A) manter revisão humana; B) checklist no PR; C) teste de template.
   - Recomendação: B + C; C já cobre a lista e os formulários, B cobre a execução.

## 9. O que esta página não cobre

- Conteúdo integral das skills: ler cada `SKILL.md`.
- Skills cloud gerenciadas na conta do usuário.
- Implementação de integração.
- Pipeline de CI/CD detalhado: ver `pipeline-ci-cd.md`.
