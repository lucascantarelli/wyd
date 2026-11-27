# `ItemList.bin` e `Itemname.bin` — layout provado campo a campo

> Pergunta: "quais são os campos, os tamanhos, os domínios de valor e as anomalias de `ItemList.bin` e `Itemname.bin`?"
> Público: pesquisa (RE), arquitetura e curadoria de seed do `wyd-db`
> Dono: equipe do `wyd-client`
> Base: `../registro-do-cliente.md` §9 (contagens); `LCLI-06` (semântica em aberto)
> Revisado em: 2026-11-27

**Estado:** proposto (2026-11-27), medido (`V`) sobre o cliente 7662 de `wyd-client/files/client/`. Todos os 140
bytes do registro do `ItemList` e os 68 do `Itemname` estão atribuídos aqui; nenhum offset fica sem
dono. Onde a **semântica** do campo não é provável a partir do arquivo, a linha diz isso e aponta
`LCLI-06` — o valor é conhecido, o significado não.

## 1. Como a medição foi feita

Toda a página vem de leitura direta dos dois `.bin`, com script descartável fora do repositório
(`node`, 2026-11-27). O procedimento, reproduzível:

1. **`ItemList.bin`**: decodificar cada byte com `byte ^ 0x5A`; o registro é o bloco de 140 B.
2. **`Itemname.bin`**: o registro é o bloco de 68 B; o nome é `plain[p] = (byte[4 + p] − p) & 0xFF`,
   lido até o primeiro `0x00`.
3. Nome do `ItemList`: `char[64]` em `@0`, lido até o primeiro `0x00`, decodificado em **Windows-1252**.
4. Conferir com três provas independentes: conta do arquivo (`registros × tamanho + cauda = bytes`),
   round-trip byte a byte e coerência de domínio campo a campo (§2.4–§2.9).

A hipótese inicial de layout veio da análise antiga em `deprecated/` (nível 4, `AGENTS.md` §3). Ela
**não** entra como fato: dois pontos dela não se reproduzem neste build — o preenchimento do
`ItemList` não é `0x5A` (é o zero codificado por XOR) e as porcentagens de `Pos`/`Extra`/`Grade`
descritas lá não batem com a contagem medida aqui (§2.9, §2.10). O que vale nesta página é o que a
medição mostrou.

## 2. `ItemList.bin`

### 2.1 Conta do arquivo

`910.004 = 0 + 6.500 × 140 + 4`. **Não há cabeçalho**: o registro 0 começa no byte 0 e os 4 bytes
finais (`ef c7 81 03`) são uma cauda constante, com papel não provado (`LCLI-06`).

### 2.2 Codificação do arquivo

O arquivo inteiro é **XOR `0x5A` por byte**. É por isso que ele "abre com preenchimento `0x5A`": o
`0x5A` é o **zero codificado** (`0x00 ^ 0x5A`). Aplicado o XOR, os slots vazios ficam inteiramente
zerados e os nomes ficam legíveis.

### 2.3 Mapa de bytes do registro (140 B)

Contagens sobre os **3.216 slots com nome**; os 3.284 slots vazios são zero em todos os offsets.

