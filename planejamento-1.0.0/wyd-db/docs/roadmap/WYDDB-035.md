# WYDDB-035 — escrever 0001_bootstrap.sql

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 7 — migrations e evolução de schema
**Prioridade:** P3

## Contexto

`0001` cria os quatro schemas, a função de `updated_at` e `content_version`; sem ele, nenhum outro arquivo da série aplica.

## Entregável

Escrever `0001_bootstrap.sql` com `up`/`down`, schemas `content`, `game`, `portal`, `ops`, função e `content_version`, conforme `ddl-confirmado.md` §3.

## DoD

- [ ] `up` cria os quatro schemas, a função e a versão; `down` remove tudo que criou.
- [ ] Nenhum `ALTER`, seed de conteúdo ou FK para arquivo futuro.
- [ ] O contrato estrutural passa e o ciclo `up → down → up` é idempotente.
- [ ] A página `migrations/0001-bootstrap.md` existe e cita o arquivo.

## Fora de escopo

Não criar dados de conteúdo, não trocar `SCHEMA_VERSION` e não aplicar em produção.

## Risco

O `down` que remove schema pode apagar objeto de outro arquivo se a ordem estiver errada; o teste de ciclo é obrigatório.

## Depende de

WYDDB-014, WYDDB-028.

## Evidência ao fechar

Arquivo versionado, teste de contrato e ciclo local.
