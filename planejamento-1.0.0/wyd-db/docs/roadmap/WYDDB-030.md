# WYDDB-030 — testar sistema PostgreSQL e ciclo

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 6 — testes de lacuna e harness

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P3

## Contexto

Só o banco real prova `CHECK`, FK, `down` e concorrência; o contrato de texto não substitui o efeito transacional.

## Entregável

Criar `sistema_pg` com aplicação da série, reaplicação sem mudança, drift de um byte, ciclo `down --to 0` e dois processos concorrentes.

## DoD

- [ ] Teste aplica a série e confere o ledger com o número real de arquivos.
- [ ] Reaplicar não muda nada; drift de um byte falha com código 4.
- [ ] `down --to 0` deixa zero tabela/view e ledger vazio; o `apply` seguinte reaplica.
- [ ] Dois processos: um aplica, o outro espera e termina sem aplicar.

## Fora de escopo

Não testar carga, latência ou volume; não usar banco compartilhado de produção.

## Risco

O ciclo completo pode ser lento e flaky; isolamento e timeout são parte do harness, não um `sleep`.

## Depende de

WYDDB-013, WYDDB-028.

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

Saída do alvo com `WYD_DB_URL_TEST` e banco descartável.
