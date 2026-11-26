# Mapeamento GitHub — labels, nomes e convenções

> Pergunta: "como nomear issue, branch, PR e label para que o projeto fique mapeado no GitHub?"
> Público: Desenvolvimento, operação e agentes
> Dono: `wyd-ops`
> Base: `.github/ISSUE_TEMPLATE/*`; `proposta-gestao-github-e-ambientes.md`; `AGENTS.md` §7, §9
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Convenção de nomes; criação de issues/milestones/boards é ato
explícito e não foi feita nesta rodada.

## 0. Estado medido do repositório (2026-11-25)

Leitura somente de `lucascantarelli/wyd` pela integração GitHub: branch padrão `develop`, `has_issues`
verdadeiro, as **40 issues** `WYDDB-001`–`040` e milestones existentes, incluindo `F7 — migrations e
evolução` (milestone 13). O repositório é público e tem `develop` como base de merge.

**Atualização de 2026-11-26 — `OPS-05` executada:** o repositório tem **111 issues abertas** — as 40
`WYDDB`, as **61 fatias** (#129–#189) e **10 lacunas** (#190–#199) —, e as labels `estado/proposta`,
`estado/aceita` e `fase/1`–`fase/6` foram criadas. A publicação saiu no PR
[#128](https://github.com/lucascantarelli/wyd/pull/128), sem merge. O board do Project segue não montado.

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
- `estado/<bloqueada|proposta|aceita>` — `estado/bloqueada` já existe.
- `prioridade/P1|P2|P3`.
- `marco/M0..M5`.

## 2.1 Decisão — namespace de label (2026-11-25)

A leitura do repositório em 2026-11-25 encontrou em uso `area/<sistema>` (por exemplo `area/wyd-db`) e
`estado/bloqueada`; `sistema/<nome>` e `tipo/<nome>` **não existiam**. **Decisão do dono em 2026-11-25:
manter `area/*` e ajustar este mapping**, em vez de renomear labels e reetiquetar as 40 issues. As 61
páginas por fatia de `OPS-05` nascem com `area/<sistema>`; `tipo/*` pode ser criado no mesmo ato, sem
renomear o que já existe.

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
- Project `WYD 1.0.0` (40 itens, 5 views, dependências `blocked by`) conforme
  `proposta-gestao-github-e-ambientes.md` §10.3. **Criação é ato separado (requer aprovação).**

## 6. O que esta página não cobre

- Criação efetiva de issues/labels/milestones/boards (requer aprovação do usuário).
- Segredos e deploy (ver `wyd-ops/docs/...`).

## 7. Inventário de issues planejadas (medido em 2026-11-25)

O repositório tem **101 páginas de issue** planejadas: as **40** `WYDDB-nnn` do `wyd-db`
e as **61** por fatia dos 11 sistemas abaixo, cada uma com estado, dependência, DoD observável e
bloco de skills (`skill_principal` + `skills_complementares`) na lista fechada de
`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1.

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
| `wyd-db` | `WYDDB-nnn` | 40 |
| **Total** | — | **101** |

A chave por fatia (`WYDBOT-F0`, `WYDPRT-0.1.0`) não antecipa número: `WYDDB-nnn` já é sequencial e
criado; os demais recebem o número na criação da issue. **101** planejadas e **101** criadas no
GitHub: **40** `WYDDB` e **61** fatias (#129–#189), mais **10** issues de lacuna (#190–#199) que não
fazem parte do inventário de fatias. Cada issue nova nasceu com `area/<sistema>`, `estado/proposta` e
`fase/<n>` (quando a página tem fase), documento de origem com link, DoD observável e a seção
"Skills a carregar".
