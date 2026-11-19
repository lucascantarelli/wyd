# Contribuir com a documentação

Cada página pertence a um sistema. Você escreve em `planejamento-1.0.0/<sistema>/docs/`; o site
apenas publica. Corrigir texto no HTML gerado ou no `.build` é trabalho perdido: o próximo build
sobrescreve.

## Antes de escrever

1. Leia `AGENTS.md` — hierarquia de fontes, invariantes e o que exige aprovação.
2. Leia o dashboard do sistema (`<sistema>-1.0.0.md`) e as lacunas dele.
3. Confirme que a página está prevista na árvore do plano diretor. Página fora do teto é proposta,
   não fato.

## Cabeçalho obrigatório

Toda página de `docs/` começa com cinco linhas e uma linha de estado:

```text
> Pergunta: "..."
> Público: <papel>
> Dono: equipe do <sistema>
> Base: <fonte consultada> ou —
> Revisado em: AAAA-MM-DD

**Estado:** proposto (AAAA-MM-DD)
```

## Sintaxe que o site entende

O Markdown do MkDocs 2.0 é GFM com tabelas, notas de rodapé, riscado e âncoras de título. Use:

| Recurso | Como escrever |
|---|---|
| Aviso | `> [!NOTE]`, `> [!TIP]`, `> [!WARNING]`, `> [!CAUTION]` na primeira linha da citação |
| Tabela | pipes, com cabeçalho |
| Código | cerca com linguagem; o botão de copiar aparece no bloco |
| Diagrama | cerca `mermaid` com um tipo por bloco |
| Ênfase | `**forte**` e `` `código` `` |

> [!WARNING] Sintaxe herdada do MkDocs 1.x
> `!!! tip`, `??? note`, `=== "Aba"`, `{ .md-button }` e `:material-x:` **não** existem no MkDocs
> 2.0. O build traduz esses casos para citação, título e lista (`compat.py`), mas página nova não
> deve usá-los: a ponte é para o material antigo, não um recurso do site.

## Publicar

O gate é local e igual ao do CI:

```bash
cd wyd-docs
.venv/Scripts/python.exe preparar.py
.venv/Scripts/mkdocs.exe build --dir .build/docs --output site
.venv/Scripts/python.exe verificar.py site
```

O `verificar.py` reprova link interno quebrado, âncora inexistente, página sem navegação e página sem
título. O MkDocs 2.0 não tem modo estrito: sem esse passo, link quebrado vai para produção.

## Revisão

Página só é `aceito` depois de revisão de alguém que não escreveu. O que reprova está na convenção do
sistema dono — para o `wyd-db`, em `planejamento-1.0.0/wyd-db/docs/convencoes.md`; para o
`wyd-protocol`, em `planejamento-1.0.0/wyd-protocol/docs/convencoes.md`.
