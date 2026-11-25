# Catálogo de goldens — o que cada byte prova

> Pergunta: "que prova de fio existe hoje, quantos goldens são e o que cada um prova?"
> Público: QA, pesquisa do cliente, desenvolvimento do `wyd-protocol`
> Dono: equipe do `wyd-protocol`
> Base: `wyd-protocol-1.0.0.md` §6.7 e §7; `docs/goldens/como-capturar.md` §7; `lacunas.md` `LPRT-03`/`LPRT-06`; contrato `X7` em `wyd-1.0.0.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Página criada em 2026-11-25 e **vazia por medição**: não existe golden aceito, nenhum arquivo em `tests/golden/` e nenhum código em `crates/wyd-protocol/`. Este é o **dono único** do total e da cobertura por família; nenhuma outra página repete o número.

## 1. Regra do total

O total é a contagem de linhas da tabela de §3, e só isso: um golden entra aqui quando a captura foi aceita e o ID `G-<familia>-<nn>` foi alocado (`wyd-protocol-1.0.0.md` §7). Captura em andamento **não** conta; ficha sem aceite fica em `lacunas.md`, não nesta tabela.

| Grandeza | Valor (2026-11-25) | Como foi medido |
|---|---:|---|
| goldens aceitos | **0** | nenhuma linha em §3 |
| famílias com prova | **0** de 5 | §2, coluna "provas" |
| arquivos em `tests/golden/` | **0** | pasta inexistente no repositório |
| capturas executadas | **0** | `LPRT-03` e `LPRT-06` seguem `bloqueada` em `lacunas.md` §2 |

## 2. Cobertura por família

As cinco famílias são as do plano (`wyd-protocol-1.0.0.md` §3). Nenhuma tem prova hoje; a coluna "o que falta" nomeia a lacuna que destrava o primeiro golden de cada uma, e não promete data.

| Família | Provas | O que falta | Lacuna |
|---|---:|---|---|
| handshake | 0 | origem da `KeyWord` e ordem dos campos na abertura | `LPRT-03` |
| enquadramento | 0 | `Size` mínimo/máximo observado e pacote colado/parcial | `LPRT-04` |
| catálogo de opcodes | 0 | total de `Type` e o que é opaco em cada faixa | `LPRT-05` |
| payloads | 0 | campos, largura e terminador de texto por família | `LPRT-06` |
| tipos e encoding | 0 | sinal, ordem de bytes e string de tamanho fixo | `LPRT-06` |

## 3. Catálogo

| ID | Família | Opcode | Origem | Data | O que prova |
|---|---|---|---|---|---|
| — | — | — | — | — | **nenhum golden aceito**: a tabela fica vazia até a primeira aceitação |

O ID `G-<familia>-<nn>` nasce na aceitação, não na captura: é o catálogo que o aloca, e o número não é reaproveitado depois de aceito. A ficha de origem completa (bytes, ambiente, operador, segunda leitura) fica fora do repositório, no campo 8 de `como-capturar.md` §7.

## 4. O que este catálogo não é

- **Não é o procedimento de captura:** como produzir um golden é de [`como-capturar.md`](como-capturar.md).
- **Não é a lista de lacunas:** o estado de cada prova pendente é de `lacunas.md` §2; aqui só entra o que já foi aceito.
- **Não é dono de opcode nem de offset:** o total de `Type` é de `tabela-de-opcodes.md` e os offsets do `HEADER` são de `header.md` — as duas páginas ainda **não existem** (`wyd-protocol-1.0.0.md` §4 é teto declarado).
- **Não guarda bytes:** o frame anonimizado e a ficha resumida vão para `tests/golden/` quando houver captura aceita; o bruto fica em acesso restrito, fora do repositório (D5).

## 5. Como um golden entra

1. A captura é feita pelo procedimento de `como-capturar.md` e a ficha é preenchida.
2. A segunda leitura aprova; sem ela, nada entra e a lacuna continua `aberta`.
3. O ID é alocado nesta tabela, com família, opcode, origem, data e **o que prova** — a frase tem de dizer o que o byte demonstra, não o que se esperava dele.
4. O frame anonimizado e a ficha resumida entram em `tests/golden/` e o teste que os consome é citado na linha da tabela.
5. O total de §1 e a cobertura de §2 são atualizados na mesma entrega; a lacuna correspondente muda de estado em `lacunas.md`.

## 6. O que falta para esta página deixar de estar vazia

| Item | Estado | Destrava |
|---|---|---|
| captura A1 (handshake e enquadramento) | autorizada em 2026-11-23, **não executada** | operador e segunda leitura; sem eles nenhum ID é alocado |
| captura B/C (payloads e texto) | **não executada** | mesma janela de captura; `LPRT-06` |
| `tests/golden/` com frame anonimizado | não existe | primeira captura aceita |
| teste que consome golden | não existe | `crates/wyd-protocol/` ainda sem código |

Nenhuma linha desta página afirma byte, opcode ou offset: enquanto a tabela de §3 estiver vazia, o layout do fio continua `H` e o dono da hipótese é a lacuna citada em `lacunas.md`.

## Referências

- Plano e teto de páginas: `wyd-protocol-1.0.0.md` §4, §6.7 e §7.
- Procedimento de captura: [`como-capturar.md`](como-capturar.md) §7.
- Lacunas: `lacunas.md` §2 (`LPRT-03`–`LPRT-06`).
- Decisão de build-alvo: `Q-PRT-1` = A (cliente 7662), em `docs/proposta-decisoes-protocol.md`.
