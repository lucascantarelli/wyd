> Pergunta: "em que versão entra cada issue `WYDPRT` e o que a fatia precisa provar?"
> Público: quem mantém o roadmap e quem opera o board
> Dono: equipe do wyd-protocol
> Base: `wyd-protocol-1.0.0.md` §4, §7 e §9.1; `lacunas.md` §1
> Revisado em: 2026-11-20

**Estado:** proposto (2026-11-20)

Esta página é o **fatiamento das issues do roadmap** por versão de entrega. Ela não decide escopo,
não escreve issue e não conta issue: o escopo e o gate de cada versão são de
`wyd-protocol-1.0.0.md` §9.1, e o total de issues é de §4 do mesmo plano. Aqui fica só a regra que
liga uma issue `WYDPRT-nnn` à versão que ela serve.

## 1. O que já tem dono

| Fato | Dono | Esta página |
|---|---|---|
| escopo e gate de cada versão (`0.1.0` → `1.0.0`) | `wyd-protocol-1.0.0.md` §9.1 | cita, não repete |
| total de issues do roadmap e o que cada uma entrega | `wyd-protocol-1.0.0.md` §4 e §9, onda 8 | cita, não reconta |
| numeração `WYDPRT-nnn` | §7 do plano diretor | aplica, não antecipa número |
| estado de lacuna e de prova | `lacunas.md` e `goldens/catalogo.md` | não fecha lacuna |

**Estado da série: `P`.** Nenhuma versão foi entregue, nenhum opcode foi gerado, nenhuma issue foi
escrita e nenhuma issue foi criada no GitHub. Não há data em nenhuma fatia.

## 2. Regras do fatiamento

1. **Uma issue, uma versão.** A issue pertence à versão cujo gate ela serve. Issue que atravessa duas
   versões é dividida em duas, não arrastada.
2. **A versão não antecipa prova.** Issue de payload não entra antes do golden da família e, em família
   que carrega texto, antes de `LPRT-06` fechar — nome de personagem já atravessa o fio como string.
3. **A ordem é a de §9.1.** A sequência de versões é ordem de entrega, não calendário: não existe
   sprint com data nesta página, e a fatia não vira prazo por estar publicada.
4. **Número só na escrita.** `WYDPRT-nnn` é sequencial, alocado quando a página da issue é escrita, na
   ordem de §7 do plano. Esta página não reserva faixa nem número, e não cria issue.
5. **Contagem não se mexe.** O total de issues é o de §4 do plano; fatiar por versão não cria, não
   apaga e não renumera issue. Mudar a fatia de uma issue é decisão do dono, registrada em
   `lacunas.md` quando a mudança for de lacuna.
6. **Gate é o da versão, não o da issue.** Nenhuma issue fecha uma versão sozinha: quem fecha é o
   critério de §9.1, com golden e leitura registrados.

## 3. Que issues entram em cada fatia

A coluna da direita diz **que trabalho pertence à versão**. Nenhum número de issue é atribuído aqui;
a contagem por versão só existe quando as páginas forem escritas.

| Versão | Que issues entram |
|---|---|
| **0.1.0** | fio sem payload (`HEADER`, enquadramento, transformação, limites, flags), captura do handshake de `LPRT-03` e goldens do fio; nenhuma issue de família |
| **0.2.0** | famílias `autenticacao` e `personagem`, mais o que fecha `LPRT-06` antes delas |
| **0.3.0** | famílias `cena-e-mundo` e `movimento` |
| **0.4.0** | família `combate`, sem regra de jogo |
| **0.5.0** | famílias `inventario` e `comercio` |
| **0.6.0** | família `quest-e-eventos` |
| **0.7.0** | família `ui-e-chat` |
| **0.8.0** | fluxos `game ↔ db`, `db ↔ np` e `np` (`LPRT-19`) |
| **0.9.0** | matriz de capacidade por variante (`LPRT-15`), fuzz agendado (`LPRT-14`) e fechamento das páginas `aceitas` |
| **1.0.0** | congelamento: catálogo medido do gerador, goldens por família e compatibilidade declarada (`LPRT-09`) |

## 4. Como isso chega ao board

A versão é um eixo de entrega, e o board pode usá-lo como campo ou como marco; **como** ele aparece
— milestone, label, sprint — é decisão de `wyd-ops`, dono da gestão de trabalho. Esta página não
propõe nome de sprint, não define data e não substitui o marco `M0`–`M5` de
`wyd-ops/proposta-gestao-github-e-ambientes.md`.

O que a operação pode assumir sem nova decisão: toda issue do roadmap tem exatamente uma versão, e a
ordem das versões é a ordem de entrega. O que ainda não existe: a distribuição numérica das issues por
versão, que nasce com as páginas.

## 5. O que esta página não faz

- não cria issue, label, milestone nem sprint;
- não aloca `WYDPRT-nnn` nem reserva faixa;
- não altera o total de issues nem o teto de páginas do sistema;
- não define data, prioridade de produto ou release;
- não substitui `§9.1` (escopo e gate) nem `§4` (total e conteúdo das issues).
