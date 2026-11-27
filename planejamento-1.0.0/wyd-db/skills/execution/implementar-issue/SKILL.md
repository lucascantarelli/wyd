---
name: implementar-issue
description: Executa uma issue do wyd-db de ponta a ponta — código, teste na pasta prevista, documentação dona, PR, checks e merge — sem sair do escopo. Use quando a issue está com DoD acordado e a página dona já está aceita, ou quando o dono pedir para "pegar a issue X".
---

# Implementar issue

## Quando usar

- A issue tem DoD e a página que a descreve está `aceita`.
- O dono pediu execução de uma issue específica.

Não use para issue bloqueada por dado (seed) ou por decisão: nelas o produto é pergunta com
recomendação, não código.

## Pré-requisitos

- `AGENTS.md` §5 e §7 lidos; `plano-migrations.md` ou a página dona do item.
- `develop` atualizado e árvore limpa: `git status --short --branch`.
- Issue com **uma** label `estado/*` e cartão no Project `WYD 1.0.0`.

## Passos

1. **Leia o dono.** Página da issue, página do arquivo/tabela e a seção de `lacunas.md` que a cita.
   Se a página não existe, pare: planejar vem antes de codar.
2. **Marque o estado.** `estado/em-andamento` na issue e o cartão em `In Progress`.
3. **Crie a branch** a partir de `develop`: `feat/<issue>-<slug>` ou `fix/...`. Nunca commit direto
   em `develop`.
4. **Implemente só o escopo da issue.** Melhoria vista de passagem vira issue nova, não commit.
5. **Teste na pasta prevista**: `crates/wyd-db/tests/` para o crate, `wyd-ops/tests/unitario/` para
   série/CI/script. Teste no root reprova a entrega.
6. **Atualize a documentação dona na mesma entrega** — página da migration, dicionário gerado, censo
   do dashboard. "Atualizo depois" significa página divergente.
7. **Rode o gate do sistema** antes de abrir PR: `make gate-fast`.
8. **Abra o PR** citando a issue e o que foi verificado. Sem PR, sem merge.
9. **Acompanhe os checks** com consulta pontual; `gh pr checks --watch` estoura o limite do terminal.

   ```bash
   gh pr checks <n>; sleep 45; gh pr checks <n>
   ```

10. **Squash merge** só com checks verdes. Depois, `make gate` completo na `develop` atualizada.
11. **Feche o ciclo**: comentário com evidência (commit, checks, comando e saída), issue para
    `estado/em-revisao` se houver teste manual do dono, senão `Done`; cartão no board na coluna certa.

## Como verificar

| Alegação | Prova |
|---|---|
| código compila e passa | `cargo test --manifest-path crates/<crate>/Cargo.toml` / `make gate-fast` |
| série íntegra | `WYD_DB_TEST_CONTAINER=$(docker compose ps -q db) cargo test --manifest-path crates/wyd-db/Cargo.toml --test sistema_pg -- --test-threads=1` sem `NotContiguous` |
| merge feito | `git log --oneline -1` em `develop` |
| árvore limpa | `git status --short --branch` só com `## develop...origin/develop` |
| issue coerente | `gh issue view <n> --json labels,state` |

## Quando parar e perguntar

- A execução exige mudar schema, fronteira, nomenclatura ou escopo → proposta com alternativas,
  `estado/aguardando-decisao`.
- O resultado depende de teste no cliente/servidor → `estado/em-revisao` e instrução passo a passo
  para o dono; não declare "validado" o que você não rodou.
- O dado necessário não existe medido → issue volta a `estado/bloqueada` com o motivo, não com número
  inventado.

## Armadilhas

- **Push direto em `develop`** e commit sem PR.
- **Teste fora da pasta prevista** e teste no root — erro recorrente e cobrado.
- **Importar do `deprecated/`** para "destravar": proibido; o legado é nível 4.
- **Marcar como concluído** o que só foi compilado: diga o que foi verificado e o que não foi.
- **`PROJECT_TOKEN` ausente** no workflow: o board não se atualiza sozinho, ele é mantido por API
  (`gh project item-edit`); conferir o cartão faz parte de fechar a issue.
