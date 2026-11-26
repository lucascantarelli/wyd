# Gestão do GitHub, CI e ambientes — proposta

> **Estado: aprovada e executada em 2026-11-19; board do `wyd-db` fechado em 2026-11-22 (§10.3).**
> §10 registra o que foi criado e o que ficou pendente. As fontes externas foram consultadas em
> 2026-11-19 e estão listadas em §8; plano gratuito muda sem aviso, então confira antes de executar.
>
> Este documento pertence a `wyd-ops` porque descreve operação do repositório e dos ambientes. Ele
> não decide arquitetura de jogo nem substitui `planejamento-1.0.0/wyd-1.0.0.md` §4.

---

## 1. Princípios

1. **Repositório público e custo zero.** Nenhum recurso pago, nenhum cartão de crédito.
2. **Rastro antes de automação.** Issue, milestone e board só existem se apontarem para documento
   com DoD verificável.
3. **CI em runner hospedado.** Em repositório público, runner padrão é gratuito e ilimitado; runner
   self-hosted é risco, não economia (§6).
4. **Free tier é homologação.** Nenhum serviço gratuito serve produção de jogo: hiberna, expira ou
   não tem compute (§7).
5. **Sem segredo em repositório.** Secret só em GitHub Secrets ou `.env` local ignorado.

---

## 2. O que existe hoje (medido em 2026-11-19)

