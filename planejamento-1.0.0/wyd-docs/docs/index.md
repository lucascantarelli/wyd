> Pergunta: "o que é o site oficial da documentação e como ele é montado?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-docs
> Base: —
> Revisado em: 2026-11-19

**Estado:** proposto (2026-11-19)

O `wyd-docs` publica a documentação oficial da `1.0.0` como site estático em **MkDocs 2.0** e
**Python 3.15**. Ele não escreve conteúdo de domínio: monta as páginas de
`planejamento-1.0.0/<sistema>/docs/` e publica com verificação própria
(`V`, 2026-11-19: o gate em container Linux passou no `verificar.py`; total e censo em `wyd-docs-1.0.0.md` §7.2).

> [!ABSTRACT] Regra que vale para todo o site
> A página é escrita no sistema dono e publicada como está. O site não edita, não resume e não
> reescreve link: se a publicação precisa de correção, a correção é no arquivo de origem.

## O que o site tem

| Peça | Onde |
|---|---|
| versão fixada da ferramenta | `wyd-docs/requirements.txt` |
| montagem das páginas e da navegação | `wyd-docs/preparar.py` |
| gate de link, âncora, navegação, título, redirect e sintaxe 1.x | `wyd-docs/verificar.py` |
| tema: template, CSS, JS e Mermaid | `wyd-docs/tema/` |
| página inicial e guia do autor | `wyd-docs/docs/` |

## Em que fase o site está

A ordem de produção é do plano diretor (`wyd-docs-1.0.0.md` §9) e o critério de saída da fase 4 é do
mapa do sistema (`roadmap-fase-3-para-4.md`); nenhum dos dois é publicado, e esta página aponta para
eles sem repetir fase nem critério.

## O que o MkDocs 2.0 não tem

Medido em 2026-11-19 no `2.0.dev6`: sem `mkdocs.yml`, sem plugin, sem tema embutido, sem
configuração de Markdown, sem modo estrito e sem suporte a Windows (`PosixPath`). Cada ausência tem
um substituto declarado no plano diretor §2.2 — e nenhum deles é configuração: é código do projeto.

## O que o site não publica

Plano diretor, `lacunas.md`, `src/`, `tests/`, `prompts/` e `skills/` ficam fora; a regra completa,
com origem e destino de cada pasta, é a de `wyd-docs-1.0.0.md` §3.

O teto de páginas e o gate são do plano diretor (`wyd-docs-1.0.0.md`); as decisões pendentes têm
dono e estado em `lacunas.md` §1.
