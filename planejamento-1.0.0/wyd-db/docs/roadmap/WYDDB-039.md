# WYDDB-039 — escrever 0013 a 0029 de conteúdo e views

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 7 — migrations e evolução de schema


## Contexto

Conteúdo, regras, mapas, combate, quests e as duas views fecham o catálogo e a leitura cruzada; os seeds bloqueados não podem ser tratados como dados reais.

## Entregável

Escrever `0013`–`0029` com DDL e os seeds aplicáveis, mantendo `0016`–`0018`, `0020`, `0023` e `0027` bloqueados até a conferência da fonte.

## DoD

- [ ] Os arquivos de DDL aplicam na ordem; as views de `0029` são criadas.
- [ ] Os seis seeds bloqueados permanecem sem dado inventado e a página declara a lacuna.
- [ ] Contagens de conteúdo só entram com fonte e data.
- [ ] O contrato passa sem FK para o futuro.

## Fora de escopo

Não extrair dado do cliente 7662, não medir contagem de seed e não liberar seed bloqueado.

## Risco

A pressão para “completar” os seeds pode inventar catálogo; a issue deve falhar explicitamente enquanto o dado não existir.

## Depende de

WYDDB-038, dados de origem dos seeds.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Arquivos DDL, views e páginas; seeds bloqueados listados.
