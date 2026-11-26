# WYDDB-037 — escrever 0003 a 0007 do corte vertical

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 7 — migrations e evolução de schema


## Contexto

O corte vertical de login, conta, personagem e sessão é o que prova a série antes de o restante entrar; ele depende do cliente 7662 e do DDL já confirmado.

## Entregável

Escrever `0003_portal_person.sql`, `0004_portal_access.sql`, `0005_game_account.sql`, `0006_game_character.sql` e `0007_game_session.sql`, com `up`/`down` por arquivo.

## DoD

- [ ] Os cinco arquivos aplicam na ordem e o `down` de cada um remove o que criou.
- [ ] O corte vertical de `plano-migrations.md` §3.2 passa com o cliente 7662.
- [ ] Constraints e FKs são nomeadas; nenhuma FK aponta para arquivo posterior.
- [ ] Cada página de migration existe e cita a verificação.

## Fora de escopo

Não escrever seeds, não trocar a série e não tratar o corte como concluído sem a evidência do cliente.

## Risco

O cliente 7662 pode revelar formato divergente; a issue bloqueia em vez de ajustar o DDL por suposição.

## Depende de

WYDDB-035, WYDDB-036, captura do cliente 7662.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Cinco arquivos, ciclo local e evidência datada do corte.
