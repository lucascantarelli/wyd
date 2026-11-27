# `InitItem.bin` — layout medido (unidade de 8 B)

> Pergunta: "o que `InitItem.bin` guarda, com que codificação e quais valores ele entrega?"
> Público: pesquisa (RE), arquitetura e curadoria do `wyd-db`
> Dono: equipe do `wyd-client`
> Base: `formato-itemlist-e-itemname.md` (faixa de slots); `../registro-do-cliente.md` §6.1 (evidência estática); `LCLI-07` (semântica em aberto)
> Revisado em: 2026-11-27

**Estado:** proposto (2026-11-27), medido (`V`) sobre o cliente 7662. A **estrutura** está fechada: o
arquivo inteiro é explicado por unidades de 8 B e cada uma entrega quatro valores dentro da faixa de
slots do `ItemList`. A **medição cruzada** com os outros arquivos do cliente está fechada (§5): o
arquivo é espelho de um CSV de servidor, não é loja de NPC, não é mapa nem classe, e as duas divisões
em grupos — `4 × 24` e `6 × 16` — estão refutadas. O que resta é o **rótulo** de cada coluna e do
enum `c3`, que só o motor entrega (§6, `LCLI-07` residual).

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

A prova não é a faixa sozinha: **216 dos 288 valores** de `c0`…`c2` complementados resolvem para um
slot **com nome** no `ItemList` (93 dos 134 distintos), enquanto nenhum valor cru resolveria, porque
todos ficam fora de `0…6.499`. Somando as 55 linhas em que `c3 = 1` alcança o slot 1, o arquivo chega
a 271 dos 384 valores com nome — mas essa parcela é coincidência de numeração, não prova de que `c3`
seja índice de slot (§5.5). Exemplo do registro 0: cru `26 ff 28 ff 28 fe fe ff` → complementado
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
- **134 valores distintos** em `c0`…`c2` (49 + 63 + 26), mais os quatro de `c3`; 96 unidades, todas
  distintas entre si.
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

## 5. Medição contra os outros arquivos do cliente (2026-11-27)

Tudo abaixo foi medido nesta estação, em 2026-11-27, sobre o build-alvo `WYD.exe`
`04c9d2a1…` (`../registro-do-cliente.md` §4) e `InitItem.bin` `edabf523…`. Os comandos estão na §7.

### 5.1 O arquivo é espelho binário de uma tabela do servidor

A tabela de strings do `WYD.exe`, no bloco que converte CSV em BIN, traz as duas pontas:

| Ordem no bloco | String medida | Leitura |
|---|---|---|
| 1 | `InitItem.bin` | arquivo de saída |
| 2 | `Can't write inititem.bin` | erro de escrita |
| 3 | `Can't read inititem` | erro de leitura |
| 4 | `./InitItem.csv` | entrada local |
| 5 | `../../TMSRV/Run/InitItem.csv` | entrada do servidor |
| 6 | `There is no file` | ausência do CSV |
| 7 | `%d %d %d %d` | **quatro inteiros por linha** |

O mesmo bloco traz `itemlist.csv`, `extraitem.csv`, `itemname.csv`, `./SkillData.csv` e
`../../TMSRV/Run/SkillData.csv` — é a mesma rotina de conversão das outras tabelas do cliente.
Três consequências, todas `V`:

1. **A unidade tem quatro inteiros**, o que corrobora os quatro `u16` da §3 sem depender da faixa de
   slots.
2. **A origem é um CSV de servidor** (`TMSRV/Run/InitItem.csv`) que **não acompanha o cliente**: a
   busca por `*.csv` em `wyd-client/files` devolve só `client/UI/TOTOGame.csv`, que é o cartão de
   loteria e não tem relação.
3. **O significado não mora no cliente.** O binário é o espelho de uma tabela do servidor; é lá que
   os quatro campos têm nome, e nenhum arquivo do cliente traduz esses nomes.

### 5.2 Nenhuma segunda cópia e nenhuma tabela derivada