| Offset | Bytes | Tipo | Campo | Domínio medido | Evidência |
|---:|---:|---|---|---|---|
| 0 | 64 | `char[64]` | `name` | 3.216 nomes; 1 a 40 B; NUL até `@63` em 3.216/3.216 | §2.4 |
| 64 | 2 | `i16` | índice de malha | 614 valores; `-1` (`0xFFFF`) em 151; maior real 2.923 | §2.5 |
| 66 | 2 | `u16` | índice de textura | 151 valores: 0 em 2.881, 1…152 (sem 103 e 104) | §2.5 |
| 68 | 2 | `u16` | efeito visual | **0 em 3.216/3.216** — campo morto neste build | §2.5 |
| 70 | 2 | `u16` | nível exigido | 206 valores; 0 em 1.451; máximo 399 | §2.6 |
| 72 | 2 | `u16` | força exigida | 193 valores; 0 em 2.131; máximo 850 | §2.6 |
| 74 | 2 | `u16` | inteligência exigida | 125 valores; 0 em 2.762; máximo 579 | §2.6 |
| 76 | 2 | `u16` | destreza exigida | 58 valores; 0 em 2.969; máximo 348 | §2.6 |
| 78 | 2 | `u16` | constituição exigida | 106 valores; 0 em 2.831; máximo 320 | §2.6 |
| 80 | 48 | `(u16, u16)[12]` | efeitos | 12 pares; uso decrescente do slot 0 ao 11 | §2.7 |
| 128 | 4 | `u32` | preço | 286 valores; 0 em 813; máximo 1.000.000.000; 105 acima de 10⁶ | §2.8 |
| 132 | 2 | `u16` | `unique id` | 53 valores: 1…51 e 145…148; 0 em 1.669 | §2.9 |
| 134 | 2 | `u16` | `pos` | 21 combinações de máscara de bits (bits 0…15) | §2.9 |
| 136 | 2 | `u16` | `extra` | 261 valores; 0 em 2.546; byte alto em {8, 9, 10, 11, 13, 14, 15} | §2.9 |
| 138 | 2 | `u16` | `grade` | 24 valores: 0…26, mais 1 registro anômalo | §2.9, §2.10 |

Todos os offsets ímpares `@81…@127` e `@133`, `@139` são zero em 3.216/3.216 — são a metade alta dos
`u16` que não passa de 255 (e, em `@133`, de 148). Isso é registro de campo, não espaço morto.

### 2.4 Campo `name` (`@0`, 64 B)

| Medição | Valor |
|---|---:|
| slots com nome | 3.216 |
| nomes distintos | 2.328 |
| nomes repetidos em mais de um slot | 888 |
| comprimento mínimo / mediano / máximo | 3 / 16 / 40 B |
| espaço no fim do nome | 0 |
| código de caractere | **Windows-1252**, não UTF-8 |
| pontos de código usados | 70 (ASCII imprimível + acentuadas) |

O charset é CP-1252, provado por três sinais: acentos aparecem como **um** byte (`0xE7` = `ç`,
`0xE3` = `ã`, `0xFA` = `ú`), a decodificação UTF-8 estrita falha em 1.081 nomes, e há um nome com
`0x9C` (`œ`), que só existe em CP-1252. Exemplos medidos: `Titã` (`54 69 74 e3`), `Elmo_de_Aço`,
`Guardião_da_Cidade`.

A repetição de nome (888 slots) é dado, não defeito: o mesmo nome aparece em variantes de slot. A
curadoria precisa decidir a chave antes de semear — `LCLI-06`.

### 2.5 Índices de malha, textura e efeito visual (`@64`…`@69`)

| Campo | Medição | Leitura |
|---|---|---|
| malha `@64` | 614 valores; `0xFFFF` em 151 slots; maior valor real 2.923 | `0xFFFF` = `-1` cai exatamente nas 151 linhas de skill (slots 5.000…5.447), que não têm malha; os valores reais ficam abaixo de 5.000, o número de registros de `Mesh/ItemList.bin` (5.000 × 104 + 4 = 520.004 B) |
| textura `@66` | 151 valores; 0 em 2.881 | subíndice de textura da malha; ausentes 103 e 104 |
| efeito visual `@68` | 0 em 3.216/3.216 | campo existe e está morto neste build |

### 2.6 Requisitos (`@70`…`@79`)

| Campo | Offset | Distintos | Zero | Mínimo | Máximo |
|---|---:|---:|---:|---:|---:|
| nível | 70 | 206 | 1.451 | 0 | 399 |
| força | 72 | 193 | 2.131 | 0 | 850 |
| inteligência | 74 | 125 | 2.762 | 0 | 579 |
| destreza | 76 | 58 | 2.969 | 0 | 348 |
| constituição | 78 | 106 | 2.831 | 0 | 320 |

