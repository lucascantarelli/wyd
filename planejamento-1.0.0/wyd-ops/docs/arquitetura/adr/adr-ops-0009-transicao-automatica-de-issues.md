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

Um workflow (`issue-status.yml`) com um script único (`wyd-ops/scripts/sync-issue-status.mjs`) aplica esta sequência:

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
| mudança de label feita pelo próprio script | não reprocessa | não mexe |
| evento em issue fechada | não altera | Done |

Regras:

1. **No máximo uma label `estado/*`.** O script remove as demais antes de adicionar a desejada.
2. **Sem regressão automática.** `em-revisao` não volta para `em-andamento` por um evento repetido; reabertura e fechamento sem merge são exceções explícitas.
3. **Labels são a fonte de verdade.** O Project é espelho; se `PROJECT_TOKEN` (PAT com escopo `project`) não existir, o script avisa e continua só com labels.
4. **`pull_request_target` sem checkout do PR.** O job roda o script da branch padrão; não executa código do PR. Isso mantém o token de escrita em PR de fork sem o risco de rodar código não confiável. Como o GitHub bloqueia esse evento por padrão em repositórios públicos a partir de 2/dez/2026, a política de execução do repositório tem uma exceção escopada **somente** para `.github/workflows/issue-status.yml`, permitindo `issues`, `pull_request_target` e `workflow_dispatch`; nenhum outro workflow ganha a exceção.
5. **Issue fechada não perde a label.** O fechamento é o fato; a label preservada permite retomar o estado ao reabrir.
6. **Troca atômica.** O estado é aplicado com uma única requisição `PUT /issues/{n}/labels` contendo o conjunto final; remover e adicionar em duas chamadas abria uma janela sem estado e publicava um `unlabeled` que reprocessava a issue.
7. **Eco do próprio script é ignorado.** A troca usa o `GITHUB_TOKEN`, então o GitHub a atribui a `github-actions[bot]`; evento de label com esse autor não é decisão de ninguém e não é reprocessado. Sem essa guarda, o `unlabeled` da label antiga chegava depois do `labeled` da nova e devolvia a issue entregue para `estado/proposta`.
8. **Estado lido da API.** A decisão usa o estado corrente do `GET /issues/{n}`, nunca o payload do evento, que pode estar desatualizado quando a execução enfileirada começa.
9. **`unlabeled` não regride com outra label presente.** Só a remoção da última label de estado devolve a issue à fila; havendo outra, ela é o estado vigente.
10. **Issue entregue e fechada carrega o último estado da sequência.** Entrega que passa pelo fluxo termina em `estado/em-revisao` antes do merge e a label é preservada no fechamento; entrega fechada sem `Closes #N` recebe o mesmo estado, porque o trabalho está em `develop` e não volta à fila.
11. **Issue fechada é terminal.** Qualquer evento de label posterior — inclusive a correção de uma label atrasada — preserva a label e mantém o `Status` em Done. Só reabrir retoma a sequência.

## Consequências

- Positivas: label e board deixam de divergir; a sequência é testável em um arquivo; PR de fork não precisa de token manual.
- Negativas/custos: o espelho do Project exige um PAT e um segredo; sem ele, o board continua manual. `pull_request_target` exige disciplina para nunca fazer checkout do head do PR e depende da exceção de política revisada quando o workflow mudar.
- Contratos afetados: `github-mapping.md` passa a documentar a sequência; o label `estado/em-andamento` foi criado; a proteção de branch não exige este workflow (ele não é gate de código).

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Só labels, sem Project | o board continuaria manual e divergente |
| Só Project, sem labels | perde o filtro por label e o estado visível na issue |
| Action de terceiro para Project | mais uma dependência; o `gh`/GraphQL cobre o caso |
| `issues` apenas | não captura PR aberto/revisão/merge |

## Como verificar

Abrir uma issue de teste, aceitar, abrir PR com `Closes #N`, marcar ready, mergear e conferir a label e o `Status`. Sem `PROJECT_TOKEN`, o log mostra o aviso e a label muda. `node --test wyd-ops/tests/unitario/` prova as regras sem falar com o GitHub (troca em lista única, `unlabeled` sem outra label e eco do bot ignorado); `node --check wyd-ops/scripts/sync-issue-status.mjs` valida a sintaxe; `gh api repos/lucascantarelli/wyd/actions/policies` mostra a exceção escopada ao workflow.

## Referências

- Decisão do dono: pedido de 2026-11-26
- Documentos donos: `docs/github-mapping.md` §2.2, `docs/devops/index.md`
- Project: `WYD 1.0.0` nº 10, campo `Status`
