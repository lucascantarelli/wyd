# `wyd-protocol` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-20).** Este é o **único** lugar onde uma lacuna do `wyd-protocol`
> 1.0.0 tem dono, impacto, marco e estado. Documento que divergir deste registro está errado até
> prova em contrário.
>
> **O que este arquivo cobre:** as **20 lacunas** `LPRT-01`–`LPRT-20` do planejamento do protocolo.
> **20 decisões foram aprovadas pelo dono** — 17 em 2026-11-20, `LPRT-09` em 2026-11-23 e
> `LPRT-03`/`LPRT-06` em 2026-11-25. **18 linhas estão fechadas e 2 seguem bloqueadas como prova**
> (`LPRT-03`/`LPRT-06`): a política está decidida, mas a prova de byte depende de captura. Nenhuma
> decisão aqui inventa byte: as pendentes de prova seguem `H` até o golden ou a leitura datada.
>
> **O que este arquivo não é:** não decide. Cada linha diz **quem** decide e **o que** precisa ser
> decidido; a decisão entra no documento dono (ou em ADR, quando estrutural) e volta aqui como linha
> fechada, com data e evidência.

---

## 0. Como ler

### 0.1 Estados

| Estado | Significado |
|---|---|
| **fechada** | decidida ou produzida, com a evidência citada na coluna própria |
| **bloqueada** | falta decisão, execução ou evidência; a linha nomeia o desbloqueio (dono e documento) |

### 0.2 Marcos (marco, não data)

Prazo é a onda do plano diretor (§9) antes da qual a lacuna precisa fechar. Não há calendário.

| Marco | Significa |
|---|---|
| **A** | antes de **aprovar este plano** — decide escopo do sistema |
| **W2** | antes da onda do fio sem payload (`wire/header`, `enquadramento`, `keyword-e-checksum`) |
| **W3** | antes do catálogo de opcodes |
| **W4** | antes das páginas de payload |
| **W5** | antes de goldens e `tests/` |
| **W6** | antes de requisitos e fluxos |
| **EXT** | dono fora do `wyd-protocol` |

### 0.3 Donos

| Dono | Decide |
|---|---|
| **produto** | quais fluxos entram na `1.0.0` e o que o jogador vê |
| **arquitetura** | forma do crate, fronteira com `wyd-net`/`wyd-core`, o que vira tipo |
| **frente `wyd-net`** | reação a pacote inválido, timeout, limite por conexão |
| **frente `wyd-core`** | semântica de campo e efeito no tick |
| **frente `wyd-client`** | captura, `strdef.bin`, nome de mensagem no cliente |
| **pesquisa (RE)** | leitura do cliente e prova de offset |

\"arquitetura\" aqui significa **eu proponho, você aprova**. Nenhuma linha deste registro foi fechada
por decisão minha sozinha.

### 0.4 Baseline de referência

Medido em **2026-11-19** em `deprecated/crates/wyd-protocol/`: 26 arquivos `.rs`, 198 variantes de
`Opcode`, 7 flags de direção, `HEADER_SIZE = 12`, `MAX_MESSAGE_SIZE = 8192`, 14 testes. O legado é
**referência** (nível 4 de `AGENTS.md` §3): serve para formular a pergunta, não para respondê-la. Cada
número do legado que entrar numa página nova precisa de `V` na fonte nova.

**198 é baseline, não total alvo.** O número mede o inventário do legado naquela data e vale como
**piso de cobertura** — todo opcode que o legado tem precisa existir no catálogo novo —, não como
tamanho do catálogo publicado. A contagem publicada é a do gerador novo, com proveniência por linha
(`LPRT-01`, `LPRT-16`, `LPRT-20`), e a cobertura chega à `1.0.0` em versões menores (plano diretor
§9.1).

---

## 1. Situação em uma linha

