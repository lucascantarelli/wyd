# WYDDB-009 — mapear ProvisionError para códigos de saída

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 2 — contratos e erros tipados


## Contexto

O provisionamento tem quatro códigos e um dono diferente por situação; sem teste, `render --check` pode sair 0 com divergência.

## Entregável

Implementar `ProvisionError`, a distinção entre erro de configuração/conexão e drift, e a tabela de códigos 0/2/3/4 de `src/provision/spec.md` §6.

## DoD
**Sem arquivo no repositório:** `db/roles.sql`, `domain/spec.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

- [ ] Teste confere código 2 para configuração/conexão e 4 para drift de `pg_roles`.
- [ ] Divergência de `db/roles.sql` é 3, decidida pelo subcomando.
- [ ] `domain/spec.rs` continua sem `Result`.
- [ ] `provision/error.md` lista cada variante e o código.

## Fora de escopo

Não criar código de saída dentro do domínio e não aplicar script no cluster.

## Risco

Confundir drift de arquivo (3) com drift de cluster (4) torna o diagnóstico ambíguo.

## Depende de

WYDDB-007.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Teste de tabela de códigos e página `src/provision/error.md`.
