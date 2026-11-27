---
name: levantar-requisito
description: Transforma pedido vago em requisito numerado do wyd-db (RF-DB/RN-DB) com entrada, saída, tabela, porta e critério de aceitação verificável. Use quando o dono descreve uma necessidade sem número de requisito, ou quando a issue não diz como provar que ficou pronto.
---

# Levantar requisito

## Quando usar

- O pedido chega como frase de produto ("o jogador precisa ver o saldo").
- A issue existe mas não tem critério de aceitação verificável.

Não use para requisito já numerado em `specs/`: ali o trabalho é implementar.

## Pré-requisitos

- `specs/requisitos.md` aberto (numeração e formato correntes).
- A página de domínio de `docs/dicionario-dados/` que o requisito toca.

## Passos

1. **Pergunte em uma rodada curta**: quem usa, o que entra, o que sai, o que acontece no erro, quem
   pode ver e quem pode escrever. Uma pergunta por vez, com opção recomendada.
2. **Escreva o enunciado sem solução**: "o sistema registra X quando Y" e não "criar tabela Z".
3. **Numere**: `RF-DB-nnn` para função, `RN-DB-nnn` para regra. Número novo é o próximo livre, nunca
   reaproveitado.
4. **Diga onde cai**: tabela(s), porta(s) do `specs/` e papel de banco que lê/escreve.
5. **Escreva o critério de aceitação como comando ou dado observável** — `SELECT` que devolve linha,
   `INSERT` que devolve `23514`, teste que falha antes e passa depois.
6. **Abra a issue** só depois de o requisito estar aceito pelo dono.

## Como verificar

O critério de aceitação tem de ser executável: rode-o contra o schema atual e confirme que ele falha
antes da implementação (senão ele não prova nada).

## Quando parar e perguntar

- O requisito muda fronteira entre sistemas ou schema: é decisão do dono, registrada em ADR.
- Não há dado para o critério (conteúdo não medido): registre a lacuna em `lacunas.md`.

## Armadilhas

- **Requisito que já é solução** ("criar coluna `x`") — fecha o desenho antes da análise.
- **Critério vago** ("funciona corretamente") — não verificável, não entra.
- **Numeração inventada** fora da sequência do arquivo dono.