| Teste | Resultado |
|---|---|
| varredura de 266 arquivos `.bin`, `.dat`, `.txt` e `.csv` do cliente (menores que 8 MB, exceto o próprio `InitItem.bin`) por quatro linhas amostradas, nos bytes complementados | **0 ocorrências** |
| `itemicon.bin` | 26.000 B = **6.500 × 4** (dois `u16` por slot; amostra 0/65.535) — tabela por slot, não carrega linha do `InitItem` |
| nomes dos 134 valores procurados em `minimap.dat` | **0** nomes iguais (§5.4) |

Não existe no cliente outra tabela que nomeie ou agrupe as 96 linhas: o arquivo é a única cópia do
conteúdo.

### 5.3 Não é loja de NPC

| Medição | Valor |
|---|---|
| `c2` com nome no `ItemList` | **96/96** |
| `c2` com preço > 0 | **0/96** |
| nomes distintos de `c2` | **22**, todos de estrutura de mapa (portão, porta, canhão, torre, runa, entrada de guilda, janela, portão temporal, caça-níquel) |
| `c0` equipável (`pos ≠ 0`) | 58 de 96 |
| `c1` equipável (`pos ≠ 0`) | 41 de 96 |
| criaturas entre os valores | `Anjo`, `Urso`, `Serpente_Gigante`, `Troll_Martelo_B`, `Grifo_Sangrento`, `Cria_de_Unicórnio` |
| NPC entre os valores | `Comerciante_Arnold` |

Uma lista de estoque de loja é uma lista de itens vendáveis. Aqui a mesma linha carrega peça de
equipamento, criatura, NPC e estrutura de mapa, e não existe coluna de preço, estoque, quantidade ou
moeda — as três colunas de índice param em 4.103 e a quarta é um enum de 0 a 3. **Refutado como loja**
(`V` negativo).

### 5.4 Não é mapa

| Medição | Valor |
|---|---|
| `minimap.dat` | 124 marcas; 118 com duas coordenadas numéricas; `x` de 148 a 3.649 e `y` de 1.273 a 4.077; 101 nomes distintos (NPCs e lugares) |
| nomes iguais entre `InitItem` e as marcas | **0** |
| pares `(c0, c1)` iguais a uma marca | **0** |
| `Env/Field0815.map`, `0816`, `0915`, `0916` | 53.057 B cada, com cabeçalho `00 08 0f ec` seguido de nomes `Tile00000`… — grade de tiles, não tabela de registros de 8 B |

Só duas coincidências de substring aparecem e nenhuma liga linha a marca: `Calça_de_Terah` contém
`Tera` e `Comerciante_Arnold` contém `Arnold`. O arquivo não posiciona objeto em mapa. **Refutado como
mapa** (`V` negativo).

### 5.5 Não é classe

- O `ItemList` tem **24 slots de classe** (seis por classe): `TransKnight` em 1, 6–9 e 41; `Foema` em
  11, 16–19 e 42; `BeastMaster` em 21, 26–29 e 43; `Huntress` em 31, 36–39 e 44. **Nenhum** valor do
  `InitItem` cai em um desses slots (interseção vazia).
- `c3` tem quatro valores, mas a distribuição é **22 / 55 / 11 / 8**. `96 = 4 × 24` exigiria 24 em cada
  valor; as 12 primeiras linhas (produto cartesiano 2 × 6 da §4) são todas `c3 = 1`.
- **Armadilha medida:** `c3 = 1` resolve para o slot 1, que é `TransKnight`. É daí que saem 55 dos 271
  valores "com nome" da §2 — coincidência de numeração, não vínculo com classe.
- Perfil de requisito por bloco de 24 (só equipamento): bloco 0 dex, bloco 1 str/int/dex, bloco 2 str,
  bloco 3 dex. Não separa quatro classes.

**Refutado como eixo de classe** (`V` negativo).

### 5.6 Agrupamento: nem `4 × 24` nem `6 × 16`

| Teste | Resultado |
|---|---|
| blocos de 16 B × 6 iguais entre si | **6 blocos distintos** |
| blocos de 24 × 4 iguais entre si | **4 blocos distintos** |
| coluna que marque início ou fim de grupo | **nenhuma** |
| ordem observável | **29 sequências contíguas de `c2`**, a maior com 21 linhas (`Canhão`); `Entrada_de_Guilda` ocupa 23 linhas em três sequências (12 + 5 + 6) |

As duas leituras candidatas estão **refutadas**: não há repetição de bloco que justifique grupo de 16
ou de 24, e a única ordem do arquivo é a sequência de `c2`. A ordem existe; o corte em grupos, não.

