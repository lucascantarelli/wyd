# ADR-OPS-0009 — Transição automática de estado de issue

> Pergunta: "como uma issue anda de proposta a Done sem depender de alguém lembrar de trocar label e Status?"
> Público: Arquitetura, desenvolvimento, operação e agentes
> Dono: `wyd-ops` (aprovado pelo dono em 2026-11-26)
> Base: `docs/github-mapping.md`; `proposta-gestao-github-e-ambientes.md` §3.3 e §3.5; `.github/workflows/issue-status.yml`
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

As issues usavam labels `estado/*` e o Project `WYD 1.0.0` tem um campo `Status` com `Backlog`, `Pronto`, `Em andamento`, `Bloqueado`, `Em revisão`, `Em teste` e `Done`. Não havia automação: abrir uma issue, aceitar, abrir PR, entrar em revisão, mergear e fechar eram atos manuais e divergiam entre label e board.

## Decisão

Um workflow (`issue-status.yml`) com um script único (`.github/scripts/sync-issue-status.mjs`) aplica esta sequência:

| Evento | Label | Status do Project |
|---|---|---|
| issue aberta | `estado/proposta` | Backlog |
| label `estado/aceita` | `estado/aceita` | Pronto |
| PR aberto/reaberto com `Closes #N` | `estado/em-andamento` | Em andamento |
| PR em `ready_for_review` | `estado/em-revisao` | Em revisão |
| PR fechado sem merge | `estado/aceita` | Pronto |
| PR mergeado / issue fechada | mantém a última label | Done |
| issue reaberta | retoma a última label não lateral | status correspondente |
| `estado/bloqueada` ou `estado/aguardando-decisao` | a label lateral | Bloqueado |
| remoção da última label de estado | `estado/proposta` | Backlog |

Regras:

1. **No máximo uma label `estado/*`.** O script remove as demais antes de adicionar a desejada.
2. **Sem regressão automática.** `em-revisao` não volta para `em-andamento` por um evento repetido; reabertura e fechamento sem merge são exceções explícitas.
3. **Labels são a fonte de verdade.** O Project é espelho; se `PROJECT_TOKEN` (PAT com escopo `project`) não existir, o script avisa e continua só com labels.
4. **`pull_request_target` sem checkout do PR.** O job roda o script da branch padrão; não executa código do PR. Isso mantém o token de escrita em PR de fork sem o risco de rodar código não confiável.
5. **Issue fechada não perde a label.** O fechamento é o fato; a label preservada permite retomar o estado ao reabrir.

## Consequências

- Positivas: label e board deixam de divergir; a sequência é testável em um arquivo; PR de fork não precisa de token manual.
- Negativas/custos: o espelho do Project exige um PAT e um segredo; sem ele, o board continua manual. `pull_request_target` exige disciplina para nunca fazer checkout do head do PR.
- Contratos afetados: `github-mapping.md` passa a documentar a sequência; o label `estado/em-andamento` foi criado; a proteção de branch não exige este workflow (ele não é gate de código).

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Só labels, sem Project | o board continuaria manual e divergente |
| Só Project, sem labels | perde o filtro por label e o estado visível na issue |
| Action de terceiro para Project | mais uma dependência; o `gh`/GraphQL cobre o caso |
| `issues` apenas | não captura PR aberto/revisão/merge |

## Como verificar

Abrir uma issue de teste, aceitar, abrir PR com `Closes #N`, marcar ready, mergear e conferir a label e o `Status`. Sem `PROJECT_TOKEN`, o log mostra o aviso e a label muda. `node --check .github/scripts/sync-issue-status.mjs` valida a sintaxe; o workflow é `issue-status.yml`.

## Referências

- Decisão do dono: pedido de 2026-11-26
- Documentos donos: `docs/github-mapping.md` §2.2, `docs/devops/index.md`
- Project: `WYD 1.0.0` nº 10, campo `Status`
