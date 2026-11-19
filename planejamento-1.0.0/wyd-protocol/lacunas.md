# `wyd-protocol` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-19).** Este é o **único** lugar onde uma lacuna do `wyd-protocol`
> 1.0.0 tem dono, impacto, marco e estado. Documento que divergir deste registro está errado até
> prova em contrário.
>
> **O que este arquivo cobre:** as **20 lacunas** `LPRT-01`–`LPRT-20` do planejamento do protocolo.
> Nenhuma foi fechada: o sistema entrou em planejamento hoje, e o cliente que impõe o layout é
> binário externo.
>
> **O que este arquivo não é:** não decide. Cada linha diz **quem** decide e **o que** precisa ser
> decidido; a decisão entra no documento dono (ou em ADR, quando estrutural) e volta aqui como linha
> fechada, com data e evidência.

---

## 0. Como ler

### 0.1 Estados

| Estado | Significado |
|---|---|
| **aberta** | sem decisão; a página que depende dela não pode ser `aceita` |
| **proposta** | existe recomendação técnica escrita; falta o \"sim\" do dono |
| **fechada** | decidida e refletida no arquivo citado em \"evidência\" |
| **externa** | decidida, mas a execução depende de frente fora do sistema |

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

---

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Escopo e contrato | 4 | 3 (`LPRT-01`, `LPRT-18`, `LPRT-19`) | 1 (`LPRT-09`) | 0 |
| Fio e codec | 6 | 4 (`LPRT-02`, `LPRT-03`, `LPRT-05`, `LPRT-07`) | 2 (`LPRT-06`, `LPRT-08`) | 0 |
| Payload e prova | 4 | 1 (`LPRT-04`) | 3 (`LPRT-10`, `LPRT-11`, `LPRT-13`) | 0 |
| Qualidade e entrega | 6 | 2 (`LPRT-14`, `LPRT-20`) | 4 (`LPRT-12`, `LPRT-15`, `LPRT-16`, `LPRT-17`) | 0 |
| **Total** | **20** | **10** | **10** | **0** |

\"Proposta\" não é decisão: descreve o caminho recomendado para o dono aceitar ou recusar. Nenhuma
página de `wire/`, `payloads/` ou `goldens/` pode ser `aceita` antes do fechamento da lacuna que a
bloqueia (§2).

---

## 2. Lacunas

### 2.1 Escopo e contrato

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPRT-01` | Quais dos 198 opcodes do legado entram na `1.0.0`? | define o tamanho do catálogo, das páginas de payload, dos goldens e da suíte; sem isso o roadmap não fatia | produto + arquitetura | `A` | aberta | ausência de lista aprovada; legado medido em 198 variantes (`deprecated/crates/wyd-protocol/src/wire/generated/opcode_table.rs`, 2026-11-19) |
| `LPRT-18` | O crate entrega payload tipado ou bytes crus a `wyd-core`? | muda a API e a fronteira: offset saindo do crate reabre a possibilidade de cada sistema interpretar o fio por conta própria | arquitetura + frente `wyd-core` | `W6` | aberta | nenhum contrato escrito com `wyd-core`; `wyd-core/docs/` não existe |
| `LPRT-19` | Os fluxos `game ↔ db` (`FLAG_GAME2DB`, `FLAG_DB2GAME`, `FLAG_DB2NP`, `FLAG_NP2DB`) entram no escopo? | `wyd-db` é acessado pelo servidor, não por CPSock na `1.0.0`; incluir fluxo sem consumidor cria contrato morto | produto + arquitetura | `A` | aberta | as 7 flags existem no legado; nenhum consumidor medido na arquitetura nova (`planejamento-1.0.0/wyd-1.0.0.md` §6) |
| `LPRT-09` | Qual build do cliente é o alvo e o que muda em relação ao build anterior? | o legado declara `APP_VERSION = 7640` e o repositório fala em cliente 7662; sem versão-alvo, offset e mensagem ficam ambíguos | frente `wyd-client` + pesquisa | `W3` | proposta | `deprecated/crates/wyd-protocol/src/wire/header.rs:31` (7640) × `README.md` do legado (7662) — divergência a resolver com captura |

### 2.2 Fio e codec

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPRT-02` | O servidor aceita checksum divergente? | o legado tolerava; tolerar em produção esconde corrupção e abre margem para pacote forjado; recusar pode quebrar cliente real | arquitetura + frente `wyd-net` | `W2` | aberta | legado tem as duas políticas (`src/codec/checksum.rs`, 2026-11-19), sem decisão registrada para a `1.0.0` |
| `LPRT-03` | De onde vem a semente (`KeyWord`) da ofuscação? | define se `KeyWord` é escolhido pelo servidor, herdado do cliente ou constante; muda o handshake e o teste | pesquisa + arquitetura | `W2` | aberta | legado usa `KEY_TABLE` com 16 pares (`src/codec/generated/keyword_table.rs`, 2026-11-19); regra de escolha não medida no cliente |
| `LPRT-05` | O layout é lido por `repr(C)` ou por leitura explícita offset a offset? | `repr(C)` depende de compilador e de padding; leitura explícita é mais lenta de escrever e mais estável. Muda `unsafe`, portabilidade e prova | arquitetura | `W2` | aberta | legado usa `#[repr(C)]` + leitura explícita (`src/wire/header.rs:74`, 2026-11-19), sem ADR |
| `LPRT-07` | O que o codec faz com frame exatamente em 8192 e acima disso? | limite é defesa contra amplificação; a reação correta depende de `wyd-net` e do cliente | arquitetura + frente `wyd-net` | `W2` | aberta | `MAX_MESSAGE_SIZE = 8192` no legado (`src/wire/header.rs:22`), sem prova de comportamento do cliente |
| `LPRT-06` | Qual encoding, tamanho e terminador de string o cliente usa? | nome de personagem e mensagem de chat atravessam o fio; errar encoding corrompe dado e cria bug intermitente | pesquisa + frente `wyd-core` | `W4` | proposta | nenhuma captura de texto acentuado; legado não declara encoding na API pública |
| `LPRT-08` | O que fazer com `Type` fora do catálogo? | descartar em silêncio esconde incompatibilidade; derrubar conexão transforma mensagem nova em queda | frente `wyd-net` + arquitetura | `W3` | proposta | legado não tem política única; decisão pertence à conexão, não ao codec |

