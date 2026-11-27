# WYDCLI-F3 — Decodificação dos binários do cliente

> Pergunta: "o que a fatia F3 entrega, do que ela depende e como sei que terminou?"
> Público: pesquisa (RE), arquitetura, curadoria do `wyd-db` e QA
> Dono: equipe do `wyd-client`
> Base: `docs/roadmap/roadmap.md` fatia F3; `docs/specs/formato-itemlist-e-itemname.md`; `docs/specs/formato-inititem.md`; `LCLI-06`, `LCLI-07`
> Revisado em: 2026-11-27

**Estado:** proposto (2026-11-27)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F3 — Decodificação
**Fase:** 2
**Prioridade:** P1

## Contexto

O seed `0017` do `wyd-db` está bloqueado por dado, não por DDL: o catálogo de itens só existe nos
`.bin` do cliente. A fatia F3 fecha essa frente arquivo por arquivo, com layout provado pela estrutura
do arquivo e cada campo com domínio medido, sem reaproveitar decodificador do `deprecated/`. O procedimento de decodificação é a skill do sistema
em `../skills/execution/decodificar-binario/SKILL.md`, com `medir-cliente` para a identidade do arquivo.

Duas páginas donas já existem: `ItemList.bin` e `Itemname.bin` estão fechados campo a campo
(`formato-itemlist-e-itemname.md`), e `InitItem.bin` tem estrutura fechada com a semântica em aberto
(`formato-inititem.md`).

## Entregável

Página dona por arquivo decodificado, cada uma com conta do arquivo, codificação provada, tabela
offset × tipo × campo × domínio e lista do que segue em `H`; os enums que o cliente não carrega ficam
em lacuna própria, não em suposição.

## DoD

- [ ] `InitItem.bin` com estrutura, codificação e correspondência fechadas; só o rótulo das colunas e do enum `0…3` em `LCLI-07`, com o que o fecha.
- [ ] `SkillData.bin` (192 B + 4 B) com página dona e domínio de cada campo medido.
- [ ] `Mixlist.bin` com layout e pares medidos, ou bloqueio declarado com o que falta.
- [ ] `object.bin` (nomes de mapa) com codificação e charset provados.
- [ ] Toda página nova passa em `make gate-fast` e cita `V`/`H` com dono; nenhum valor inventado.
- [ ] `0017` desbloqueado por dado: nome e colunas numéricas com fonte citada, ou bloqueio residual
      reduzido a `LCLI-06`/`LCLI-07`.

## Fora de escopo

distribuir ou versionar conteúdo do cliente; escrever o seed dentro desta fatia; mexer no schema;
captura de protocolo (é `wyd-protocol`).

## Depende de

`LCLI-06` e `LCLI-07` para a semântica dos enums; cliente 7662 local (já disponível); nada de captura.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `auditar-planejamento` |
| `skills_complementares` | `writing-plans`, `documentation-writer` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída dos comandos do DoD no PR, com as páginas donas linkadas no índice do sistema e `make gate`
verde na `develop`.
