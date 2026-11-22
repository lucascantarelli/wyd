# WYDDB-003 — congelar contrato público de provision

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 2 — contratos e erros tipados
**Prioridade:** P1

## Contexto

O provisionamento também é chamado pelo binário: `Command`, `Report` e `run<B>` precisam ser estáveis antes de o adapter e o golden de `roles.sql` serem escritos.

## Entregável

Fixar a API de `src/provision/mod.rs` e `provision/app/runner.rs` com `Command::{Render, Plan, Apply, Check}`, `Report` e `run<B: Catalog + Script>`, conforme `src/provision/spec.md` §5.

## DoD

- [ ] Um alvo `contrato_api_provision` compila os quatro comandos e o `Report`.
- [ ] `render` devolve texto e não escreve arquivo; a comparação é do binário.
- [ ] `plan` e `check` não chamam `Script`.
- [ ] `provision/mod.md` e `provision/app/mod.md` citam a mesma assinatura.

## Fora de escopo

Não criar o adapter PostgreSQL, o golden ou o comando de CLI; isso é da fase 7 e do binário.

## Risco

Separar `Plan` de `Report` sem deixar o código de saída dentro da biblioteca.

## Depende de

WYDDB-001.

## Evidência ao fechar

Saída de `cargo test -p wyd-db --test contrato_api_provision` e páginas de módulo atualizadas.