### 2.3 Payload e prova

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPRT-04` | Qual é a fonte de verdade do layout de payload: cliente, captura ou legado? | é a lacuna que decide se o projeto copia o legado ou reverifica o fio; sem ela nenhuma página de `payloads/` é `aceita` | arquitetura + pesquisa | `W4` | aberta | legado tem payloads por família, sem proveniência por campo (`src/payload/*`, 2026-11-19) |
| `LPRT-10` | Como um golden é capturado, anonimizado e aprovado? | golden é a unidade de prova do sistema; sem processo, prova vira opinião e dado de jogador pode vazar | frente `wyd-client` + QA + arquitetura | `W5` | proposta | legado tem 10 testes de golden sem registro de origem (`tests/golden/pacotes.rs`, 2026-11-19) |
| `LPRT-11` | Qual é a semântica de `FLAG_NEW` e das flags de direção no roteamento? | valor de fio é composto por flag; interpretar errado faz mensagem cair no fluxo errado | arquitetura + frente `wyd-net` | `W3` | proposta | 7 flags no legado (`opcode_table.rs:9-15`, 2026-11-19), sem documento de semântica |
| `LPRT-13` | Qual é a taxonomia final de erro e o mapeamento para reação na conexão? | erro do codec precisa de decisão única; duplicar em `wyd-net` cria duas políticas | arquitetura + frente `wyd-net` | `W6` | proposta | legado tem `HeaderError`, `TransformError` e erro de payload; mapeamento não escrito |

### 2.4 Qualidade e entrega

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPRT-12` | Quem garante ordem entre mensagens de uma conexão? | se o codec permite paralelismo, o consumidor precisa saber; ordem é requisito de combate e inventário | arquitetura + frente `wyd-net` | `W6` | proposta | nenhuma decisão escrita; a `1.0.0` não tem código de rede |
| `LPRT-14` | Qual é o escopo de fuzz/propriedade e quanto tempo roda no CI? | decode de bytes hostis é a superfície mais exposta do servidor; fuzz sem orçamento não roda | arquitetura + frente `wyd-ops` | `W5` | aberta | `.github/workflows/` não tem job de fuzz; CI atual é stub (`planejamento-1.0.0/wyd-1.0.0.md` §1.4) |
| `LPRT-15` | Como medir 85% num crate com tabela gerada? | tabela gerada infla cobertura e esconde caminho sem teste; a meta é por capacidade, não por linha | arquitetura + QA | `W5` | proposta | `AGENTS.md` §8 fixa 85% sem regra de exclusão de código gerado |
| `LPRT-16` | O gerador de `Opcode`/`KEY_TABLE` é próprio e reprodutível? | tabela congelada que ninguém sabe regenerar vira arqueologia; gerador do legado está arquivado | arquitetura | `W3` | proposta | legado cita `tools/protocol/gen-protocol.mjs` congelado (`opcode_table.rs:4`, 2026-11-19) |
| `LPRT-17` | Como o projeto declara que a ofuscação **não** é criptografia? | leitor pode assumir sigilo; precisa estar escrito, com o que a transformação impede e o que não impede | arquitetura + segurança | `W7` | proposta | transformação do legado é XOR por tabela (`src/codec/keyword.rs`, 2026-11-19) |
| `LPRT-20` | O catálogo de opcodes publicado é gerado ou escrito? | 198 linhas mantidas à mão divergem do código na primeira mudança | arquitetura + frente `wyd-docs` | `W8` | aberta | nenhum gerador de página no `wyd-docs`; site nasce em 2026-11-19 sem esse pipeline |

---

## 3. O que depende de cada lacuna

| Documento | Bloqueado por |
|---|---|
| `docs/wire/header.md`, `docs/wire/enquadramento.md`, `docs/wire/keyword-e-checksum.md` | `LPRT-02`, `LPRT-03`, `LPRT-05`, `LPRT-07` |
| `docs/wire/tabela-de-opcodes.md`, `docs/wire/opcodes/*` | `LPRT-01`, `LPRT-09`, `LPRT-11`, `LPRT-16` |
| `docs/payloads/*` | `LPRT-04`, `LPRT-06`, `LPRT-10` |
| `docs/goldens/*`, `docs/tests/*` | `LPRT-10`, `LPRT-15` |
| `docs/specs/requisitos-*.md`, `docs/specs/fluxos/*` | `LPRT-01`, `LPRT-13`, `LPRT-18`, `LPRT-19` |
| `docs/arquitetura/seguranca-do-fio.md` | `LPRT-17` |
| `docs/arquitetura/versionamento-e-compatibilidade.md` | `LPRT-09` |
| `docs/roadmap/WYDPRT-nnn.md` | `LPRT-01` |

**Não bloqueado:** `docs/index.md`, `docs/convencoes.md` e toda a onda 1 (método: guias, planos,
prompts, skills). Essas páginas descrevem procedimento e podem ser escritas agora.

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
