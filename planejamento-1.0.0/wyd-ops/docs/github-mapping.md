# Mapeamento GitHub — labels, nomes e convenções

> Pergunta: "como nomear issue, branch, PR e label para que o projeto fique mapeado no GitHub?"
> Público: Desenvolvimento, operação e agentes
> Dono: `wyd-ops`
> Base: `.github/ISSUE_TEMPLATE/*`; `proposta-gestao-github-e-ambientes.md`; `AGENTS.md` §7, §9
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Convenção de nomes; criação de issues/milestones/boards é ato
separado e não foi feito.

## 1. Título de issue

Formato: `[<tipo>] <sistema>: <resumo curto>`.

- Tipos: `planejamento`, `decisão`, `tarefa`, `defeito`.
- Sistema: um dos 13 (`wyd-protocol`, `wyd-net`, …, `wyd-ops`) ou `repositório`.
- Exemplo: `[tarefa] wyd-net: WYDNET-001 baseline do crate`.

## 2. Labels (namespaces)

- `sistema/<nome>` — dono do trabalho.
- `tipo/<planejamento|decisao|tarefa|defeito>`.
- `fase/<1..7>` ou `fatia/F0..`.
- `estado/<bloqueada|proposta|aceita>`.
- `prioridade/P1|P2|P3`.
- `marco/M0..M5`.

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