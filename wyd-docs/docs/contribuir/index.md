# Contribuir com a documentação

Cada página pertence a um sistema. Você escreve em `planejamento-1.0.0/<sistema>/docs/`; o site
apenas publica. Corrigir texto no HTML gerado ou no `.build` é trabalho perdido: o próximo build
sobrescreve.

> [!TIP] A regra está escrita, não aqui
> As três normas do `wyd-docs` valem como regra desde 2026-11-19 (`V`): forma da página em
> [convenções](../sistemas/wyd-docs/convencoes.md), processo de autoria em
> [guia do autor](../sistemas/wyd-docs/guia-autor/index.md) e veredito em
> [revisão](../sistemas/wyd-docs/revisao.md). Esta página só diz onde cada pergunta é respondida.

## O caminho, em cinco passos

1. Leia `AGENTS.md` — hierarquia de fontes, invariantes e o que exige aprovação.
2. Leia o dashboard do sistema (`<sistema>-1.0.0.md`) e as lacunas dele.
3. Confirme que a página está prevista na árvore do plano diretor — o guia do autor explica o teto.
4. Escreva no sistema dono seguindo as convenções; rode o gate como o guia do autor manda.
5. Abra o PR e peça revisão a quem não escreveu — o critério está em revisão.

## Onde cada pergunta é respondida

| Pergunta | Página dona |
|---|---|
| Que forma a página precisa ter: cabeçalho, estado, evidência, sintaxe, o que reprova? | [convenções](../sistemas/wyd-docs/convencoes.md) |
| Como escrevo e entrego: teto, gate, PR, pedido de revisão? | [guia do autor](../sistemas/wyd-docs/guia-autor/index.md) |
| O que o revisor confere e quando a página fica `aceito`? | [revisão](../sistemas/wyd-docs/revisao.md) |

## O que esta página não repete

Cabeçalho de cinco linhas, tabela de sintaxe, comando do gate e critério de revisão moram nas três
normas. Se a norma mudar, esta página continua certa: ela aponta, não copia.