| Item | Estado | Fonte |
|---|---|---|
| Visibilidade | público | API do GitHub |
| Branch padrão | `develop` | API do GitHub |
| Issues | habilitadas, **40 abertas** (#82–#121) | API do GitHub, 2026-11-22 |
| Milestones | **6** (`M0 — Fundação` a `M5 — Operação`, nº 1–6) | API do GitHub, 2026-11-19; criação em §10 |
| Projects do repositório | **habilitado** (`has_projects: true`), project `WYD 1.0.0` nº 10 ligado ao repositório, 40 itens com `Sistema`, `Fase`, `Prioridade`, `Marco` e `Status` preenchidos, 5 views com filtro | API do GitHub, 2026-11-22; criação em §10, fechamento em §10.3 |
| Projects da conta | o `WYD 1.0.0` (§10) e 1 de outro assunto (“GP-100 Pipeline”) | API do GitHub |
| Merge | apenas squash (merge commit e rebase não permitidos); branch apagada no merge | API do GitHub |
| Segurança | secret scanning e Dependabot ativos | API do GitHub |
| CI | 2 workflows: `Full Workflow` (1 job `Get Info`, **verde** e sem build/teste/cobertura; grava `version`/`commit` vazios — §2.1) e `docs` (gate do site + publicação) | `.github/workflows/` |
| Actions compostas | 1 (`docs-check`, usada pelo workflow `docs`), além do `.gitkeep` | `git ls-files .github`, 2026-11-19 |
| Templates | 4 formulários de issue (`bug`, `planejamento`, `decisao`, `tarefa`), `config.yml` e PR template (§10) | `git ls-files .github`, 2026-11-19 |

O job **não** falha: ele conclui `success` e grava saída vazia (§2.1). O `run:` contém `$(0.0.0-<sha>)`
e `$(<sha>)`, que o shell tenta executar como comando; a falha fica dentro
da substituição de comando, não no status do passo. Além disso, não há `Cargo.toml` na raiz para um
job Rust compilar.

### 2.1 Baseline medido do CI `full-workflow` (`V`, 2026-11-19; somente leitura)

Medido por consulta de leitura às execuções do workflow e ao log do job. Nenhum workflow foi
acionado, editado ou reexecutado por esta coleta.

| Campo | Valor |
|---|---|
| Workflow | `Full Workflow` — `.github/workflows/full-workflow.yml` |
| Execução | [#14](https://github.com/lucascantarelli/wyd/actions/runs/44505419869), ID `44505419869`, evento `push`, `develop`, SHA `aedd1f7338fdfd7125c0c25f4c47255ca88d8672` |
| Janela | `created_at` `2026-11-19T10:24:48Z`, `updated_at` `10:24:54Z` |
| Conclusão da execução | `success` |
| Job | `Get Info` [ID `135495629487`](https://github.com/lucascantarelli/wyd/actions/runs/44505419869/job/135495629487), `success`; os passos `Get version` e `Get commit` também concluíram `success` |
| Execuções com conclusão `failure` | **0** para este workflow em todo o repositório (filtro `status=failure`) |

**Mensagem exata do shell (log do job, `2026-11-19T10:24:52Z`):**

```text
/home/runner/work/_temp/679c9341-bb0f-4f0d-9866-62bce7758d08.sh: line 1: 0.0.0-aedd1f7338fdfd7125c0c25f4c47255ca88d8672: command not found
/home/runner/work/_temp/4f5a1ba4-f2b6-4b95-a770-1c0f224ee838.sh: line 1: aedd1f7338fdfd7125c0c25f4c47255ca88d8672: command not found
```

**Causa (`V`, medida):** o `${{ ... }}` das linhas 40 e 44 é interpolado pelo GitHub **antes** do
shell, então o `run:` chega ao bash como `echo "version=$(0.0.0-<sha>)"` e `echo "commit=$(<sha>)"`.
O `$(...)` restante é substituição de comando: o bash tenta executar `0.0.0-<sha>` e `<sha>` como
programas e falha com `command not found`. A falha ocorre **dentro** da substituição, então o `echo`
executa mesmo assim e retorna 0 — o passo fica `success`, o job fica verde e o valor gravado em
`$GITHUB_OUTPUT` sai vazio (`version=`, `commit=`).

**Efeito:** não há job vermelho para consertar; há **gate verde que não publica metadado**. O
`version`/`commit` que um job futuro consumiria chega vazio, e o defeito não aparece em status check
nenhum. Um gate que não pode falhar não é gate. Continua verdade que não existe gate de código,
build nem cobertura: só `crates/wyd-db` tem `Cargo.toml` (com `dsn`, desde 2026-11-24) e não há workspace
na raiz.

**O que esta medição não decide:** corrigir a interpolação do `run:` e substituir o stub pela
arquitetura de §4 são a decisão 6 de §8 e a Fase 4 do panorama. O workflow não foi alterado.

**Alinhamento com o panorama (`V`, 2026-11-19):** `planejamento-1.0.0/wyd-1.0.0.md` §3 (`D3`) afirmava
que o job "falha"; a medição acima mostra conclusão `success`. O panorama foi corrigido nesta data e
cita esta seção como dono do detalhe, junto da linha `CI` de §1.4 e do passo 5 de §6.

---

## 3. Modelo de trabalho

### 3.1 Três níveis

| Nível | Representa | Regra |
|---|---|---|
| **Milestone** | entrega com começo e fim (fase ou release) | agrupa issues; não é backlog |
| **Issue** | unidade de trabalho com DoD verificável | uma issue por PR; sem DoD não entra |
| **PR** | mudança revisável | liga a issue; squash; gate verde |

Épico grande usa **sub-issue** (até 100 por pai, 8 níveis) e **dependência** entre issues. Não use
checklist em texto para substituir hierarquia: checklist não aparece no board como item.

### 3.2 Tipos e campos de issue

O GitHub liberou **issue types e fields para contas pessoais em 2026-11-11** (GA). Como o repositório
é de conta pessoal, isso está disponível sem organização.

**Proposta em aberto:**

| Recurso | Valores |
|---|---|
| Tipo | `Bug`, `Task`, `Feature` (defaults; não criar tipo novo sem necessidade) |
| Campo `Priority` | `P0` bloqueia release · `P1` próxima · `P2` depois · `P3` algum dia |
| Campo `Effort` | `P` pequeno · `M` médio · `G` grande |

Tipo responde “que natureza”; label responde “que área”. Não use label para repetir tipo.

### 3.3 Labels (lista fechada)

| Grupo | Labels | Uso |
|---|---|---|
| Área | `area/wyd-protocol`, `area/wyd-net`, `area/wyd-core`, `area/wyd-server`, `area/wyd-sim`, `area/wyd-db`, `area/wyd-admin`, `area/wyd-portal`, `area/wyd-launcher`, `area/wyd-client`, `area/wyd-bots`, `area/wyd-docs`, `area/wyd-ops` | uma por issue |
| Estado | `estado/bloqueada`, `estado/aguardando-decisao`, `estado/em-revisao` | no máximo uma |
| Origem | `origem/legado` | afirmação `H` vinda do `deprecated` |
| Contribuição | `good first issue`, `help wanted` | labels padrão, para repositório público |
| Risco | `risco/seguranca`, `risco/dados` | quando o PR exige revisão extra |

Não crie `bug`, `enhancement` e `documentation` como label se usar tipo: vira duas taxonomias para
a mesma coisa.

### 3.4 Milestones

| Milestone | Escopo | Critério de fechamento |
|---|---|---|
| `M0 — Fundação` | `AGENTS.md`, panorama, memória, skills, proposta de gestão | agente novo entende o projeto |
| `M1 — Planejamento` | fechar lacunas e páginas do `wyd-db` | gate da Fase 2 do panorama |
| `M2 — wyd-db` | implementar a série e o crate | corte vertical + suíte verde |
| `M3 — Sistemas` | planejar e implementar os demais | cada sistema com gate próprio |
| `M4 — 1.0.0` | integração, E2E, carga | metas da `1.0.0` |
| `M5 — Operação` | CI/CD, monitoramento, ambientes | deploy reproduzível com rollback |

Milestone é marco, não data. Prazo por dependência, como em `lacunas.md` §0.2.

### 3.5 Project

Um project `WYD 1.0.0` com campos:

| Campo | Tipo | Valores |
|---|---|---|
| `Status` | single select | `Backlog`, `Pronto`, `Em andamento`, `Bloqueado`, `Em revisão`, `Done` |
| `Sistema` | single select | os 13 sistemas |
| `Fase` | single select | `0`–`5` do panorama |
| `Prioridade` | single select | `P0`–`P3` |
| `Marco` | single select | `M0`–`M5` |

Views: **Roadmap** (por milestone), **Por sistema**, **Bloqueados**, **Minhas issues** — a execução
manteve a view padrão renomeada como **Fila** (`-status:Done`), e o estado medido está em §10.3. Auto-add
filtrado por label `area/*`. O número de automações embutidas depende do plano: conte com uma e
confirme o limite antes de depender de várias.

### 3.6 Templates

Criar em `.github/ISSUE_TEMPLATE/`:

| Arquivo | Para quê | Campos obrigatórios |
|---|---|---|
| `bug.yml` | defeito | o que aconteceu, esperado, como reproduzir, evidência, sistema |
| `planejamento.yml` | página/documento novo | sistema, pergunta que responde, fonte, gate |
| `decisao.yml` | lacuna ou ADR | ID, dono, opções, impacto, prazo/marco |
| `tarefa.yml` | implementação | entregável, DoD, dependências, sistema |
| `config.yml` | links | documentação, panorama, como contribuir |

`planejamento.yml` e `decisao.yml` são o diferencial deste projeto: impedem issue sem dono ou sem
fonte. Todo formulário pede o **sistema** e o **marco**.

PR template: issue ligada, o que mudou, como foi provado (comando + saída), cobertura, doc atualizada
no mesmo PR, sem segredo, sem código fora do escopo.

### 3.7 Branches e revisão

| Regra | Valor |
|---|---|
| Branch de trabalho | `tipo/escopo-curto` a partir de `develop` |
| `develop` | PR obrigatório, status check obrigatório, squash |
| `main` | só release promovida de `develop` |
| Histórico | squash evita merge commit; mensagem em Conventional Commits |
| Review | uma aprovação; autor não aprova o próprio PR |

Rulesets e branch protection estão disponíveis em repositório público no plano Free. Não habilite
“require approvals” sem outro revisor disponível: travaria o próprio dono.

---

## 4. CI/CD

### 4.1 Arquitetura proposta

```text
.github/
├── actions/
│   ├── setup-rust/action.yml       # toolchain + cache
│   ├── setup-node/action.yml       # node + npm cache
│   ├── docs-check/action.yml       # preparar.py + mkdocs build + verificar.py
│   └── coverage-gate/action.yml    # lê cobertura e falha abaixo de 85%
└── workflows/
    ├── ci.yml                      # orquestra em estágios
    ├── _docs.yml                   # workflow_call
    ├── _rust.yml                   # workflow_call
    ├── _portal.yml                 # workflow_call
    ├── _security.yml               # workflow_call
    └── release.yml                 # tag
```

Regras:

- Workflow reutilizável declara `on: workflow_call` com `inputs` e `secrets` explícitos. Nada de
  variável global implícita.
- Action composta usa `using: composite` e `shell: bash` em cada `run`.
- Action de terceiro é fixada por **commit SHA**, não por tag móvel. Dependabot atualiza.
- `permissions` no topo é `contents: read`; cada job pede só o que precisa.
- Deploy usa **OIDC** e environment com revisor; nunca chave de longa duração em secret.
- `pull_request_target` não roda código do fork com secret. Para PR de fork, use `pull_request` e
  nenhum secret.

### 4.2 Estágios

| Estágio | Jobs | Falha se |
|---|---|---|
| 1. Qualidade | `fmt`, `clippy -D warnings`, markdown/link, `cargo-deny`, Trivy | qualquer aviso ou advisory |
| 2. Teste | unitário, contrato, integração com Postgres 17 real | teste falha, `SKIP` ou cobertura < 85% |
| 3. Portal | `tsc --noEmit`, Vitest, Playwright | tipo, teste ou E2E falha |
| 4. Build | imagens Docker, binários | build quebra |
| 5. Gate | agrega os anteriores; release/deploy | estágio anterior não verde |

Enquanto não houver código, os estágios 1–4 rodam só o que existe (documentação). Não crie job que
sempre passa: gate que não pode falhar não é gate.

### 4.3 Exemplo mínimo de action composta

```yaml
# .github/actions/docs-check/action.yml
name: docs-check
description: Monta, constrói e verifica o site MkDocs 2.0.
runs:
  using: composite
  steps:
    - name: Instalar dependências
      shell: bash
      run: pip install -r wyd-docs/requirements.txt
    - name: Montar fontes
      shell: bash
      working-directory: wyd-docs
      run: python preparar.py
    - name: Construir site
      shell: bash
      working-directory: wyd-docs
      run: mkdocs build --dir .build/docs --output site
    - name: Verificar
      shell: bash
      working-directory: wyd-docs
      run: python verificar.py site
```

Exemplo de workflow reutilizável:

O arquivo citado no exemplo (`.github/workflows/_docs.yml`) **não existe** no repositório: é a forma
proposta, não implementada.

```yaml
# .github/workflows/_docs.yml
on:
  workflow_call:
    inputs:
      python-version:
        type: string
        default: "3.13"
jobs:
  docs:
    runs-on: ubuntu-latest
    permissions:
      contents: read
    steps:
      - uses: actions/checkout@<sha>
      - uses: ./.github/actions/docs-check
```

Versões de action e de linguagem são resolvidas na implementação e travadas; não invente tag aqui.

---

## 5. Custo no GitHub

| Uso | Custo | Fonte |
|---|---|---|
| Runner padrão em repositório público | **gratuito e ilimitado** | GitHub Docs, 2026-11-19 |
| Runner self-hosted | sem cobrança de minutos; você paga a máquina | GitHub Docs |
| Issues, Projects, Milestones | incluídos no plano Free | github.com/pricing, 2026-11-19 |
| Actions em repositório privado | 2.000 min/mês no Free, depois pago | github.com/pricing |

Conclusão: o CI inteiro cabe em $0 **se** ficar em runner hospedado e repositório público.

---

## 6. Runner self-hosted — análise

| Dimensão | Self-hosted | Hospedado |
|---|---|---|
| Custo de minutos | $0 | $0 em repo público |
| Custo real | máquina, energia, manutenção, patch de SO | nenhum |
| Performance | depende do hardware; pode ser melhor | VM nova, 2–4 vCPU típicas |
| Isolamento | **não efêmero por padrão**; compromisso persiste | VM limpa e descartável |
| Segurança em repo público | **desaconselhado**: qualquer PR pode executar código e comprometer o runner, secrets e `GITHUB_TOKEN` | isolado por job |
| Cache entre jobs | possível, mas vira vetor de persistência | cache gerenciado |
| Complexidade | registro, labels, atualização, rede, disco | zero |

**Recomendação:** não usar self-hosted enquanto o repositório for público e receber PR de fork. Se
um dia houver necessidade real (hardware específico, teste de cliente Windows), usar **somente** em
job confiável (`push` de branch protegida, `schedule`, `workflow_dispatch`), com runner efêmero,
rede isolada, sem secret de produção e label restrita. A economia é zero — o ganho teria que ser
hardware, não minuto.

Fonte: GitHub Docs, *Secure use reference* e *Self-hosted runners* (2026-11-19).

---

## 7. Ambientes e hospedagem gratuita

### 7.1 Matriz de ambientes

| Ambiente | Onde | Dado | Serve para |
|---|---|---|---|
| Local | Docker Compose na máquina | sintético | desenvolvimento, migrations, E2E |
| CI | runner hospedado + serviço Postgres | efêmero | gate de PR |
| Homologação | free tier (portal + banco) | sintético | demonstrar portal, nunca jogo |
| Produção | máquina própria/VPS + Postgres gerenciado ou dedicado | real | jogo |

### 7.2 Free tiers verificados

| Serviço | O que dá grátis | Cartão | Limite que importa | Veredito |
|---|---|---|---|---|
| **Neon** (Postgres) | 1 GB/projeto, 20 GB/conta, 100 CU-h/mês | não | escala a zero após 5 min; suspende ao estourar CU-h | melhor opção para banco de homologação |
| **Supabase** (Postgres) | 500 MB, 2 projetos ativos | não | pausa após 1 semana de inatividade; sem backup no Free | alternativa de homologação |
| **Render** (web + Postgres) | 750 h/mês de web service; Postgres 1 GB | não | web dorme após 15 min; **Postgres Free é apagado 30 dias após criação** | portal de demonstração; nunca banco |
| **Vercel** (portal) | Hobby $0 | não | limites de uso e termos de uso comercial | preview de portal; conferir termos |
| **Railway** | $1 de crédito/mês; trial de $5 | não | sem free tier real; volume apagado ao fim | não conta como gratuito |
| **Fly.io** | $1 de trial por 7 dias | não | **sem free tier**; fatura < $5 é dispensada | não conta como gratuito |
| **Koyeb** | 1 instância, 5 h de compute/mês | não | dorme após 5 min | inviável para serviço |
| **Cloudflare Pages/Workers** | plano Free | não | runtime Workers; não roda binário Rust TCP | site estático/documentação, não jogo |

### 7.3 O que **não** cabe em free tier

O servidor de jogo precisa de processo persistente, porta TCP `8281` (CPSock), estado em memória e
latência baixa. Nenhum free tier acima oferece isso: todos dormem, expiram ou são serverless. Banco
de produção também não cabe: 1 GB e scale-to-zero não sustentam personagens, auditoria e catálogo.

**Proposta em aberto:** produção só depois de decisão de custo — máquina própria, VPS barata ou crédito de
provedor. Até lá, free tier é vitrine de portal e banco de homologação.

### 7.4 Documentação e portal

- **MkDocs:** publicar em GitHub Pages ou Cloudflare Pages (grátis, sem runtime). O site é estático;
  não precisa de servidor.
- **Portal Next.js:** Vercel Hobby para preview; qualquer uso com loja/doação exige conferir termos.
  Alternativa: Render free, aceitando o sleep de 15 min.

---

## 8. Decisões que dependem de aprovação

| # | Decisão | Recomendação |
|---|---|---|
| 1 | Habilitar Projects do repositório e criar `WYD 1.0.0` | sim |
| 2 | Adotar issue types + fields e a lista de labels de §3.3 | sim, sem duplicar tipo em label |
| 3 | Criar os 6 milestones `M0`–`M5` | sim |
| 4 | Criar os 4 formulários de issue + PR template | sim |
| 5 | Ruleset de `develop` com PR e status check | sim, sem exigir revisor externo |
| 6 | Substituir o CI stub pela arquitetura de §4 | sim, na Fase 4 |
| 7 | Usar runner hospedado e proibir self-hosted em PR | sim |
| 8 | Homologar portal no Vercel Hobby e banco no Neon | sim, com dado sintético |
| 9 | Definir produção só após decisão de custo | sim, registrar como risco |
| 10 | Migrar as 40 issues do `wyd-db` para o GitHub | **executada em 2026-11-22:** páginas com DoD e issues #82–#121 no board |

As ações de estrutura 1–4, 7 e 9 foram executadas em 2026-11-19; a ação 10, em 2026-11-22. A ação 6
(CI real) continua pendente por decisão de fase. Criar recurso novo no GitHub exige pedido explícito.

---

## 9. Fontes

Consultadas em **2026-11-19**:

- GitHub Actions billing — <https://docs.github.com/en/billing/concepts/product-billing/github-actions>
- GitHub-hosted runners (gratuito e ilimitado em repo público) —
  <https://docs.github.com/en/actions/reference/runners/github-hosted-runners>
- Secure use reference (self-hosted em repo público) —
  <https://docs.github.com/en/actions/reference/security/secure-use>
- Self-hosted runners —
  <https://docs.github.com/en/actions/concepts/runners/self-hosted-runners>
- Planos e preços (Issues, Projects, Milestones no Free) — <https://github.com/pricing>
- Issue types e fields para contas pessoais (GA 2026-11-11) —
  <https://github.blog/changelog/2026-11-11-issue-types-and-fields-are-available-for-personal-accounts>
- Sub-issues — <https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/adding-sub-issues>
- Boas práticas de Projects —
  <https://docs.github.com/en/issues/planning-and-tracking-with-projects/learning-about-projects/best-practices-for-projects>
- Sintaxe de issue forms —
  <https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms>
- Rulesets em repositório público no Free —
  <https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets>
- Render free — <https://render.com/docs/free>
- Fly.io pricing (sem free tier) — <https://fly.io/docs/about/pricing>
- Comparativo de Postgres gratuito (Neon, Supabase, Render) —
  <https://fastero.com/blog/free-postgres-database>
- Limites de free tiers (Vercel, Render, Neon, Supabase, Cloudflare) —
  <https://codingplanlimits.com/free-tiers>

---

## 10. Execução (2026-11-19)

Aprovada pelo time nesta data. O que foi criado no GitHub, por API e `gh`:

| Item | Estado | Detalhe |
|---|---|---|
| Projects do repositório | habilitado | `has_projects: true` |
| Project `WYD 1.0.0` | criado | `https://github.com/users/lucascantarelli/projects/10` (nº 10) |
| Campos do project | criados | `Sistema` (13 opções), `Fase` (0–5), `Prioridade` (P0–P3), `Marco` (M0–M5); `Status` é o padrão do GitHub |
| Labels `area/*` | 13 criadas | uma por sistema |
| Labels `estado/*` | 3 criadas | `bloqueada`, `aguardando-decisao`, `em-revisao` |
| Outras labels | 3 criadas | `origem/legado`, `risco/seguranca`, `risco/dados` |
| Milestones | 13 criados | `M0`–`M5` (nº 1–6) + fases `F1`–`F7` (nº 7–13), criadas em 2026-11-22 para as 40 issues |
| Issues `WYDDB` | 40 criadas | #82–#121, uma por página com DoD; labels `area/wyd-db` e milestone de fase |
| Formulários de issue | 4 criados | `bug`, `planejamento`, `decisao`, `tarefa` + `config.yml` |
| Template de PR | criado | `.github/pull_request_template.md` |
| Ruleset de `develop` | ativo | PR obrigatório, sem deleção, sem force-push, thread resolvida |
| Ruleset de `main` | ativo | mesmas regras |
| Workflow de documentação | criado | `.github/workflows/docs.yml` + `.github/actions/docs-check/` |

### 10.1 Desvios conscientes

| Desvio | Motivo | Quando corrigir |
|---|---|---|
| Sem *required status check* nos rulesets | o `Full Workflow` é verde sem build (§2.1) e o `docs` não avalia código | Fase 4, ao substituir o CI stub |
| Sem aprovação obrigatória de revisor | o repositório tem um mantenedor; exigir aprovação travaria o próprio dono | quando houver segundo revisor |
| Sem views customizadas no project | a API cria view sem filtro; view sem filtro não é a view prometida em §3.5 | **corrigido em 2026-11-22**: `createProjectV2View` cria sem filtro e `updateProjectV2View` aplica (§10.3) |
| Labels padrão do GitHub mantidas | remover `bug`/`enhancement` é destrutivo e o histórico pode depender delas | decisão do time; a regra de não duplicar tipo vale para issue nova |
| 40 issues criadas sem o formulário | o roadmap do `wyd-db` já tinha página com DoD; o formulário não aceita corpo tão longo nem milestone de fase | manter o formulário para issue nova; as 40 são migração de página existente |

### 10.2 O que continua pendente

- CI real, que segue stub; não é bloqueio das 40 issues de planejamento.
- Campo de data no project: sem ele a view `Roadmap` não plota linha do tempo (§10.3).

### 10.3 Fechamento do board do `wyd-db` (`V`, 2026-11-22)

O project `WYD 1.0.0` existia desde 2026-11-19 com os 40 itens e **nenhum campo preenchido**, sem
vínculo com o repositório, sem view útil e sem `Status` de seis estados. O que foi escrito nele, e o
estado medido no fim da execução:

| Item | Estado final | Como verificar |
|---|---|---|
| Vínculo com o repositório | project ligado a `lucascantarelli/wyd` (`linkProjectV2ToRepository`) | aba **Projects** do repositório |
| Itens | 40 (#82–#121), um por página `WYDDB-nnn` | `gh project item-list 10 --owner @me` |
| Campos preenchidos | **40 de 40** com `Sistema`, `Fase`, `Prioridade`, `Marco` e `Status`; 0 incompletos | a mesma consulta, lida campo por campo |
| `Sistema` | `wyd-db` nos 40 | — |
| `Fase` | `4 — Implementação` nos 40: as issues implementam o crate, o fechamento do planejamento é a Fase 2 de `wyd-1.0.0.md` §4 | `wyd-1.0.0.md` §4 |
| `Marco` | `M2` nos 40 | §3.4 |
| `Prioridade` | `P1` **15** (`001`–`015`), `P2` **11** (`016`–`026`), `P3` **14** (`027`–`040`) | página de cada issue; `roadmap/index.md` §1 |
| `Status` | `Pronto` **1** (`WYDDB-001`, sem dependência), `Backlog` **25**, `Bloqueado` **14** (`027`–`040`, os bloqueios de `roadmap/index.md` §2) | — |
| Opções de `Status` | as seis de §3.5: `Backlog`, `Pronto`, `Em andamento`, `Bloqueado`, `Em revisão`, `Done` | `gh project field-list 10 --owner @me` |
| Views | **5, todas com filtro**: `Fila` (`-status:Done`, é a view padrão renomeada), `Por sistema` (`sistema:wyd-db`), `Bloqueados` (`status:Bloqueado`), `Minhas issues` (`assignee:@me`) e `Roadmap` (`-status:Done`) | `gh api graphql` na consulta do project |
| Descrição e README do project | preenchidos, cada item apontando o documento dono do fato | página do project |
| Tipo de issue | `Task` nas 40 | `gh api repos/lucascantarelli/wyd/issues/82 -q .type` |
| Dependências | **63** vínculos `blocked by`, derivados do bloco "Depende de" das 40 páginas; 39 issues têm ao menos um bloqueador | `gh api repos/lucascantarelli/wyd/issues/{n}/dependencies/blocked_by` |
| `estado/bloqueada` | 14 issues (`027`–`040`), as mesmas que `roadmap/index.md` §2 declara bloqueadas | `gh issue list --label estado/bloqueada` |
| Formulário `planejamento.yml` | ganhou o campo obrigatório `Marco`, que §3.6 exige e o arquivo não tinha | `.github/ISSUE_TEMPLATE/planejamento.yml` |

Três limites de API, medidos e não contornados:

1. **`createProjectV2View` não aceita `filter`** (`argumentNotAccepted`), embora `updateProjectV2View`
   aceite. Cada view filtrada custa duas chamadas: criar e depois filtrar.
2. **Agrupamento não é exposto:** `ProjectV2ViewConfigurationInput` só aceita `visibleFieldIds`, então
   a view "por sistema" é filtro, não agrupamento.
3. **`Roadmap` é layout de data:** não há campo de data no project, então a linha do tempo não plota
   nada. A view existe para quando houver data; a leitura por fase continua sendo o milestone, e por
   dependência, o vínculo `blocked by`.

O `Status` foi reescrito preservando o `id` de `Todo`, `In Progress` e `Done`; renomear com o mesmo
`id` não perde valor de item. Nenhum item tinha valor antes, e a ordem das seis opções é a de §3.5.

