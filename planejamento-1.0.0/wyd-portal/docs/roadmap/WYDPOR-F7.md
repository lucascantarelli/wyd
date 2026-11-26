# WYDPOR-F7 — Armaria read-only

> Pergunta: "o que a fatia F7 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F7; `docs/specs/requisitos.md` (`RF-POR-019` a `RF-POR-022`, `RNF-POR-008`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F7 — Armaria read-only
**Fase:** 6

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F7 do `wyd-portal` cobre `/personagem/[id]` com cabeçalho, os 16 slots, habilidades, ficha do item, filtro e ordenação e existe para
cumprir `RF-POR-019` a `RF-POR-022`, `RNF-POR-008` sem antecipar a fatia seguinte.

## Entregável

`/personagem/[id]` com cabeçalho, os 16 slots, habilidades, ficha do item, filtro e ordenação

## DoD

- [ ] rota responde sem sessão.
- [ ] personagem apagado devolve `404`.
- [ ] resposta sem `account_id`, e-mail, IP ou ouro.
- [ ] slot vazio e item não catalogado não quebram a página.
- [ ] a ficha exibe habilidades especiais pelo símbolo e, quando houver origem provada, dano e alcance.
- [ ] durabilidade não aparece como número.
- [ ] `stat_source` declara o que não foi medido e nenhum número é inventado.
- [ ] nenhuma escrita do portal em `game`.

## Fora de escopo

edição de perfil, importação manual de dado, simulador de build

## Depende de

F2; views de `wyd-db/armaria-e-mercado.md` §2 no lote; `LAC-POR-08` fechada em 2026-11-22 (ficha publica só o provado; captura em `P-POR-08`)

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

Saída do comando do DoD no PR, com a fatia F7 concluída em `docs/roadmap/roadmap.md`.
