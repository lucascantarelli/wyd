# Prompts — roteador

Um prompt diz **o que pedir** a um agente: entrada, passos, saída e critério de parada. Como executar
é responsabilidade da skill correspondente.

| Prompt | Para quê | Skill que executa |
|---|---|---|
| [`auditoria-de-cenario.md`](auditoria-de-cenario.md) | medir o repositório e atualizar o panorama sem alterar nada | `auditar-planejamento` |

## Onde cada prompt mora

- **Repositório:** `.agents/prompts/` — vale para qualquer sistema, como auditoria de cenário.
- **Sistema:** `planejamento-1.0.0/<sistema>/prompts/` — procedimento exclusivo daquele sistema,
  como o plano diretor do `wyd-db` prevê. Não crie prompt genérico lá.

## Regras

1. Prompt é específico do WYD. Nada de “escreva documentação de qualidade”.
2. Todo prompt declara o que **não** fazer (escrever no GitHub, fechar lacuna, copiar legado).
3. Saída tem formato fixo: tabela, lista ou arquivo — nunca “um resumo”.
4. Prompt que depende de decisão não pede para decidir: pede para registrar a decisão pendente com
   dono.
5. Não duplique skill: se o prompt precisar de 10 passos de execução, ele aponta a skill.

## Fronteira com o legado

`.agents/prompts/análises` no projeto antigo é **referência**, não planejamento novo. Não copie
prompt de lá; reescreva para a `1.0.0`.