A ordem dos quatro atributos é a do bloco contíguo `@72…@79`; qual atributo ocupa qual par **não** é
provável só pelo arquivo — o que está provado é que são quatro `u16` de requisito, sem sinal e dentro
de faixa de atributo. Fechar a ordem exige o enum do motor (`LCLI-06`).

### 2.7 Efeitos (`@80`…`@127`, 12 pares)

Cada slot é um par `(id, valor)` de dois `u16`. O par `(0, 0)` significa slot livre. Slots usados e
ids observados:

| Slot | Offset do id | Itens com id ≠ 0 | Ids observados |
|---:|---:|---:|---|
| 0 | 80 | 3.030 | 18, 33, 38, 43 |
| 1 | 84 | 2.703 | 2, 3, 4, 5, 8, 11, 12, 13, 14, 18, 26, 27, 33, 34, 38, 42, 43, 45, 46, 47, 49, 54, 56, 60, 61, 64, 65, 66, 78, 112 |
| 2 | 88 | 1.990 | 2, 3, 4, 5, 7, 11, 12, 13, 14, 18, 27, 34, 36, 38, 39, 43, 44, 46, 47, 48, 50, 54, 56, 57, 58, 60, 61, 112, 116 |
| 3 | 92 | 1.673 | 3, 4, 5, 7, 8, 12, 18, 21, 27, 29, 33, 37, 42, 43, 44, 45, 46, 48, 51, 53, 55, 57, 61, 112, 648 |
| 4 | 96 | 1.409 | 2, 3, 9, 13, 21, 27, 29, 33, 42, 45, 46, 48, 52, 53, 54, 55, 60, 61, 112 |
| 5 | 100 | 758 | 2, 3, 8, 10, 14, 26, 29, 42, 54, 55, 57, 60, 112, 116 |
| 6 | 104 | 505 | 7, 10, 29, 42, 44, 46, 53, 55, 57, 112 |
| 7 | 108 | 386 | 2, 7, 9, 42, 45, 46, 55, 56, 57, 112, 116 |
| 8 | 112 | 124 | 2, 9, 55, 56, 57, 112, 116 |
| 9 | 116 | 19 | 56, 57, 116 |
| 10 | 120 | 11 | 56, 57 |
| 11 | 124 | 1 | 56 |

São **53 ids distintos** no bloco inteiro. Observações que fecham a estrutura:

- par com `id ≠ 0` e valor `0` existe (2.304 pares): id não implica valor.
- par com `id = 0` e valor `≠ 0` também existe (1.836 pares) — o valor é lido independentemente do id,
  então **a posição do par é que manda**, não a ordem dos pares preenchidos.
- no slot 0, o id 18 domina (3.030 itens) e aparece com valor 1 em itens de classe (slots 1…8) e com
  valor 255 em consumíveis e mantos (slots 400…403, `Poção_de_Cura*`). Valor 255 não é magnitude
  pequena: é sentinela.

O que é o id e o que é o valor de cada efeito é `H` (`LCLI-06`): o arquivo entrega a estrutura, não o
catálogo de efeitos.

### 2.8 Preço (`@128`, 4 B)

`u32` sem sinal, 286 valores distintos. 0 em 813 slots; máximo **1.000.000.000**; 105 itens acima de
10⁶. Os valores mais frequentes: `0` (813), `10.000` (169), `500.000` (95), `1.200` (63), `270.000`
(60), `50.000` (49), `650.000` (44), `550.000` (40).

O teto de 1.000.000.000 não é aleatório e aparece em mais de um item; o preço é `u32` cheio, sem
saturação em `u16`.

### 2.9 Cauda: `unique id`, `pos`, `extra`, `grade` (`@132`…`@139`)

**`unique id` (`@132`)** — 53 valores: `1…51`, mais `145`, `146`, `147` e `148` (5 itens). Zero em
1.669 slots. A faixa alta (145…148) aparece só nos slots 2.441…2.444 e 3.386…3.389, o que é
distribuição de dado, não erro de leitura.

