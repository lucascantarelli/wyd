# WYDDB-003 — congelar contrato público de provision

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 1 — higiene e baseline

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P1

## Contexto
**Sem arquivo no repositório:** `roles.sql` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

O provisionamento também é chamado pelo binário: `Command`, `Report` e `run<B>` precisam ser estáveis antes de o adapter e o golden de `roles.sql` serem escritos.

## Entregável
**Sem arquivo no repositório:** `provision/app/runner.rs`, `src/provision/mod.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

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

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída de `cargo test -p wyd-db --test contrato_api_provision` e páginas de módulo atualizadas.
