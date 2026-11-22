# WYDDB-040 — fechar 0030 a 0034 e trocar a série

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 7 — migrations e evolução de schema
**Prioridade:** P3

## Contexto

Os adendos finais (portal, MFA, grants, idempotência e mercado) e a troca da série são a última etapa; sem o corte vertical verde, a troca não existe.

## Entregável

Escrever `0030`–`0034`, rodar catálogo/contrato/sistema e trocar a série com `SCHEMA_VERSION = 34`, mantendo a série herdada fora do runtime.

## DoD

- [ ] `0030`–`0034` aplicam e desfazem; o catálogo lista as 109 tabelas e 2 views.
- [ ] Contrato, sistema e ciclo passam; nenhum seed bloqueado é usado.
- [ ] `SCHEMA_VERSION` é 34 e a série herdada não é lida no boot.
- [ ] A troca é registrada em `registro-de-execucao.md` com data e comando.

## Fora de escopo

Não fazer ETL da base legada, não apagar a série antiga sem decisão e não declarar produção validada.

## Risco

A troca pode deixar o runtime apontando para a série errada; o teste de boot e o registro de execução são obrigatórios.

## Depende de

WYDDB-037, WYDDB-038, WYDDB-039 e o corte vertical verde.

## Evidência ao fechar

Série aplicada, catálogo/contagens e registro da troca.