**`pos` (`@134`) — máscara de bits de equipamento (`V`).** O domínio é composto só de potências de
dois e combinações delas (0, 1, 2, 4, 8, 16, 32, 64, 128, 192, 256, 512, 1.024, 2.048, 2.667, 3.840,
4.096, 8.192, 16.384, 32.768, 41.248), e a correlação com o grupo do nome é limpa:

| Grupo do nome | `pos` dominante |
|---|---:|
| `Elmo` | 2 |
| `Armadura`, `Túnica` | 4 |
| `Calça` | 8 |
| `Luvas`, `Manoplas` | 16 |
| `Botas` | 32 |
| `Varinha`, `Cajado` | 64 |
| `Machado`, `Arma` | 192 (64 + 128) |
| `Amuleto` | 512 |
| `Pedra` | 2.048 e 3.840 |
| `Medalha`, `Símbolo` | 4.096 |
| `Emblema` | 8.192 |
| `Dragão`, `Cavalo` | 16.384 |
| `Poção`, `Pergaminho`, `Kit`, `Baú`, `Cristal` | 0 |

Ou seja: `pos` é o **conjunto de slots de equipamento** em que o item cabe; 0 é "não equipável". Qual
bit é qual peça é `H` (`LCLI-06`) — a tabela acima dá a correlação, não o nome do bit.

**`extra` (`@136`)** — 261 valores, zero em 2.546 slots. Estrutura medida: o byte alto (`@137`) só
assume `0, 8, 9, 10, 11, 13, 14, 15`, e itens equipáveis (os mesmos de `pos ≠ 0`) carregam o bit
`0x0800`. Os valores vêm em blocos contíguos: `1.221…1.224`, `1.230…1.233`, `1.356…1.359`,
`1.365…1.368`, `1.506…1.509`, `1.515…1.518`, `1.656…1.659`, `1.665…1.668`, `1.710…1.713`,
`2.171…2.250`, `2.451…2.955` (passo 4), `3.464`, `3.601…3.865` (passo 5). O papel desse campo não é
provável pelo arquivo (`LCLI-06`); o que está provado é o domínio e o agrupamento.

**`grade` (`@138`)** — 24 valores: `0` (1.883), `1` (249), `2` (220), `3` (260), `4` (107), `5`
(114), `6` (114), `7` (122), `8` (114), `9` e `10` e `12` (1 cada), `16`…`26` (3, 3, 7, 3, 1, 1, 5,
1, 1, 1, 3). Os valores altos concentram-se em itens de evento e de duração (`Fada_Azul(3dias)`,
`Coroa_Celestial`, `Amuleto_Arcano`), o que sugere classe/raridade do item, não refino — a hipótese
fica em `LCLI-06`.

### 2.10 Cobertura e anomalias

| Medição | Valor |
|---|---:|
| slots do arquivo | 6.500 |
| com nome | 3.216 |
| totalmente zero (vazios) | 3.284 |
| sem nome mas com qualquer byte ≠ 0 | **0** |
| cauda do arquivo | `ef c7 81 03` |

Anomalia única: o slot **1.712** (`Escudo_do_Guardião`) tem `grade = 1.711` (`0x06AF`), valor fora do
domínio de todos os outros 3.215 itens; é o único registro com `@139 ≠ 0` (`0x06`). Registrado como
dado do cliente, não corrigido.

## 3. `Itemname.bin`

### 3.1 Layout

`215.084 = 3.163 × 68`, **sem cabeçalho e sem cauda**. Registro:

| Offset | Bytes | Tipo | Campo | Medição |
|---:|---:|---|---|---|
| 0 | 4 | `u32 LE` | índice do item | 1…5.750; 3.163 valores distintos; estritamente crescente; nunca ≥ 6.500 |
| 4 | 62 | `char[62]` mascarado | nome | 3 a 40 caracteres; terminador `0x00` |
| 66 | 2 | `u16 LE` | trailer | **0 em 3.163/3.163** |

### 3.2 Máscara do nome

