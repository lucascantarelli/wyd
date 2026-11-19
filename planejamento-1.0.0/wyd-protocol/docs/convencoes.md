> Pergunta: "como leio uma página de fio e o que cada marca significa?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-protocol
> Base: —
> Revisado em: 2026-11-19

**Estado:** proposto (2026-11-19)

Esta página é o contrato de **leitura** da documentação do `wyd-protocol`. As marcações de evidência
do projeto (`V`, `H`, `P`) valem aqui como valem no `wyd-db`; o que esta página acrescenta são as
regras que só existem quando a afirmação é sobre **byte**.

> [!TIP] A regra que resolve quase tudo
> Se a frase afirma offset, tipo, tamanho ou valor e não aponta um golden, uma captura datada ou
> uma leitura do cliente, ela é hipótese. O verbo correto é "pretende", não "é".
## Marcações de evidência

| Marca | Significa | O que você pode concluir |
|---|---|---|
| `V` | provado por golden, captura datada, leitura do cliente com endereço ou código que existe | o fato foi medido; pode apoiar decisão |
| `H` | hipótese explícita | não é regra; a página diz qual `LPRT-nn` a fecha |
| `P` | planejado: ainda não existe | descreve intenção |

`H` sem ID é omissão, não hipótese: sem o ID, o leitor não tem o que cobrar.

## Proveniência de byte

Toda afirmação de layout carrega a origem, no formato:

```text
V — golden G-auth-01 (tests/golden/, capturado 2026-11-20 do cliente 7662)
V — cliente 7662, leitura em 0x0040A1C2 (registro de RE, 2026-11-22)
H — LPRT-04 (sem captura de string acentuada)
```

Três origens são aceitas: **golden**, **captura** e **leitura do cliente**. As três exigem data.
"O legado faz assim" não é origem: `deprecated/crates/wyd-protocol/` é nível 4 de `AGENTS.md` §3,
inspira a pergunta e não responde por ela.

## Direção é parte da afirmação

Fluxo `client → game`, `game → client`, `game ↔ db` e `np` não se misturam na mesma tabela sem
coluna de direção. Opcode que existe em duas direções tem duas linhas, cada uma com seu payload.
Tabela de campo sem direção reprova.

## Catálogo gerado × semântica escrita

| O que | Dono | Regra |
|---|---|---|
| `Opcode`, flags, `KEY_TABLE` | gerador versionado | valor não se digita à mão em página; a linha cita o gerador |
| nome de mensagem, família, observação | texto | toda linha aponta payload e golden |
| offset, tipo, tamanho, faixa | página de payload | quatro colunas obrigatórias, com proveniência |

Página que copia valor de opcode do gerador cria a segunda boca que diverge na primeira mudança:
reprova.

## Estados

`**Estado:** <estado> (AAAA-MM-DD)` na primeira linha depois do cabeçalho. Estados: `proposto`,
`aceito`, `substituído`, `retirado`. `aceito` exige revisão de quem não escreveu.

## Números

Número sem fonte e data é hipótese. Formato:

```text
198 opcodes no legado (deprecated/crates/wyd-protocol/src/wire/generated/opcode_table.rs, 2026-11-19)
12 bytes de HEADER (deprecated/crates/wyd-protocol/src/wire/header.rs:18, 2026-11-19)
```

Números do legado descrevem o que a reescrita substitui; não são meta da `1.0.0`. O total de opcodes
da `1.0.0` é o que `docs/wire/tabela-de-opcodes.md` medir depois de `LPRT-01` fechar.

## Links e citações

| Situação | Como aparece |
|---|---|
| outra página desta árvore | link relativo: `../wire/header.md` |
| código do crate | texto simples: `crates/wyd-protocol/src/wire/header.rs:18` |
| página planejada | caminho em `code`, sem link: `wire/tabela-de-opcodes.md` |
| golden | ID em `code` + página que o descreve |

Link relativo para fora de `docs/` reprova: o site publica só `docs/`.

## Sintaxe da página

A página publicada usa só o que a 2.0 renderiza: GFM (parágrafo, lista, tabela, cerca de código com
linguagem, nota de rodapé) mais o que o tema injeta — citação `[!TIPO]` vira aviso e
`<div class="cartoes"></div>` antes de uma lista vira grade. O conjunto completo e a lista de
proibidos são de `wyd-docs-1.0.0.md` §6.

| Recurso | Sintaxe | Quando usar |
|---|---|---|
| Aviso | `> [!TIP]`, `> [!NOTE]`, `> [!WARNING]`, `> [!DANGER]` | regra de byte, risco ou resumo de seção |
| Tabela | pipes | comparar 2+ itens por atributo; tabela de campo traz as colunas obrigatórias |
| Nota de rodapé | `[^1]` no texto e `[^1]: fonte` no fim | fonte externa citada no texto |
| Código | cerca com linguagem | trecho que o leitor vai reproduzir |
| Lista de cartões | `<div class="cartoes"></div>` na linha anterior à lista | índice com 4+ páginas |
| Emoji | `🗄️` Unicode direto no texto | sinalizar tipo; um por título |
| Diagrama | cerca `mermaid` | relação que a tabela não mostra (seção abaixo) |

Admonition `!!!`, aba `=== "…"`, detalhe `???`, attr-list `{ … }`, grade de cartões e ícone
`:material-…:` **não renderizam** na 2.0: página que os usa reprova.

## Diagramas

Mermaid, um tipo por diagrama, rótulo em português. Fluxo de pacote é `sequenceDiagram`; estado do
buffer é `stateDiagram-v2`; componente é `flowchart`. Diagrama que não renderiza no build é defeito
bloqueante.

## O que reprova uma página

- ☐ offset, tipo ou valor sem proveniência datada;
- ☐ `H` sem `LPRT-nn`;
- ☐ tabela de campo sem `offset`, `tipo`, `tamanho`, `faixa`, `obrigatório` e `proveniência`;
- ☐ afirmação de direção sem fluxo;
- ☐ valor de opcode digitado onde existe tabela gerada;
- ☐ total de opcode, golden ou payload sem fonte e data;
- ☐ placeholder, link quebrado, segredo ou captura com dado de jogador;
- ☐ página sem segunda leitura.
