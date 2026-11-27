# `InitItem.bin` — layout medido (unidade de 8 B)

> Pergunta: "o que `InitItem.bin` guarda, com que codificação e quais valores ele entrega?"
> Público: pesquisa (RE), arquitetura e curadoria do `wyd-db`
> Dono: equipe do `wyd-client`
> Base: `formato-itemlist-e-itemname.md` (faixa de slots); `LCLI-07` (semântica em aberto)
> Revisado em: 2026-11-27

**Estado:** proposto (2026-11-27), medido (`V`) sobre o cliente 7662. A **estrutura** está fechada: o
arquivo inteiro é explicado por unidades de 8 B e cada uma entrega quatro valores dentro da faixa de
slots do `ItemList`. O **significado** de cada coluna e a divisão em grupos **não** estão provados e
aparecem como `H` (§5), com `LCLI-07`.

## 1. Conta do arquivo

| Medição | Valor |
|---|---:|
| bytes | 768 |
| unidades de 8 B | **96** |
| resto (`768 mod 8`) | 0 |
| cabeçalho ou cauda | nenhum |

`768 = 96 × 8` fecha sem sobra e sem cabeçalho: a unidade de 8 B é a única divisão que explica o
arquivo inteiro.

## 2. Codificação: complemento byte a byte

O arquivo **não** é legível direto: cada byte está gravado como complemento (`byte ^ 0xFF`). Lido cru,
todo `u16` cai em `0xFF00…0xFFFF`; complementado, o mesmo `u16` cai em `0…4.103` — a faixa de slots do
`ItemList` (§3, `formato-itemlist-e-itemname.md`).

A prova não é a faixa sozinha: **271 dos 384 valores** complementados resolvem para um slot **com
nome** no `ItemList` (94 dos 138 distintos), enquanto nenhum valor cru resolveria, porque todos ficam
fora de `0…6.499`. Exemplo do registro 0: cru `26 ff 28 ff 28 fe fe ff` → complementado
`217, 215, 471, 1` → `Anjo`, `Troll_Martelo_B`, `Entrada_de_Guilda`, `TransKnight`.

## 3. Unidade de 8 B: quatro `u16`

Cada unidade entrega quatro `u16 LE` complementados, sem preenchimento e sem terminador.

| Coluna | Offset | Distintos | Mínimo | Máximo | Amostra do domínio |
|---|---:|---:|---:|---:|---|
| `c0` | 0 | 49 | 64 | 4.061 | 64, 161, 217, 786, 787, 844, 845, 1.060… |
| `c1` | 2 | 63 | 64 | 3.999 | 64, 151, 157, 163, 215, 221, 227, 1.174… |
| `c2` | 4 | 26 | 457 | 4.103 | 457, 458, 459, 461, 462, 463, 464, 468, 471, 472, 746, 757… |
| `c3` | 6 | **4** | 0 | 3 | 0, 1, 2, 3 |

Fatos que fecham a leitura da unidade:

- **384 valores**, todos `< 6.500`; **nenhum** acima da faixa de slots do `ItemList` (§1 do formato).
- **138 valores distintos**; 96 unidades, todas distintas entre si.
- `c3` só assume `0, 1, 2, 3` — é enum pequeno, não índice de slot.
- `c2` concentra objetos de cenário e portas (`Entrada_de_Guilda`, `Primeira_Porta`, `Canhão`,
  `Torre_do_Triunfo_I`), enquanto `c0`/`c1` trazem criaturas e peças de equipamento — a divisão de
  natureza entre as colunas é visível, o significado não (§5).

## 4. Ordem do arquivo

As linhas estão agrupadas por `c2`, não por `c0`: o valor 471 ocupa 12 linhas seguidas, depois 472
ocupa 2, 471 volta por 5, e então `1733`, `1734`, `1735` e `1736` ocupam 4 linhas cada. `c0` também
aparece em séries (217 e 161 por 6 linhas; 2.624 por 5). `c1` é quase sempre distinto de linha para
linha (91 séries em 96 linhas). Amostra das 12 primeiras unidades:

| Linha | `c0` | `c1` | `c2` | `c3` |
|---:|---:|---:|---:|---:|
| 0 | 217 | 215 | 471 | 1 |
| 1 | 217 | 221 | 471 | 1 |
| 2 | 217 | 227 | 471 | 1 |
| 3 | 217 | 151 | 471 | 1 |
| 4 | 217 | 157 | 471 | 1 |
| 5 | 217 | 163 | 471 | 1 |
| 6 | 161 | 215 | 471 | 1 |
| 7 | 161 | 221 | 471 | 1 |
| 8 | 161 | 227 | 471 | 1 |
| 9 | 161 | 151 | 471 | 1 |
| 10 | 161 | 157 | 471 | 1 |
| 11 | 161 | 163 | 471 | 1 |

O padrão dessas 12 linhas é um **produto cartesiano** de dois conjuntos (2 valores em `c0` × 6 em
`c1`), o que é assinatura de tabela de combinação, não de lista de inventário.

## 5. O que **não** está provado (`H`, `LCLI-07`)

- **Significado de cada coluna.** Que os valores sejam índices de slot do `ItemList` é `V`; *o que* o
  slot representa em cada coluna (item, criatura, objeto de mapa, valor de classe) é `H`.
- **A divisão em grupos.** A leitura de "**16 por classe**" (`96 = 6 × 16`) é hipótese de trabalho e
  **não se reproduz nos bytes**: não há periodicidade de 16 (blocos de 16 não são idênticos, nenhuma
  coluna separa os grupos, e as séries de `c2` cortam os blocos). Além disso o `ItemList` deste build
  tem **4** classes (`TransKnight`, `Foema`, `BeastMaster`, `Huntress`), não 6. As duas leituras
  candidatas — `4 × 24` e `6 × 16` — ficam as duas em `H`.
- **`c3`.** Quatro valores, distribuição desigual entre as linhas (22, 55, 11 e 8 ocorrências) e sem
  relação com a posição da linha; não é índice de classe, porque as classes são 4 e a contagem por
  valor não é 24. O que é `c3` é `H`.
- **Uso do arquivo.** Se ele alimenta inventário inicial de personagem, loja de NPC ou mapa é `H`: o
  arquivo não traz referência a conta, personagem ou mapa.

O que fecha esses pontos é o enum do motor (mesma situação de `LCLI-06` para o `ItemList`) ou uma
medição de comportamento com o cliente em execução — nenhum dos dois está no arquivo.

## 6. Como reproduzir

1. Ler `wyd-client/files/client/InitItem.bin` e complementar cada byte (`^ 0xFF`).
2. Cortar em unidades de 8 B e ler quatro `u16 LE` por unidade (§3).
3. Conferir as contas: `768 = 96 × 8`; 384 valores, todos `< 6.500`; `c3 ∈ {0,1,2,3}`.
4. Resolver os valores contra os slots de `ItemList.bin` (§2 de
   `formato-itemlist-e-itemname.md`).

Nenhum conteúdo do cliente é copiado para o repositório além de exemplo pontual: a distribuição segue
congelada por `LCLI-05`. A continuação desta frente é a fatia [`../roadmap/WYDCLI-F3.md`](../roadmap/WYDCLI-F3.md)
(issue [#230](https://github.com/lucascantarelli/wyd/issues/230)).