`plain[p] = (byte[4 + p] − p) & 0xFF`. O resto do campo é preenchido com o zero **codificado**, isto
é, `byte = p`, e é daí que vem a rampa `0x0B, 0x0C, … 0x3D` observada logo depois do nome. Prova: o
round-trip (decodificar e recodificar com a mesma máscara) reproduz **3.163 de 3.163 registros byte a
byte**.

### 3.3 Charset e cobertura

O nome usa o mesmo **Windows-1252** do `ItemList`: acentos em um byte (`0xE7`, `0xE3`, `0xFA`) e a
mesma presença de `0x9C` (`œ`). 72 pontos de código, no máximo 40 caracteres.

Dos 3.163 registros, **3.109 nomes são idênticos** ao nome do mesmo slot no `ItemList.bin` e **54
divergem**. O arquivo cobre ids até 5.750 (inclui linhas de skill), com 2.587 ids ausentes entre 1 e
5.750 — a numeração é esparsa por desenho, não truncamento.

### 3.4 As 54 divergências

São dado real do cliente, não erro de leitura. Padrões medidos:

| Padrão | Quantos | Exemplo (slot) |
|---|---:|---|
| o `Itemname` nomeia outro item vendável, o `ItemList` mostra o genérico | 12 | 473: `Caixa_Misteriosa` × `Presente_de_Natal` |
| o `Itemname` traz nome de trabalho, o `ItemList` está vazio | 9 | 649 e 650: `not_used` × vazio |
| diferença só de pontuação | 1 | 1731: `Carta_de_Duelo(A)` × `Carta_de_Duelo_(A)` |
| item renomeado entre builds (pares trocados) | 6 | 2400/2417 e 2411/2418 |
| linhas de skill com nome próprio | 2 | 5003/5013: `Samaritano` × `Possuído` |

O restante (24) é renomeação pontual do mesmo item (`Cosmo_Energia_=_200` × `Moeda_WYD(200)`,
`Traje_Mont:_Tigre_Branco` × `Tigre_Branco`, `NONAME` × vazio, `Sem_Nome` × vazio). A escolha de qual
nome vence é da curadoria (`LCLI-06`), não da decodificação.

## 4. O que está provado e o que não está

**Provado (`V`, 2026-11-27):** tamanho de registro e conta dos dois arquivos; codificação do arquivo
(XOR `0x5A`) e máscara do nome (`− p`), ambos com round-trip byte a byte; todos os offsets, tipos,
tamanhos e domínios de valor das §2.3–§2.9; charset CP-1252; cobertura (6.500 / 3.216 / 3.284 e
3.163 / 3.109 / 54); anomalia do slot 1.712.

**Não provado (`H`, `LCLI-06`):** o significado de cada id de efeito e de cada bit de `pos`; o papel
de `unique id`, `extra` e `grade`; qual dos quatro pares `@72…@79` é força, inteligência, destreza e
constituição; o papel dos 4 bytes de cauda do `ItemList`; e qual nome vence quando os dois arquivos
divergem. Nada disso muda o layout: são enums e políticas do motor, que o cliente não carrega.

## 5. Como reproduzir

O script não é versionado (executável não mora em `planejamento-1.0.0/`, `AGENTS.md` §5, item 6). O
procedimento mínimo, em uma linha de `node` sobre `wyd-client/files/client/`:

1. `ItemList`: `clean[i] = raw[i] ^ 0x5A`; registro em `slot * 140`; nome até o primeiro `0x00` dos
   64 B iniciais; campos de §2.3 com `u16 LE`/`u32 LE`.
2. `Itemname`: registro em `k * 68`; índice `u32 LE` em `@0`; nome `(byte[4 + p] − p) & 0xFF` até
   `0x00`; trailer `u16 LE` em `@66`.
3. Conferir: `6.500 × 140 + 4 = 910.004` e `3.163 × 68 = 215.084`.

Nenhum conteúdo do cliente é copiado para o repositório (só contagem, domínio e exemplo pontual),
porque a distribuição segue congelada por `LCLI-05`.