| Grupo | Lacunas | Bloqueadas | Fechadas |
|---|---:|---:|---:|
| Escopo e contrato | 4 | 0 | 4 (`LPRT-01`, `LPRT-18`, `LPRT-19`, `LPRT-09`) |
| Fio e codec | 6 | 2 | 4 (`LPRT-03` e `LPRT-06` bloqueadas como prova; `LPRT-02`, `LPRT-05`, `LPRT-07`, `LPRT-08` fechadas) |
| Payload e prova | 4 | 0 | 4 (`LPRT-04`, `LPRT-10`, `LPRT-11`, `LPRT-13`) |
| Qualidade e entrega | 6 | 0 | 6 (`LPRT-12`, `LPRT-14`, `LPRT-15`, `LPRT-16`, `LPRT-17`, `LPRT-20`) |
| **Total** | **20** | **2** | **18** |

As 18 fechadas são **decisões do dono**: 17 registradas em 2026-11-20 e `LPRT-09` em 2026-11-23; a
proposta correspondente está no plano diretor §10. Fechar a lacuna não fecha a prova: página de `wire/`, `payloads/` ou `goldens/`
continua exigindo `V` (golden, captura ou leitura do cliente) nas afirmações de byte.

**A1 autorizada em 2026-11-23 (resposta A).** O dono autorizou a implementação do peer da opção A, do
`ECAP` e da CLI, e a execução da captura A1. A decisão de política de `LPRT-03` e `LPRT-06` foi aprovada
em 2026-11-25 (decididas), com a **recomendação do bloco — opção A em ambas — aceita pelo dono na mesma data**; a prova de byte segue dependente da captura completa, da segunda leitura e dongolden (`LPRT-03`), e das capturas B/C e do seed sintético (`LPRT-06`).

---

## 2. Lacunas

