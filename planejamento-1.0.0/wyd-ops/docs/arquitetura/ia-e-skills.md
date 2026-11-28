# IA, skills e integrações — organização

> Pergunta: "onde vivem os arquivos de IA, skills e integrações e como o agente deve usá-los?"
> Público: Desenvolvimento e agentes
> Dono: `wyd-ops` (organização) + arquitetura
> Base: `AGENTS.md`; `.agents/README.md`; `.agents/skills/*/SKILL.md`; `.freebuff/project-id`
> Revisado em: 2026-11-28

**Estado:** proposto (2026-11-25; servidores MCP inventariados em 2026-11-27; guia de uso em
2026-11-28).

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
| sete cloud skills do projeto | conforme o tipo de trabalho; carregar via `cloud_read_skill` quando aplicável | conta do usuário; a lista fechada e a matriz estão em `wyd-ops/docs/planos/template-de-issue.md` §3 |

Regra: o agente deve carregar a skill relevante **antes** de escrever, não depois. A lista fechada
(doze nomes) e o dono do valor são de `template-de-issue.md` §3; esta página só organiza as pastas e
não republica a lista. Issue templates têm campo "Skills que o agente deve carregar".

## 3. Integrações e servidores MCP (Freebuff)

Nenhuma integração externa (GitHub/Slack) é usada no código hoje; as contas listadas no prompt
inicial são do próprio mantenedor. Se uma frente precisar (ex.: notificação), usar `search_integration_tools`
e registrar a decisão em `decisoes-registradas.md`. Segredos só via secret do GitHub ou `.env` local
ignorado (AGENTS.md §7, §9).

Servidor MCP é ferramenta do agente, não integração do projeto: não tem configuração neste
repositório, não entra no CI e não recebe segredo. O cliente tem dois servidores disponíveis:

| Servidor | Ferramenta | Quando usar | Limite |
|---|---|---|---|
| `sequential-thinking` | `sequentialthinking` | decompor tarefa grande em passos revisáveis antes de decidir | a decisão aprovada vai para o documento dono, não para o histórico de pensamento |
| `server-memory` | grafo de entidades (`create_entities`, `search_nodes`, `open_nodes`) | rascunho de contexto entre sessões | nunca é fonte de verdade: o fato vai para `planejamento-1.0.0/` (`.agents/README.md`, regra 3) |

Regra: MCP acelera o trabalho e não substitui evidência. Conclusão obtida com MCP entra no
planejamento pela mesma porta de qualquer outra — documento dono, marca `V`/`H`/`P` e gate
(AGENTS.md §5.4).

## 4. Do zero: carregar a skill, usar o MCP e registrar no PR

Caminho de quem entra novo. O valor de cada campo continua com o dono (`template-de-issue.md` §3 para
a lista de skills, `.github/pull_request_template.md` para o registro); aqui está o uso.

### 4.1 Carregar a skill da issue

Exemplo real — a fatia `WYDOPS-F5`, em `planejamento-1.0.0/wyd-ops/docs/roadmap/WYDOPS-F5.md`:

| Campo da página | Valor |
|---|---|
| `skill_principal` | `github-actions-docs` |
| `skills_complementares` | `writing-plans` |

Ordem de execução:

1. ler o bloco `## Skills` da página ou do formulário da issue — é ele que manda, não a memória de
   quem executa;
2. carregar a principal **inteira**, antes do primeiro edit:
   - skill em disco: ler o `SKILL.md` e os recursos que ele cita —
     `.agents/skills/planejar-sistema/SKILL.md`,
     `planejamento-1.0.0/wyd-db/skills/execution/escrever-migration/SKILL.md`;
   - skill cloud: `cloud_read_skill` com o nome exato (`github-actions-docs`), sem procurar arquivo no
     disco — ela vive na conta, não no repositório;
3. só usar complementar quando a página declarar: aqui `writing-plans` entra porque fechar a fatia é
   plano de várias etapas;
4. editar. Skill lida depois do edit é skill não usada.

```bash
# skill em disco: o arquivo inteiro, não um trecho
cat .agents/skills/planejar-sistema/SKILL.md
cat planejamento-1.0.0/wyd-db/skills/execution/escrever-migration/SKILL.md
```

O gate não prova carga de skill: `wyd-docs/tests/test_skill_contract.py` cobra a lista, os formulários
e o bloco do PR; a prova de execução é o diff mais o registro da §4.4 (AGENTS.md §7, Etapa 2.1).

### 4.2 `sequential-thinking` — plano que se revisa

Configuração é do cliente, não do repositório: nada em `.freebuff/`, `.agents/` ou workflow. O
servidor de referência roda por `npx`, e o bloco abaixo serve a qualquer cliente MCP (no cliente do
Freebuff ele já aparece disponível — confirme com `search_mcp_tools` antes de configurar):

```json
{
  "mcpServers": {
    "sequential-thinking": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-sequential-thinking"]
    }
  }
}
```

No Windows, alguns clientes exigem `cmd` com
`args: ["/c", "npx", "-y", "@modelcontextprotocol/server-sequential-thinking"]` no lugar de `npx`
direto.

