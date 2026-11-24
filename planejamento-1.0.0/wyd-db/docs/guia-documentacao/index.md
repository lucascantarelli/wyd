> Pergunta: "qual regra de escrita eu procuro?"
> Público: Quem escreve ou revisa página do `wyd-db`
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.14; `docs/convencoes.md`; `proposta-paginas.md` §6; `pendencias-de-fechamento.md` §6.6 (`Q-BLK-01`, aprovada em 2026-11-24)
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-22)

Esta pasta é o **manual de escrita** da documentação: onde o assunto mora, como a frase é escrita, como se
ilustra, revisa e publica. Ela não substitui o contrato de **leitura** — marcas `V`/`H`/`P`, estados e
cabeçalho são de [`convencoes.md`](../convencoes.md) — nem o molde de cada tipo de página, que é de
`wyd-db-1.0.0.md` §6.

**Decisão de 2026-11-24 (`Q-BLK-01`):** três páginas que estavam previstas aqui — `markdown-e-mkdocs.md`,
`diagramas.md` e `links-referencias-e-ids.md` — **saíram do teto**. O conteúdo delas já era de
`convencoes.md`, e escrever as duas versões criaria dois donos para o mesmo fato. O teto autoral caiu de
**147 para 144**.

## 1. Mapa dos quatro guias

| Tarefa | Guia | Pergunta | Estado |
|---|---|---|---|
| criar página e pasta | [`estrutura-e-navegacao.md`](estrutura-e-navegacao.md) | onde este assunto mora? | escrito |
| escrever | [`estilo-e-linguagem.md`](estilo-e-linguagem.md) | como escrevo? | escrito |
| ilustrar e provar | [`imagens-e-evidencias.md`](imagens-e-evidencias.md) | como provo com imagem? | escrito (2026-11-24) |
| revisar e publicar | [`revisao-e-publicacao.md`](revisao-e-publicacao.md) | como isto é aprovado e publicado? | escrito |

Recurso de Markdown, diagrama e link **não têm página própria**: a regra é de
[`convencoes.md`](../convencoes.md) §Links e citações, §Diagramas e §Recursos do site. Aqui só se aponta
para lá; repetir a tabela seria o defeito que a decisão de 2026-11-24 evitou.

## 2. Ordem de leitura

1. **Antes de escrever:** [`estrutura-e-navegacao.md`](estrutura-e-navegacao.md) (onde mora) e
   [`estilo-e-linguagem.md`](estilo-e-linguagem.md) (como se escreve).
2. **Durante:** as seções de [`convencoes.md`](../convencoes.md) sobre links, diagramas e recursos do site;
   imagem, só se o objeto for visual, por [`imagens-e-evidencias.md`](imagens-e-evidencias.md).
3. **Antes de abrir PR:** [`revisao-e-publicacao.md`](revisao-e-publicacao.md) e o
   [`checklist de PR`](../guia-desenvolvedor/checklist-de-pr.md).

## 3. O que esta pasta não cobre

- **O contrato de leitura:** marcas, estados e cabeçalho são de [`convencoes.md`](../convencoes.md).
- **O molde de cada tipo de página:** é de `wyd-db-1.0.0.md` §6; o de migration, em especial, é de
  `migrations-checklist.md` §5.
- **O gate de link, build e contagem:** é de
  [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) e do `wyd-docs`.
- **A fila de páginas e o teto:** são de `proposta-paginas.md` §6 e `pendencias-de-fechamento.md` §1, na
  raiz de `planejamento-1.0.0/wyd-db/` — caminho em `code`, porque o site publica só `docs/`.