### 2.1 Escopo e contrato

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPRT-01` | Que opcodes o catálogo novo cobre e como a cobertura chega à `1.0.0`? | define o tamanho do catálogo, das páginas de payload, dos goldens e da suíte; sem isso o roadmap não fatia | produto + arquitetura | `A` | fechada | **decisão do dono (2026-11-20): o catálogo novo cobre todo o inventário do legado e não se limita a ele; nada fica fora da série.** Os **198 são baseline do legado** (§0.4), portanto **piso de cobertura, não total alvo**: a contagem publicada vem do gerador novo, com proveniência por linha (`LPRT-16`, `LPRT-20`). A entrega é fatiada em versões menores, da `0.1.0` à `1.0.0` (plano diretor §9.1), e a `1.0.0` fecha com cobertura integral Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-18` | O crate entrega payload tipado ou bytes crus a `wyd-core`? | muda a API e a fronteira: offset saindo do crate reabre a possibilidade de cada sistema interpretar o fio por conta própria | arquitetura + frente `wyd-core` | `W6` | fechada | **decisão do dono (2026-11-20): payload tipado e validado**; bytes crus não saem do crate — contrato em §1.4 do plano diretor Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-19` | Os fluxos `game ↔ db` (`FLAG_GAME2DB`, `FLAG_DB2GAME`, `FLAG_DB2NP`, `FLAG_NP2DB`) entram no escopo? | `wyd-db` é acessado pelo servidor, não por CPSock na `1.0.0`; incluir fluxo sem consumidor cria contrato morto | produto + arquitetura | `A` | fechada | **decisão do dono (2026-11-20): os fluxos `game ↔ db`, `db ↔ np` e `np` entram no escopo**; as 7 flags são documentadas em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10, com a ressalva de que ainda não há consumidor medido na arquitetura nova |
| `LPRT-09` | Qual build do cliente é o alvo e o que muda em relação ao build anterior? | o legado declara `APP_VERSION = 7640` e o repositório fala em cliente 7662; sem versão-alvo, offset e mensagem ficam ambíguos | frente `wyd-client` + pesquisa | `W3` | fechada | **decisão do dono (2026-11-23, `Q-PRT-1` opção A): build-alvo é o cliente 7662, identificado por versão e SHA-256 completos; qualquer outro build é não suportado.** Identidade medida em `wyd-client/registro-do-cliente.md` §4 (2026-11-22). "O que muda em relação ao build anterior" continua não medido: é leitura datada de `docs/goldens/como-capturar.md` §5, marcada `H`, não decisão pendente |

### 2.2 Fio e codec

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPRT-02` | O servidor aceita checksum divergente? | o legado tolerava; tolerar em produção esconde corrupção e abre margem para pacote forjado; recusar pode quebrar cliente real | arquitetura + frente `wyd-net` | `W2` | fechada | **decisão do dono (2026-11-20): recusar checksum divergente, sem modo tolerante**; o legado tem as duas políticas (`deprecated/crates/wyd-protocol/src/codec/checksum.rs`, 2026-11-19) e a `1.0.0` fica estrita Refletida em `planejamento-1.0.0/wyd-protocol/docs/arquitetura/componentes.md` §"codec". |
| `LPRT-03` | De onde vem a semente (`KeyWord`) da ofuscação? | define se `KeyWord` é escolhido pelo servidor, herdado do cliente ou constante; muda o handshake e o teste | pesquisa + arquitetura | `W2` | **bloqueada como prova** (2026-11-25, decisão aprovada pelo dono: o crate recebe a `KeyWord` do chamador e não assume origem; falta a captura e a segunda leitura para fechar a prova de byte) — **bloqueio medido em 2026-11-25:** não existe `ECAP` nem `PEER-A1` para o cliente conectar (0 `Cargo.toml` e 0 `src` em `crates/wyd-server`, `crates/wyd-protocol` e `crates/wyd-net`; `wyd-client/tools/` ausente), e sem endpoint operado não há tráfego legítimo do cliente (`wyd-client/registro-do-cliente.md` §7). Destrava: implementar `ECAP`+CLI (frente `wyd-server`/`wyd-client`), operador nomeado em `LCLI-03` e segunda leitura de QA. | **decisão do dono (2026-11-20): o crate recebe a `KeyWord` do chamador e não assume origem**; legado usa `KEY_TABLE` com 16 pares (`deprecated/crates/wyd-protocol/src/codec/generated/keyword_table.rs`, 2026-11-19) e a regra de escolha continua não medida no cliente |
| `LPRT-05` | O layout é lido por `repr(C)` ou por leitura explícita offset a offset? | `repr(C)` depende de compilador e de padding; leitura explícita é mais lenta de escrever e mais estável. Muda `unsafe`, portabilidade e prova | arquitetura | `W2` | fechada | **decisão do dono (2026-11-20): leitura explícita offset a offset, sem `unsafe`**; o legado mistura `#[repr(C)]` e leitura explícita (`src/wire/header.rs:74`, 2026-11-19) Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-07` | O que o codec faz com frame exatamente em 8192 e acima disso? | limite é defesa contra amplificação; a reação correta depende de `wyd-net` e do cliente | arquitetura + frente `wyd-net` | `W2` | fechada | **decisão do dono (2026-11-20): aceitar `Size = 8192` e recusar acima com erro tipado**; `MAX_MESSAGE_SIZE = 8192` no legado (`src/wire/header.rs:22`); a borda ainda exige golden, pelo processo de `LPRT-10` Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-06` | Qual encoding, tamanho e terminador de string o cliente usa? | nome de personagem e mensagem de chat atravessam o fio; errar encoding corrompe dado e cria bug intermitente | pesquisa + frente `wyd-core` | `W4` | **bloqueada como prova** (2026-11-25, decisão aprovada pelo dono: o codec não transcodifica; string é bytes de tamanho fixo declarado; faltam as capturas B/C para fechar a prova de byte) — **bloqueio medido em 2026-11-25:** as capturas B/C usam o mesmo `ECAP`/`PEER-A1` que não existe e exigem personagem sintético criado no servidor; sem endpoint operado não há texto acentuado atravessando o fio. Destrava: o mesmo `ECAP`+CLI de `LPRT-03`, depois o operador e a segunda leitura. | **decisão do dono (2026-11-20): o codec não transcodifica; string é bytes de tamanho fixo declarado**; encoding e terminador fecham com captura de texto acentuado — nenhuma existe hoje e o legado não declara encoding na API pública |
| `LPRT-08` | O que fazer com `Type` fora do catálogo? | descartar em silêncio esconde incompatibilidade; derrubar conexão transforma mensagem nova em queda | frente `wyd-net` + arquitetura | `W3` | fechada | **decisão do dono (2026-11-20): o codec devolve `UnknownType` tipado e não decide**; reação (log, descarte ou encerramento) é de `wyd-net` Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |

