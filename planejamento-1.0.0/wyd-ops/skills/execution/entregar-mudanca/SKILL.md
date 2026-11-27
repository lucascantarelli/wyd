---
name: entregar-mudanca
description: Leva uma mudança do WYD de issue a develop — branch, commits pequenos, PR, checks obrigatórios, squash merge e gate final. Use ao pegar qualquer issue dos sistemas, ao retomar PR parado, ou quando o merge foi feito e falta fechar issue e board.
---

# Entregar mudança

## Quando usar

- Issue com DoD acordado e página dona aceita, em qualquer sistema.
- PR existente que precisa ser concluído.

## Pré-requisitos

- `develop` atualizado e árvore limpa (`git status --short --branch`).
- Issue com **uma** label `estado/*` e cartão no Project `WYD 1.0.0`.

## Passos

1. Branch a partir de `develop`: `feat/<issue>-<slug>`. **Nunca** commit direto em `develop`.
2. Commits pequenos e temáticos; um assunto por commit. Escopo da issue, nada além.
3. Antes do PR: `make gate-fast`.
4. PR citando a issue, o escopo e **o que foi verificado** — comando e saída, não adjetivo.
5. Acompanhe checks com consulta pontual; `gh pr checks --watch` estoura o limite do terminal:

   ```bash
   gh pr checks <n>; sleep 45; gh pr checks <n>
   ```

6. Squash merge só com `mergeStateStatus = CLEAN`. Nunca force-push em `develop`.
7. `make gate` completo na `develop` já atualizada.
8. Comentário final na issue com evidência (commit, checks, gate). Fechar issue e mover o cartão:
   `Done` quando você provou; `em-revisao` quando depende de teste do dono.

## Como verificar

| Alegação | Prova |
|---|---|
| checks verdes | `gh pr checks <n>` sem `fail`/`pending` |
| merge feito | `git log --oneline -1` em `develop` |
| gate completo | `make gate` com `gate: PASS (full, …)` |
| árvore limpa | `git status --short --branch` só com a linha da branch |

## Quando parar e perguntar

- A mudança exige decisão de arquitetura, escopo, nomenclatura ou estrutura de pastas.
- A prova depende de teste no cliente: entregue o roteiro passo a passo e deixe em `em-revisao`.
- O blocker é dado não medido: `estado/bloqueada` com motivo, nunca com número inventado.

## Armadilhas

- **Push direto em `develop`** ou merge sem PR.
- **Teste no root** e teste fora da pasta prevista — erro recorrente e cobrado.
- **Declarar validado o que só compilou.**
- **Deixar árvore suja** ao fim; a próxima execução herda o lixo.
