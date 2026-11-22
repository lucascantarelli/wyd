# WYDDB-032 — testar stack de capacidades

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 6 — testes de lacuna e harness
**Prioridade:** P3

## Contexto

A matriz `CAP-nn` liga requisito a alvo; sem um alvo que a execute, a cobertura é declaração sem prova.

## Entregável

Criar `sistema_stack_caps` para executar os cenários da matriz `CAP-nn`, um por capacidade, e falhar quando uma capacidade não tiver cenário.

## DoD

- [ ] Cada linha `CAP-nn` tem cenário e alvo declarados.
- [ ] Uma capacidade sem cenário reprova o alvo, não passa vazio.
- [ ] O teste usa banco real e dado isolado.
- [ ] A saída lista capacidade, cenário e resultado.

## Fora de escopo

Não inventar capacidades, não definir cobertura percentual e não substituir os alvos de contrato/sistema.

## Risco

A matriz ainda não existe; a issue deve bloquear até ela ser escrita, em vez de criar cenários soltos.

## Depende de

WYDDB-030, WYDDB-031, e a matriz `CAP-nn`.

## Evidência ao fechar

Matriz versionada e saída do alvo.
