# WYDDB-002 — congelar contrato público de migrate

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

`Command`, `DownTo`, `Report` e `run` são a fachada que o binário e o boot chamam. Se a assinatura nascer sem teste de contrato, uma mudança de variante quebra o chamador sem aviso.

## Entregável

Fixar a API pública de `src/migrate/mod.rs` e `src/migrate/app/mod.rs` no molde de `src/migrate/spec.md` §8–§10, com `#[non_exhaustive]` nos tipos que podem crescer e `run<B, S>` genérico nas portas.

## DoD

- [ ] Um alvo `contrato_api_migrate` compila `Command`, `DownTo`, `Report` e `run` e falha se um campo/variante público mudar sem atualização do teste.
- [ ] `status`, `apply` e `down` são representáveis sem `bool` fora do nome do comando.
- [ ] `Report` é dado, não string; a formatação fica no binário.
- [ ] `src/migrate/mod.md` e `src/migrate/app/mod.md` citam a mesma assinatura.

## Fora de escopo

Não implementar o runner completo nem o adapter; esta issue fecha a fachada e o teste de compilação.

## Risco

`#[non_exhaustive]` mal usado esconde uma variante necessária ao chamador; o teste precisa cobrir os três comandos.

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

Saída de `cargo test -p wyd-db --test contrato_api_migrate` e link para o commit da página de módulo.
