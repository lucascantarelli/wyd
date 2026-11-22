> Pergunta: "onde este assunto mora, como o arquivo se chama e como a pasta vira navegação?"
> Público: Autor e revisor de documentação
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §4, §6.14 e `P8`; `convencoes.md`; `wyd-docs/preparar.py`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

A árvore é organizada por **pergunta do leitor**, não por tipo de arquivo. Cada pasta tem um `index.md`
que é a porta; a navegação é gerada a partir da árvore real, não de uma lista manual. O nome do arquivo
é `kebab-case`, sem acento, e a página nasce no diretório dono do assunto — não no diretório de quem
escreveu.

## 1. O que pertence a cada pasta

| Pasta | Pergunta | Não entra |
|---|---|---|
| `arquitetura/` | por que o banco é assim? | passo a passo de operação |
| `specs/` | o que o banco precisa fazer? | DDL e coluna |
| `dicionario-dados/` | onde está a tabela e o que o campo significa? | regra de jogo sem relação com a tabela |
| `planos/` | como o trabalho é planejado, medido e implantado? | tarefa executável sem critério |
| `roadmap/` | qual é a próxima issue e qual o DoD? | decisão de arquitetura |
| `guia-desenvolvedor/` | como mexo no crate? | operação do banco |
| `guia-administrador/` | como instalo, configuro e recupero? | modelagem de tabela |
| `guia-teste/` | como provo? | cenário detalhado de um alvo |
| `guia-documentacao/` | como escrevo e publico? | conteúdo de domínio |

Uma página que responde a duas perguntas está no lugar errado ou é duas páginas. O dono da fronteira é
`wyd-db-1.0.0.md` §5; esta página só a aplica.

## 2. Nome e índice

- Nome de arquivo: `kebab-case`, sem acento, sem espaço e sem prefixo de numeração fora de
  `migrations/` e `WYDDB-nnn`.
- `index.md` é a porta da pasta; não é uma lista de links sem pergunta.
- `README.md` não é usado para página de documentação.
- Um assunto novo vira seção dentro da página dona; só vira página quando tem pergunta própria e
  precisa de link estável.
- A página não copia `convencoes.md`: cita a convenção e escreve o conteúdo.

## 3. Como a árvore vira navegação

`wyd-docs/preparar.py` percorre `planejamento-1.0.0/<sistema>/docs/**/*.md`, copia cada página para
`.build/docs/sistemas/<sistema>/` e monta a navegação com o primeiro `# ` de cada arquivo. A ordem é:
`index.md` primeiro, depois páginas por nome e subpastas. Não há `nav:` manual no MkDocs 2.0.

Consequências:

| Se você… | A navegação… |
|---|---|
| cria uma pasta sem `index.md` | mostra a pasta, mas não tem porta de leitura |
| renomeia um arquivo | quebra o link antigo; não há redirect automático |
| usa acento no nome | gera URL inconsistente; o padrão proíbe |
| cria link para fora de `docs/` | o verificador acusa destino inexistente no site |
| adiciona sintaxe 1.x | o verificador reprova antes do build |

Não existe `mkdocs.yml`/`nav` aprovado nesta árvore; a configuração é do `wyd-docs`. O que o autor
controla é o nome, o `index.md` e o link relativo.

## 4. Checklist rápido

- [ ] A página está na pasta dona da pergunta?
- [ ] O nome é `kebab-case` sem acento?
- [ ] A pasta tem `index.md` de verdade?
- [ ] O link relativo resolve dentro de `docs/`?
- [ ] A página cita a fonte e não duplica `convencoes.md`?
- [ ] O título `# ` é único e descreve a pergunta?

## 5. O que esta página não cobre

- **Publicação no GitHub Pages:** é do `wyd-docs`; o fluxo de revisão está em
  [`revisao-e-publicacao.md`](revisao-e-publicacao.md).
- **Sintaxe de Markdown/MkDocs:** é de `estilo-e-linguagem.md` e das convenções; esta página não a
  redigita.
- **Diagramas e imagens:** continuam bloqueados por duplicação com `convencoes.md`.