Uso: a ferramenta é `sequentialthinking`, um passo por chamada, com `thought`, `thoughtNumber`,
`totalThoughts` e `nextThoughtNeeded`. Para reconsiderar um passo, `isRevision` + `revisesThought`;
para explorar alternativa sem apagar a linha principal, `branchFromThought` + `branchId`.

Quando vale: trabalho com três ou mais passos dependentes — fechar fatia, decidir contrato, mexer em
workflow. Sequência típica para `WYDOPS-F5`:

1. escopo: o que a fatia entrega e o que fica fora, lido da página;
2. dono: qual documento guarda a decisão que sobrar;
3. prova: qual comando fecha o DoD;
4. revisão: o passo 2 apontou o dono errado → `isRevision: true`, `revisesThought: 2`.

Limite: o histórico de pensamento é rascunho da sessão. O plano aprovado vai para a página ou o PR;
nada em `planejamento-1.0.0/` cita "pensamento 3" como evidência.

### 4.3 `server-memory` — contexto entre sessões

Também é configuração do cliente. O grafo é um arquivo `.jsonl` local, fora do repositório:

```json
{
  "mcpServers": {
    "server-memory": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-memory"],
      "env": { "MEMORY_FILE_PATH": "<pasta-do-usuário>/wyd-memory.jsonl" }
    }
  }
}
```

`MEMORY_FILE_PATH` fica em pasta do usuário (`%LOCALAPPDATA%` no Windows, `~/.local/share` no Linux),
nunca em `planejamento-1.0.0/` nem em `.agents/memory/`: o arquivo é descartável e não versionado.

Ferramentas: `create_entities`, `add_observations`, `create_relations`, `search_nodes`, `open_nodes`
e `read_graph` (caro — só quando precisar do grafo todo).

Exemplo — o que a sessão seguinte precisa para retomar `WYDOPS-F5`:

```json
{
  "entities": [{
    "name": "WYDOPS-F5",
    "entityType": "fatia",
    "observations": [
      "skill_principal: github-actions-docs",
      "DoD: make gate verde e cobertura onde aplicável",
      "dono do estado: planejamento-1.0.0/wyd-ops/docs/roadmap/WYDOPS-F5.md"
    ]
  }]
}
```

Depois, `create_relations` liga `WYDOPS-F5` a `wyd-ops` com `relationType` em voz ativa
(`WYDOPS-F5 pertence a wyd-ops`). Na sessão seguinte, `open_nodes` com o nome exato devolve o mesmo
contexto; `search_nodes` serve quando só se lembra parte do nome.

Limite: observação não é fonte de verdade. Se a memória divergir da página, a página vence e a
observação é corrigida; token, DSN, hash de jogador ou senha nunca entram como observação.

### 4.4 Registro no PR

O template do PR tem a seção **Skills carregadas (e MCP/integrações)**; preencha os cinco campos.
`nenhum`/`nenhuma` é resposta válida — campo vazio não é. Exemplo do mesmo `WYDOPS-F5`:

```markdown
## Skills carregadas (e MCP/integrações)

- **Skill obrigatória da issue:** `github-actions-docs`
- **Complementares usadas:** `writing-plans` — plano da fatia antes do primeiro edit
- **MCP usado:** sequential-thinking — quatro passos de escopo, dono e prova; server-memory — nenhum
- **Integrações usadas:** GitHub (issue #N, PR #N, run do CI)
- **Como foi verificado:** `cloud_read_skill` antes do primeiro edit; `make gate` verde no run
```

Regras do registro:

- o nome da skill é o da lista fechada, com a mesma grafia do bloco `## Skills` da página;
- MCP é o servidor realmente chamado, não o que estava disponível;
- integração é a que foi tocada (GitHub, Slack, GHCR) — não a que existe na conta;
- "como foi verificado" nomeia comando ou ferramenta; "testei localmente" não é prova.

`test_pr_template_exige_registro_da_skill` garante que a seção e o campo "Skill obrigatória da issue"
continuam no template; o conteúdo é conferido por pessoa na revisão do PR.

### 4.5 Checklist de quem entra novo

- [ ] leu `AGENTS.md` §7, Etapa 2.1, e sabe que skill vem antes do edit;
- [ ] conferiu que as skills cloud respondem a `cloud_read_skill` e que as de disco abrem em
      `.agents/skills/` e `planejamento-1.0.0/wyd-db/skills/`;
- [ ] `search_mcp_tools` mostra `sequential-thinking` e `server-memory`; se faltar, configurou no
      cliente — não no repositório;
- [ ] rodou `make gate-fast` antes do primeiro push (o `pre-push` já o executa);
- [ ] preencheu a §4.4 no PR.

## 5. O que esta página não cobre

- Conteúdo das skills (ler o `SKILL.md` de cada uma).
- Skills cloud: gerenciadas na conta, não no repo.
- Lista fechada de nomes e o dono do valor: `template-de-issue.md` §3.
- Verificação do contrato: `verificacao-de-skills-e-integracoes.md`.
