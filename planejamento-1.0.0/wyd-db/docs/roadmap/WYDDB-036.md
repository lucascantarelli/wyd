# WYDDB-036 — escrever 0002_ops_audit.sql

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 7 — migrations e evolução de schema


## Contexto

Auditoria e tentativa de login são a trilha de operação; precisam nascer cedo e sem FK para tabela futura.

## Entregável

Escrever `0002_ops_audit.sql` com `ops.audit_log` e `ops.login_attempt`, incluindo `actor_kind`/`actor_id`/`actor_ref`, CHECKs e índice, conforme `ddl-confirmado.md` §5.3–§5.4.

## DoD

- [ ] `up` cria as duas tabelas com constraints nomeadas; `down` as remove.
- [ ] Nenhuma FK para tabela criada depois.
- [ ] O teste de contrato passa e o teste de sistema prova um CHECK violado.
- [ ] A página `migrations/0002_ops_audit.md` existe.

## Fora de escopo

Não implementar retenção, partição ou exportação de auditoria.

## Risco

A trilha pode conter PII; o DDL e a página não devem gravar e-mail cru, só hash quando for o caso.

## Depende de

WYDDB-035.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Arquivo, teste de contrato e teste de CHECK no sistema.
