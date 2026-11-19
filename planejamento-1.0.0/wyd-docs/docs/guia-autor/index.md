> Pergunta: "como escrevo e entrego uma página no site oficial, do rascunho ao aceite?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-docs
> Base: `wyd-docs-1.0.0.md` §3, §4, §7; `wyd-ops/proposta-gestao-github-e-ambientes.md` §3.6–§3.7
> Revisado em: 2026-11-19

**Estado:** aceito (2026-11-19)

# wyd-docs — guia do autor

**Norma aceita pelo dono de `documentação` em 2026-11-19 (`V`; fase 3 em `wyd-docs-1.0.0.md`
§9).** Esta página descreve o **processo** de autoria. A **forma** da página é de
`planejamento-1.0.0/wyd-docs/docs/convencoes.md` e o **veredito** é de
`planejamento-1.0.0/wyd-docs/docs/revisao.md`; esta página não repete nem um nem outro.

> [!TIP] O caminho é sempre o mesmo
> Você escreve no sistema dono, roda o gate, abre PR com a saída do gate e pede revisão a quem não
> escreveu. O site não tem editor: publica o que o sistema entregou (`V`, 2026-11-19; total em `wyd-docs-1.0.0.md` §7.2).

## 1. Antes de escrever: a página está prevista?

| Passo | O que conferir | Onde |
|---:|---|---|
| 1 | a página existe na árvore do plano do sistema (o teto de páginas) | `<sistema>-1.0.0.md`, árvore de páginas |
| 2 | a dúvida que ela responde ainda não tem dono em outra página | `<sistema>-1.0.0.md` e `docs/` do sistema |
| 3 | a lacuna que a bloqueia está fechada, se houver | `<sistema>/lacunas.md` |
| 4 | o assunto pertence a este sistema, não a outro | `AGENTS.md` §6 |

Página **fora do teto** não se escreve direto: entra como proposta no plano do sistema, com a
pergunta que responde e o lugar na árvore. Publicar sistema novo no site depende de `LPDOC-08`, com
estado em `planejamento-1.0.0/wyd-docs/lacunas.md`; até a decisão, o comportamento medido é que `preparar.py`
publica toda pasta `docs/` que existir (`V`, 2026-11-19; total em `wyd-docs-1.0.0.md` §7.2).

## 2. Onde a página mora

```text
planejamento-1.0.0/<sistema>/docs/<caminho>.md
```

- `index.md` é a porta de entrada da pasta e responde "por onde começo"; não é índice de links.
- O caminho relativo é preservado na publicação, e mover arquivo muda a URL publicada. A regra de
  publicação e de URL é de `wyd-docs-1.0.0.md` §3 e §4.2; o efeito de renomear está em
  `convencoes.md` §4.

## 3. Como escrever

A forma — cabeçalho, estado, evidência, sintaxe e links — é de `convencoes.md` §2–§5, e não se repete
aqui. O que é do processo:

1. Uma pergunta por página. Se o texto responde duas, são duas páginas.
2. Se o assunto é de outro dono, aponte a seção em vez de reescrever.
3. Antes de entregar, releia contra a lista de reprovação de `convencoes.md` §7.

## 4. Rodar o gate antes do PR

O comando é o de `wyd-docs-1.0.0.md` §7.1: no Linux/macOS, venv; no Windows, dentro do container,
porque a `2.0.dev6` falha por `PosixPath` (`LPDOC-12`). O que o gate reprova é a tabela de §7.2 —
esta página não mantém cópia de nenhum dos dois.

Cole no PR a saída do `verificar.py` e o número de páginas montadas, citado de `wyd-docs-1.0.0.md`
§7.2. "Funcionou aqui" não é prova.

## 5. Abrir o PR

Branch, título, corpo, merge e revisão são de `wyd-ops` §3.6 (template do PR) e §3.7 (branch,
histórico e revisão); esta página não repete a regra.

O que ela acrescenta é o efeito de separar decisão e atualização: fechar lacuna sem atualizar o plano
diretor deixa o registro dizendo o contrário do estado.

## 6. Pedir revisão

Pedir revisão a quem não escreveu é a última etapa do processo. Quem revisa, quantas leituras, por que
o ruleset não exige aprovação, o que o revisor confere e como o veredito é registrado: a norma é de
`revisao.md` §1 e seguintes, e esta página não a repete.

## 7. O que esta página não decide

- Não decide a forma da página: é de `convencoes.md`.
- Não decide o veredito da revisão: é de `revisao.md`.
- Não decide quem aprova sistema novo (`LPDOC-08`) nem página `proposto` publicada (`LPDOC-03`).
- Não decide redirect, idioma, busca ou versão (`LPDOC-06`, `LPDOC-07`, `LPDOC-13`, `LPDOC-05`).
- Não decide conteúdo de sistema: só o processo de quem escreve.
