# Mapeamento GitHub — labels, nomes e convenções

> Pergunta: "como nomear issue, branch, PR e label para que o projeto fique mapeado no GitHub?"
> Público: Desenvolvimento, operação e agentes
> Dono: `wyd-ops`
> Base: `.github/ISSUE_TEMPLATE/*`; `proposta-gestao-github-e-ambientes.md`; `AGENTS.md` §7, §9
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26). Convenção de nomes; a transição automática de estado está em
§2.2. O board e as 121 issues foram medidos em 2026-11-26; novas issues, labels ou views continuam
ato explícito e não são criadas por esta página.

## 0. Estado medido do repositório (2026-11-26)

Leitura somente de `lucascantarelli/wyd` pela integração GitHub: branch padrão `develop`, `has_issues`
verdadeiro, as **40 issues** `WYDDB-001`–`040` e milestones existentes, incluindo `F7 — migrations e
evolução` (milestone 13). O repositório é público e tem `develop` como base de merge.

**Atualização de 2026-11-26 — `OPS-05` fechada:** o repositório tem **121 issues abertas** — as 40
`WYDDB`, as **61 fatias** (#129–#189), **10 lacunas** (#190–#199) e as **10 `ADM`** (#208–#217) —, e as
labels `estado/proposta`, `estado/aceita` e `fase/1`–`fase/6` estão em uso. O board `WYD 1.0.0` nº 10
tem os **121 itens**, 5 views e os campos `Sistema`, `Status`, `Fase`, `Prioridade` e `Marco`
preenchidos; a publicação original saiu no PR [#128](https://github.com/lucascantarelli/wyd/pull/128),
sem merge, e a consolidação documental foi integrada em `develop` pelos PRs #205–#207.

## 1. Título de issue

Formato: `[<tipo>] <sistema>: <resumo curto>`.

- Tipos: `planejamento`, `decisão`, `tarefa`, `defeito`.
- Sistema: um dos 13 (`wyd-protocol`, `wyd-net`, …, `wyd-ops`) ou `repositório`.
- Exemplo: `[tarefa] wyd-net: WYDNET-001 baseline do crate`.

## 2. Labels (namespaces)

- `area/<nome>` — dono do trabalho. **Em uso** no repositório; substitui o `sistema/<nome>` que esta
  página prescrevia antes da decisão de 2026-11-25 (§2.1).
- `tipo/<planejamento|decisao|tarefa|defeito>` — previsto; **não existe** no repositório hoje.
- `fase/<1..7>` ou `fatia/F0..`.
- `estado/<proposta|aceita|em-andamento|em-revisao|bloqueada|aguardando-decisao>` — no máximo uma; a sequência está em §2.2.
- `prioridade/P1|P2|P3`.
- `marco/M0..M5`.

## 2.1 Decisão — namespace de label (2026-11-25)

A leitura do repositório em 2026-11-25 encontrou em uso `area/<sistema>` (por exemplo `area/wyd-db`) e
`estado/bloqueada`; `sistema/<nome>` e `tipo/<nome>` **não existiam**. **Decisão do dono em 2026-11-25:
manter `area/*` e ajustar este mapping**, em vez de renomear labels e reetiquetar as 40 issues. As 61
páginas por fatia de `OPS-05` nascem com `area/<sistema>`; `tipo/*` pode ser criado no mesmo ato, sem
renomear o que já existe.

## 2.2 Transição automática de estado (2026-11-26)

O workflow `.github/workflows/issue-status.yml` executa `wyd-ops/scripts/sync-issue-status.mjs` em
eventos de issue e de PR. Labels são a fonte de verdade; o campo `Status` do Project `WYD 1.0.0`
(nº 10) é espelho e só é atualizado se o secret `PROJECT_TOKEN` existir (PAT com escopo `project`).

| Evento | Label aplicada | Status do Project |
|---|---|---|
| issue aberta | `estado/proposta` | Backlog |
| label `estado/aceita` | `estado/aceita` | Pronto |
| PR aberto/reaberto com `Closes #N` | `estado/em-andamento` | Em andamento |
| PR em `ready_for_review` | `estado/em-revisao` | Em revisão |
| PR fechado sem merge | `estado/aceita` | Pronto |
| PR mergeado ou issue fechada | mantém a última label | Done |
| issue reaberta | retoma a última label não lateral | status correspondente |
| `estado/bloqueada` / `estado/aguardando-decisao` | a label lateral | Bloqueado |
| remoção da última label de estado | `estado/proposta` | Backlog |
| mudança de label feita pelo próprio script | não reprocessa | não mexe |

Regras: no máximo uma label `estado/*`, aplicada em **uma** requisição `PUT /labels` (troca atômica);
o script não reprocessa evento de label que ele mesmo gerou (`github-actions[bot]`), não regride
`em-revisao` para `em-andamento` por evento repetido e decide sobre o estado lido da API, não sobre o
payload do evento; `pull_request_target` não faz checkout do código do PR; sem `PROJECT_TOKEN`, o
board fica manual e o log avisa. A decisão está em `ADR-OPS-0009`; a prova é
`node --test wyd-ops/tests/unitario/`.

## 3. Branch

- `feat/<sistema>-<id>-<slug>` para implementação.
- `docs/<sistema>-<id>-<slug>` para documentação.
- `fix/<sistema>-<id>-<slug>` para defeito.
- Sempre a partir de `develop`; PR para `develop`. `main` é release.

## 4. Pull request

- Um PR por issue. Título repete o da issue.
- Corpo: contexto, entregável, DoD, evidência (saída de comando), arquivos alterados.
- Checklist do template em `.github/pull_request_template.md`.
- Gate do sistema e gate de docs verdes antes de abrir.

## 5. Milestones e Projects

- Milestones por marco (`M0`–`M5`), nomeados `M<n> — <nome>`.
- Project `WYD 1.0.0` nº 10 (121 itens, 5 views, dependências `blocked by`) conforme
  `proposta-gestao-github-e-ambientes.md` §10.3. O board foi montado em 2026-11-26; a criação de
  novas issues, labels ou views continua ato separado e exige aprovação.

## 6. O que esta página não cobre

- Criação efetiva de issues/labels/milestones/boards (requer aprovação do usuário).
- Segredos e deploy (ver `wyd-ops/docs/...`).

## 7. Inventário de issues planejadas (medido em 2026-11-26)

O repositório tem **111 páginas de issue** planejadas e **121 issues** no GitHub: as **40**
`WYDDB-nnn` do `wyd-db`, as **61** por fatia dos 11 sistemas abaixo, as **10** `ADM-nn` do `wyd-admin`
e as **10** de lacuna (#190–#199), cada uma com estado, dependência, DoD observável e bloco de skills
(`skill_principal` + `skills_complementares`) na lista fechada do
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

| Sistema | Chave | Páginas |
|---|---:|---:|
| `wyd-bots` | `WYDBOT-F<n>` | 4 |
| `wyd-client` | `WYDCLI-F<n>` | 3 |
| `wyd-core` | `WYDCORE-F<n>` | 5 |
| `wyd-docs` | `WYDDOC-F<n>` | 4 |
| `wyd-launcher` | `WYDLCH-F<n>` | 4 |
| `wyd-net` | `WYDNET-F<n>` | 5 |
| `wyd-ops` | `WYDOPS-F<n>` | 6 |
| `wyd-portal` | `WYDPOR-F<n>` | 12 |
| `wyd-protocol` | `WYDPRT-F<n>` | 10 |
| `wyd-server` | `WYDSRV-F<n>` | 4 |
| `wyd-sim` | `WYDSIM-F<n>` | 4 |
| `wyd-admin` | `ADM-nn` | 10 |
| `wyd-db` | `WYDDB-nnn` | 40 |
| **Total** | — | **111** |

A chave por fatia (`WYDBOT-F0`, `WYDPRT-0.1.0`) não antecipa número: `WYDDB-nnn` já é sequencial e
criado; os demais recebem o número na criação da issue. Das **111** páginas planejadas, **111** estão
no GitHub: as **40** `WYDDB`, as **61** fatias #129–#189 e as **10** `ADM-nn` #208–#217. Além delas,
**10** issues de lacuna (#190–#199) existem e não fazem parte do inventário de fatias. Cada issue
criada nasceu com `area/<sistema>`, `estado/proposta` e `fase/<n>` (quando a página tem fase),
documento de origem com link para `develop`, `**Gate de verificação:**`, DoD observável e o bloco `## Skills`. O board
nº 10 tem os 121 itens, com `Sistema`, `Status`, `Fase`, `Prioridade` e `Marco` preenchidos.

## 8. Mapeamento determinístico issue → board (2026-11-26)

A importação usou somente labels, título e o sistema dono. Não há campo digitado à mão no board:

| Campo do board | Fonte | Regra aplicada |
|---|---|---|
| `Sistema` | label `area/<sistema>` | o valor depois de `area/`; 0 itens sem valor |
| `Status` | labels `estado/*` | `estado/bloqueada` ou `estado/aguardando-decisao` → `Bloqueado`; demais → `Backlog`; `Done` quando a issue fechar |
| `Fase` | sistema/issue | `OPS-04` (#198) → `5 — Operação`; as demais → `4 — Implementação`, inclusive as fatias de planejamento |
| `Prioridade` | bloqueio | bloqueada/aguardando decisão → `P1`; demais → `P2` |
| `Marco` | sistema | `wyd-db` → `M2`; `wyd-ops` → `M5`; os outros → `M3` |

O campo `Milestone` continua sendo o milestone de fase do GitHub, não o `Marco` do Project. A
prioridade P1/P2 acima é uma classificação operacional inicial, não uma promessa de prazo; o dono pode
ajustá-la sem mudar a página da fatia. O board é espelho: a página/issue continua dona do DoD e da
evidência.