### 5.7 O que cada coluna carrega — fato medido, sem rótulo

| Coluna | Nomeados | Equipável (`pos ≠ 0`) | Preço > 0 | Composição medida |
|---|---:|---:|---:|---|
| `c0` | 63/96 | 58 | 45 | peça de equipamento, criatura e um NPC; **33 vazios** |
| `c1` | 57/96 | 41 | 33 | mesma mistura; **39 vazios** |
| `c2` | 96/96 | 5 | 0 | **só** estrutura de mapa |
| `c3` | — | — | — | enum de 0 a 3; 0, 2 e 3 caem em slot vazio |

Duas consequências estruturais:

- **`c2` é obrigatório e `c0`/`c1` são opcionais.** As 15 linhas com as duas primeiras colunas vazias
  existem, e **100%** dos alvos vazios são registro todo zero no `ItemList` — ou seja, "sem valor", não
  "slot sem nome".
- **As duas colunas de equipamento não formam um conjunto vestível consistente**: em 34 linhas `c0` e
  `c1` são equipáveis, e em 7 delas as duas máscaras `pos` se sobrepõem (mesmo slot disputado duas
  vezes). Isso refuta ler a linha como *loadout* de personagem — a leitura continua `H`.

## 6. O que continua em aberto — só o motor (`H`, `LCLI-07` residual)

O cliente entrega o valor e a ordem; não entrega o nome do campo. Restam três perguntas, e nenhuma
se fecha com arquivo do cliente — a §5.1 mostra que a tabela é espelho de `TMSRV/Run/InitItem.csv`:

1. **Papel das três colunas de índice.** Qual o servidor lê como criatura, qual como item e qual como
   estrutura de mapa. A composição medida (§5.7) é indício forte, mas não é enum: só a tabela do
   servidor nomeia a coluna.
2. **Enum `c3` (0…3).** Domínio e distribuição provados; o significado (equipe, estado, fase ou
   contador) não.
3. **Quem lê as 96 linhas e em que ordem.** A sequência por `c2` é do arquivo, não uma regra provada.

Fechar isso exige o enum do servidor (`TMSRV`, a mesma `InitItem.csv` que o cliente converte) ou uma
medição de comportamento com cliente e servidor em execução. Nenhum dos dois está no repositório; é a
mesma situação de `LCLI-06` para o `ItemList`.

## 7. Como reproduzir

1. Ler `wyd-client/files/client/InitItem.bin` e complementar cada byte (`^ 0xFF`).
2. Cortar em unidades de 8 B e ler quatro `u16 LE` por unidade (§3).
3. Conferir as contas: `768 = 96 × 8`; 384 valores, todos `< 6.500`; `c3 ∈ {0,1,2,3}`; 96 linhas
   distintas.
4. Resolver os valores contra os slots de `ItemList.bin` (§2 de `formato-itemlist-e-itemname.md`) e
   conferir a §5: `c2` sempre nomeado e sem preço, blocos de 16 e de 24 todos distintos, 29 sequências
   de `c2`, vazios de `c0`/`c1` sempre em registro zero.
5. Ler as strings do conversor no executável (offset e conteúdo, sem copiar conteúdo do cliente):

   ```bash
   cd wyd-client/files/client
   py -3 -c "import re; raw=open('WYD.exe','rb').read(); print(' | '.join(s.decode() for s in re.findall(rb'[ -~]{3,}', raw[0x222400:0x222900])))"
   ```

   O bloco mostra `InitItem.bin`, `./InitItem.csv`, `../../TMSRV/Run/InitItem.csv` e o formato
   `%d %d %d %d` ao lado de `itemlist.csv` e `SkillData.csv`.
6. Conferir as marcas de mapa (`minimap.dat`, 124 marcas) e o `itemicon.bin` (6.500 × 4) contra os
   valores do arquivo.

Nenhum conteúdo do cliente é copiado para o repositório além de exemplo pontual: a distribuição segue
congelada por `LCLI-05`. A continuação desta frente é a fatia [`../roadmap/WYDCLI-F3.md`](../roadmap/WYDCLI-F3.md)
(issue [#230](https://github.com/lucascantarelli/wyd/issues/230)).
