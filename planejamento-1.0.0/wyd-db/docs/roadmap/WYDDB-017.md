# WYDDB-017 — implementar save em lote do personagem

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 4 — filas, save e shutdown


## Contexto

O save do personagem chega ao banco pelo caminho de transferência; gravar campo a campo aumenta o custo e deixa estado parcial quando uma escrita falha.

## Entregável

Definir e implementar o lote de save em `transfer/save.rs`, com uma transação por lote, versão otimista e erro que nomeia o personagem sem expor PII.

## DoD

- [ ] Teste de sistema prova que um lote grava todos os campos ou nenhum.
- [ ] Duas versões concorrentes não sobrescrevem silenciosamente; a segunda recebe conflito.
- [ ] O lote não inclui campo sem decisão de modelo.
- [ ] O shutdown chama o flush e não deixa lote em memória.

## Fora de escopo

Não definir tamanho de lote, timeout ou política de retry; não criar cache de personagem.

## Risco

Um lote grande demais pode segurar transação; sem medição, a issue não fixa um número.

## Depende de

WYDDB-016.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar
**Sem arquivo no repositório:** `src/transfer-save.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Teste de sistema de save e página `src/transfer-save.md` (a escrever).
