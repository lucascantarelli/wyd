> Pergunta: "qual regra de escrita eu procuro?"
> Público: Quem escreve ou revisa página do `wyd-db`
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.14; `docs/convencoes.md`; `proposta-paginas.md` §5–§6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Esta pasta é o **manual de escrita** da documentação: onde o assunto mora, como a frase é escrita,
que recurso de Markdown a 2.0 renderiza, como se diagrama, linka, ilustra, revisa e publica. Ela não
substitui o contrato de **leitura** — marcas `V`/`H`/`P`, estados e cabeçalho são de
[`convencoes.md`](../convencoes.md) — nem o molde de cada tipo de página, que é de
`wyd-db-1.0.0.md` §6.

## 1. Mapa dos sete guias

| Tarefa | Guia | Pergunta | Estado |
|---|---|---|---|
| criar página e pasta | `estrutura-e-navegacao.md` | onde este assunto mora? | não escrito |
| escrever | [`estilo-e-linguagem.md`](estilo-e-linguagem.md) | como escrevo? | escrito |
| usar Markdown e MkDocs | `markdown-e-mkdocs.md` | que recurso posso usar? | não escrito |
| diagramar | `diagramas.md` | que diagrama uso? | não escrito |
| linkar e citar | `links-referencias-e-ids.md` | como aponto para outra página ou para o código? | não escrito |
| ilustrar e provar | `imagens-e-evidencias.md` | como provo com imagem? | não escrito |
| revisar e publicar | `revisao-e-publicacao.md` | como isto é aprovado e publicado? | não escrito |

Caminho em `code` não é link: a página ainda não existe. O estado de recorte ou de não escrita é o de
`proposta-paginas.md` §5–§6; esta página não o reabre.

## 2. Ordem de leitura

1. **Antes de escrever:** `estrutura-e-navegacao.md` (onde mora) e
   [`estilo-e-linguagem.md`](estilo-e-linguagem.md) (como se escreve).
2. **Durante:** `markdown-e-mkdocs.md`, `diagramas.md`, `links-referencias-e-ids.md` e
   `imagens-e-evidencias.md`.
3. **Antes de abrir PR:** `revisao-e-publicacao.md` e o
   [`checklist de PR`](../guia-desenvolvedor/checklist-de-pr.md).

## 3. O que esta página não cobre

- **O contrato de leitura:** marcas, estados e cabeçalho são de
  [`convencoes.md`](../convencoes.md).
- **O molde de cada tipo de página:** é de `wyd-db-1.0.0.md` §6; o de migration, em especial, é de
  `migrations-checklist.md` §5.
- **O gate de link, build e contagem:** é de
  [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) e do `wyd-docs`.
- **A fila de páginas:** é de `proposta-paginas.md` §8.
