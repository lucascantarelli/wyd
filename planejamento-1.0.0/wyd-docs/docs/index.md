> Pergunta: "o que é o site oficial da documentação e como ele é montado?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-docs
> Base: —
> Revisado em: 2026-11-19

**Estado:** proposto (2026-11-19)

O `wyd-docs` publica a documentação oficial da `1.0.0` como site estático em **MkDocs 2.0** e
**Python 3.15**. Ele não escreve conteúdo de domínio: monta as páginas de
`planejamento-1.0.0/<sistema>/docs/`, aplica a ponte de sintaxe e publica com verificação própria.

!!! abstract "Regra que vale para todo o site"
    A página é escrita no sistema dono e publicada como está. O site não edita, não resume e não
    reescreve link: se a publicação precisa de correção, a correção é no arquivo de origem.

## O que o site tem

| Peça | Onde |
|---|---|
| versão fixada da ferramenta | `wyd-docs/requirements.txt` |
| montagem das páginas e da navegação | `wyd-docs/preparar.py` |
| ponte de sintaxe do material antigo | `wyd-docs/compat.py` |
| gate de link, âncora, navegação e título | `wyd-docs/verificar.py` |
| tema: template, CSS, JS e Mermaid | `wyd-docs/tema/` |
| página inicial e guia do autor | `wyd-docs/docs/` |

## O que o MkDocs 2.0 não tem

Medido em 2026-11-19 no `2.0.dev6`: sem `mkdocs.yml`, sem plugin, sem tema embutido, sem
configuração de Markdown, sem modo estrito e sem suporte a Windows (`PosixPath`). Cada ausência tem
um substituto declarado no plano diretor §2.2 — e nenhum deles é configuração: é código do projeto.

## O que o site não publica

Plano diretor, `lacunas.md`, `src/`, `tests/`, `prompts/` e `skills/` de cada sistema continuam
internos. `deprecated/` e os arquivos do cliente também ficam fora: são referência e binário
proprietário.

O plano diretor do sistema (planejamento) define o teto de páginas, o gate e as quatorze decisões
pendentes.
