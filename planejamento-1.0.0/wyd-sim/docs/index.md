# wyd-sim — visão geral

> Pergunta: "o que o `wyd-sim` calcula e por que ele é separado do `wyd-core`?"
> Público: Desenvolvedor novo, QA e produto
> Dono: equipe do wyd-sim
> Base: `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7.2; `wyd-protocol-1.0.0.md` §1.2
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O `wyd-sim` concentra o **cálculo** do jogo em funções puras e determinísticas: dano, chance, curva
de nível, experiência, drop e cooldown. Foi separado do `wyd-core` para que a regra (quando aplicar)
e a fórmula (quanto resulta) tenham donos distintos, e para que a fórmula seja provável por vetor de
entrada e saída, sem subir servidor.

## O que ele não faz

| Fora | Dono |
|---|---|
| decidir quando a fórmula é aplicada | `wyd-core` |
| carregar e versionar o dado de conteúdo | `wyd-db` (`content`) |
| equilibrar o jogo | produto |
| formatar ou arredondar para a tela | `wyd-portal` / `wyd-protocol` |

## Garantia central

Mesma entrada, mesma saída. Nenhuma fórmula lê banco, relógio, ambiente ou estado global — se
precisa de um valor do mundo, ele entra como parâmetro.

## O que falta

Quatro perguntas abertas — catálogo de fórmulas, versão do cálculo, vetores de prova e política
aritmética — estão em `lacunas.md` (planejamento). Nenhuma fórmula está escrita: esta página não
afirma coeficiente, curva nem faixa de dano.

## Limite desta documentação

- Não afirma número de balanceamento: valor sem fonte é hipótese, não regra.
- Não mede desempenho de cálculo: nada foi executado.
## Decisões em aberto

As perguntas que ainda esperam o dono — `LSIM-01`–`LSIM-04` — estão em [Decisões do `wyd-sim`](proposta-decisoes-sim.md). O estado de cada lacuna continua em `lacunas.md`.