### 2.3 Payload e prova

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPRT-04` | Qual é a fonte de verdade do layout de payload: cliente, captura ou legado? | é a lacuna que decide se o projeto copia o legado ou reverifica o fio; sem ela nenhuma página de `payloads/` é `aceita` | arquitetura + pesquisa | `W4` | fechada | **decisão do dono (2026-11-20): cliente + captura são a fonte de verdade; o legado é só `H`**; nenhuma página de `payloads/` é `aceita` sem golden por família; o legado tem payloads por família, sem proveniência por campo (`src/payload/*`, 2026-11-19) Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-10` | Como um golden é capturado, anonimizado e aprovado? | golden é a unidade de prova do sistema; sem processo, prova vira opinião e dado de jogador pode vazar | frente `wyd-client` + QA + arquitetura | `W5` | fechada | **processo aprovado pelo dono (2026-11-20):** captura datada, anonimização de conta/nome/IP, ID `G-<familia>-<nn>` em `tests/golden/`, origem registrada e segunda leitura antes de aceitar; a execução das capturas continua pendente; o legado tem 10 testes de golden sem registro de origem (`deprecated/crates/wyd-protocol/tests/golden/pacotes.rs`, 2026-11-19) Refletida em `planejamento-1.0.0/wyd-protocol/docs/goldens/como-capturar.md` §"Regras que valem para as três". |
| `LPRT-11` | Qual é a semântica de `FLAG_NEW` e das flags de direção no roteamento? | valor de fio é composto por flag; interpretar errado faz mensagem cair no fluxo errado | arquitetura + frente `wyd-net` | `W3` | fechada | **decisão do dono (2026-11-20): as flags de direção roteiam; `FLAG_NEW` é marcador de versão e não roteia; combinação inválida é erro tipado**; posição de bit segue sem leitura no cliente (7 flags no legado, `opcode_table.rs:9-15`, 2026-11-19) Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-13` | Qual é a taxonomia final de erro e o mapeamento para reação na conexão? | erro do codec precisa de decisão única; duplicar em `wyd-net` cria duas políticas | arquitetura + frente `wyd-net` | `W6` | fechada | **decisão do dono (2026-11-20): `HeaderError`, `TransformError` e `PayloadError`, cada um classificado em recuperável ou fatal; o crate classifica e `wyd-net` decide a reação**; o legado tem as três famílias e nenhum mapeamento escrito Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |

### 2.4 Qualidade e entrega

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPRT-12` | Quem garante ordem entre mensagens de uma conexão? | se o codec permite paralelismo, o consumidor precisa saber; ordem é requisito de combate e inventário | arquitetura + frente `wyd-net` | `W6` | fechada | **decisão do dono (2026-11-20): a ordem por conexão é garantida por `wyd-net`; o codec é stateless e agnóstico de ordem**; a `1.0.0` ainda não tem código de rede Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-14` | Qual é o escopo de fuzz/propriedade e quanto tempo roda no CI? | decode de bytes hostis é a superfície mais exposta do servidor; fuzz sem orçamento não roda | arquitetura + frente `wyd-ops` | `W5` | fechada | **decisão do dono (2026-11-20): propriedade curta em cada PR + fuzz com teto de tempo em execução agendada**; o agendamento é `P` até o CI deixar de ser stub (`.github/workflows/` não tem job de fuzz; `planejamento-1.0.0/wyd-1.0.0.md` §1.4) |
| `LPRT-15` | Como medir 85% num crate com tabela gerada? | tabela gerada infla cobertura e esconde caminho sem teste; a meta é por capacidade, não por linha | arquitetura + QA | `W5` | fechada | **decisão do dono (2026-11-20): código gerado fica fora da métrica de linha e cada variante é provada em matriz de capacidade**; `AGENTS.md` §8 fixa 85% sem regra de exclusão, que passa a ser esta | Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10.
| `LPRT-16` | O gerador de `Opcode`/`KEY_TABLE` é próprio e reprodutível? | tabela congelada que ninguém sabe regenerar vira arqueologia; gerador do legado está arquivado | arquitetura | `W3` | fechada | **decisão do dono (2026-11-20): gerador próprio no repositório, entrada versionada, saída determinística e diff verificado no CI**; o legado cita `tools/protocol/gen-protocol.mjs` congelado (`opcode_table.rs:4`, 2026-11-19) Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-17` | Como o projeto declara que a ofuscação **não** é criptografia? | leitor pode assumir sigilo; precisa estar escrito, com o que a transformação impede e o que não impede | arquitetura + segurança | `W7` | fechada | **decisão do dono (2026-11-20): declarar ofuscação (XOR por tabela), não criptografia, sem confidencialidade nem integridade, e checksum não é MAC**; `deprecated/crates/wyd-protocol/src/codec/keyword.rs`, 2026-11-19 Refletida em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §10. |
| `LPRT-20` | O catálogo de opcodes publicado é gerado ou escrito? | catálogo mantido à mão diverge do código na primeira mudança, e o inventário do legado (`lacunas.md` §0.4) não dimensiona o catálogo novo | arquitetura + frente `wyd-docs` | `W8` | fechada | **decisão do dono (2026-11-20): catálogo escrito à mão, citando os valores da tabela gerada**; mantém a `LPDOC-10` (não publicar catálogo gerado na `1.0.0`) e fica destravada por `LPRT-01` (cobertura integral do inventário do legado, sem teto em 198); o total publicado é medido do gerador, na versão Refletida em `planejamento-1.0.0/wyd-protocol/docs/convencoes.md` §"Catálogo gerado × semântica escrita". |

---

## 3. O que depende de cada lacuna
**Sem arquivo no repositório:** `docs/wire/enquadramento.md`, `docs/wire/header.md`, `docs/wire/keyword-e-checksum.md`, `docs/wire/tabela-de-opcodes.md`, `docs/arquitetura/seguranca-do-fio.md`, `docs/arquitetura/versionamento-e-compatibilidade.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Documento | Bloqueado por |
|---|---|
| `docs/wire/header.md`, `docs/wire/enquadramento.md`, `docs/wire/keyword-e-checksum.md` | `LPRT-03` (origem da `KeyWord`) + prova de byte por golden/captura |
| `docs/wire/tabela-de-opcodes.md`, `docs/wire/opcodes/*` | alvo fixado (`LPRT-09`); falta a prova de byte |
| `docs/payloads/*` | `LPRT-06` (encoding/terminador) + golden por família |
| `docs/goldens/*`, `docs/tests/*` | capturas reais: o processo de `LPRT-10` está aprovado e o procedimento das duas lacunas de captura e da leitura de build está escrito em `docs/goldens/como-capturar.md`; nenhuma captura existe |
| `docs/specs/requisitos-*.md`, `docs/specs/fluxos/*` | nada em decisão; dependem das ondas 2–5 |
| `docs/arquitetura/seguranca-do-fio.md` | destravado por `LPRT-17` |
| `docs/arquitetura/versionamento-e-compatibilidade.md` | alvo fixado (`LPRT-09`); falta a comparação entre builds |
| `docs/roadmap/WYDPRT-nnn.md` | destravado por `LPRT-01` |

**Não bloqueado:** `docs/index.md`, `docs/convencoes.md` e toda a onda 1 (método: guias, planos,
prompts, skills). Na onda 2, `header.md` e `keyword-e-checksum.md` já têm forma decidida (`LPRT-02`,
`LPRT-05`, `LPRT-07`), faltando só a origem da `KeyWord` e a prova de byte; na onda 3, o escopo do
catálogo está decidido (`LPRT-01`) e o build-alvo também (`LPRT-09`, 2026-11-23); falta a prova de byte.

---

## 4. Como uma lacuna fecha

1. O dono decide e a decisão é escrita no documento dono (ou em `ADR-PRT-nnnn`, se muda fronteira,
   forma do crate ou operação).
2. A linha deste registro ganha `fechada`, **data** e a evidência (`arquivo:linha` ou comando).
3. O plano diretor (§10) e a página dependente são atualizados na mesma entrega.
4. Divergência entre a decisão e o documento dono é defeito a corrigir no documento, não
   interpretação.

Lacuna que muda de escopo não é apagada: ganha estado `retirada`, com motivo e data, e o ID não é
reaproveitado.
---

## 5. Bloco de decisão das lacunas bloqueadas

As linhas `LPRT-03` e `LPRT-06` em §2 não mudam de estado até a prova existir; a política de cada uma
já foi aprovada pelo dono, e o que segue é a proposta da arquitetura — **aceita pelo dono em 2026-11-25, opção A em ambas** — para o que falta (`AGENTS.md` §5,
item 8).

**LPRT-03 — origem da `KeyWord`.**
- **Dono da decisão:** pesquisa + arquitetura; o processo de prova é
  `planejamento-1.0.0/wyd-protocol/docs/goldens/como-capturar.md`.
- **Pergunta fechada:** no 1.0.0 a `KeyWord` é gerada pelo servidor por sessão a partir da `KEY_TABLE`,
  enviada pelo cliente no handshake, ou constante do build?
- **Opções:** (A) **servidor gera por sessão e envia** — o crate continua recebendo a palavra do
  chamador e uma sessão não reusa a da anterior; custo: o handshake precisa carregar a palavra e o
  golden prova duas sessões com palavras distintas; (B) **cliente envia** — o servidor aceita a palavra
  do cliente; custo: um cliente modificado escolhe a ofuscação e a prova depende do binário; (C)
  **constante do build** — custo: nenhuma rotação e a ofuscação vira valor fixo publicado.
- **Recomendação:** A, porque preserva a política já aprovada (o crate não assume origem) e é a única
  opção em que a prova de byte não depende de confiar no cliente.
- **Aceite do dono (2026-11-25):** opção **A** aprovada — a `KeyWord` é gerada pelo servidor por sessão e enviada no handshake; o crate continua recebendo a palavra do chamador e não assume origem.
- **Artefato que falta (o que fecha):** duas capturas de handshake com palavras distintas, segunda leitura e golden `G-LOGIN`. **Tentativa registrada em 2026-11-25:** não executável neste ambiente — o procedimento exige `ECAP` + `PEER-A1` e nenhum dos dois existe no repositório (medido: 0 `Cargo.toml` e 0 `src` em `crates/wyd-server`, `crates/wyd-protocol` e `crates/wyd-net`; `wyd-client/tools/` ausente); sem endpoint, o cliente 7662 não tem para onde conectar e não há tráfego a gravar. Quem destrava: frente `wyd-server`/`wyd-client` implementa `ECAP`+CLI; depois operador nomeado e QA fazem a segunda leitura.
  no processo de `goldens/como-capturar.md`; a linha `LPRT-03` passa a citar o golden.

**LPRT-06 — encoding e terminador de string.**
- **Dono da decisão:** pesquisa + frente `wyd-core`; o processo de prova é
  `planejamento-1.0.0/wyd-protocol/docs/goldens/como-capturar.md`.
- **Pergunta fechada:** nome de personagem e mensagem de chat viajam como bytes de tamanho fixo em
  `CP1252` com terminador `\0`, ou em `UTF-8` sem terminador?
- **Opções:** (A) **`CP1252` fixo com `\0`** — o codec não transcodifica e o teste de ida e volta usa
  nome acentuado; custo: caractere fora da tabela é recusado; (B) **`UTF-8` sem terminador** — custo:
  nome de 16 bytes pode cortar caractere multibyte e o cliente 7662 não é `UTF-8` (`H` com `LPRT-06`).
- **Recomendação:** A, porque o build-alvo é o cliente Windows 7662 e o contrato aprovado já diz que o
  codec não transcodifica.
- **Aceite do dono (2026-11-25):** opção **A** aprovada — `CP1252` fixo com terminador `\0`, sem transcodificação no codec.
- **Artefato que falta (o que fecha):** capturas B/C com nome acentuado, golden de ida e volta e teste que recusa caractere fora da tabela. **Tentativa registrada em 2026-11-25:** mesmo bloqueio de `LPRT-03` — sem `ECAP` existente e sem personagem sintético em servidor, não há texto atravessando o fio; nenhuma medição foi inventada nem reaproveitada.; a linha `LPRT-06` passa a citar o golden.
