# wyd-protocol 1.0.0 — plano diretor da documentação oficial

> **Estado: APROVADO em 2026-11-22** (rascunho de 2026-11-19; decisões de escopo em 2026-11-20). Este documento define **o que** a documentação
> oficial do `wyd-protocol` contém, **em que arquivo** cada assunto mora, **quem** responde por ele e
> **em que ordem** o material é produzido. A árvore de §4 é o **teto**, não o estado: página escrita
> se reconhece pelo cabeçalho de §6.1 e pelo estado; o que está pronto é medido em
> `planos/metricas.md`, que ainda não existe.
>
> Segundo sistema piloto, escolhido pelo time em **2026-11-19** por dependência: `wyd-net` e
> `wyd-core` não têm contrato sem o fio. O molde é o de `planejamento-1.0.0/wyd-db/` — estrutura,
> não texto.
>
> **Organização (D-04 = A): fechada em 2026-11-25.** A auditoria (`wyd-ops/auditoria-de-organizacao.md`) listava para este sistema: árvore de `src/`/`tests/` ausente, `RF-PRT`/`RNF-PRT` não numerados, plano de teste próprio ausente, roadmap parcial. Fechado em 2026-11-25 com `src/.gitkeep` + `tests/.gitkeep`, `docs/arquitetura/componentes.md`, `docs/planos/plano-de-testes.md` e `docs/specs/requisitos.md` (6 `RF-PRT` + 6 `RNF-PRT`). Os 2 lacunas de captura (`LPRT-03`, `LPRT-06`) permanecem abertas e não bloqueiam o checklist de organização.

Baseline de referência, medido em **2026-11-19** em `deprecated/crates/wyd-protocol/`: **26
> arquivos `.rs`**, **198 variantes de `Opcode`**, **7 flags de direção**, **`HEADER_SIZE = 12`**,
> **`MAX_MESSAGE_SIZE = 8192`**, **14 testes** (10 em `tests/golden/pacotes.rs`, 4 em
> `tests/integracao/roundtrip.rs`). Esses números descrevem o que a reescrita **substitui**.
> **198 é o baseline do legado, não o total alvo do catálogo novo:** vale como piso de cobertura, e a
> contagem publicada vem do gerador novo, com proveniência (`LPRT-01`, `LPRT-16`, `LPRT-20`). A
> cobertura integral é entregue em versões menores, da `0.1.0` à `1.0.0` (§9.1); nenhum número do
> legado entra nesta árvore como regra sem reverificação (`LPRT-04`).

---

## 0. Como ler este documento

| Se você quer… | Vá para |
|---|---|
| entender o que é o sistema e o que ele não é | §1 |
| saber para quem escrevemos e que pergunta cada público faz | §2 |
| conhecer as regras editoriais específicas do fio | §3 |
| ver a árvore completa, arquivo por arquivo | §4 |
| entender o papel de cada pasta | §5 |
| ver o conteúdo obrigatório de cada página | §6 |
| conhecer os identificadores (`RF-PRT`, `LPRT`, `WYDPRT`…) | §7 |
| saber quando uma página está pronta | §8 |
| ver a ordem de produção | §9 |
| saber o que depende de decisão do time | §10 |

---

## 1. Objetivo, escopo e limites

### 1.1 Objetivo

Produzir a documentação **final** do `wyd-protocol` — o crate que transforma bytes do CPSock em
tipos e tipos em bytes — de modo que:

1. **o layout do fio** (cabeçalho, ofuscação, checksum, enquadramento, catálogo de `Type`) esteja
   descrito de forma que um desenvolvedor implemente o codec sem abrir o cliente;
2. **cada payload** tenha forma, faixa de valor e semântica declaradas, com a origem da afirmação;
3. **cada opcode do catálogo, na versão em que entra,** tenha uma linha com direção, família,
   payload e golden;
4. **o erro** seja tipado e mapeado para a política de conexão de `wyd-net`, sem decisão duplicada;
5. a documentação seja **publicável** no site do `wyd-docs` sem reescrita.

### 1.2 Escopo da `1.0.0`

| Dentro | Fora |
|---|---|
| codec CPSock: `HEADER` de 12 bytes, ofuscação `pKeyWord`, checksum, enquadramento por `Size` | transporte, socket, sessão, timeout e rate-limit — dono: `wyd-net` |
| catálogo de `Type` (`Opcode`), flags de direção e `FLAG_NEW` | roteamento de mensagem e despacho por sistema — dono: `wyd-net`/`wyd-server` |
| os fluxos `game ↔ db`, `db ↔ np` e `np` (`FLAG_GAME2DB`, `FLAG_DB2GAME`, `FLAG_DB2NP`, `FLAG_NP2DB`), decididos em 2026-11-20 (`LPRT-19`) | consumo desses fluxos pelos sistemas de destino — dono: `wyd-db`/`wyd-server` |
| payload de cada opcode do escopo: campos, offsets, tipos, faixas e nulidade | regra de jogo (dano, EXP, cooldown, drop) — dono: `wyd-core`/`wyd-sim` |
| goldens de bytes e testes de propriedade do codec | captura de tráfego em produção e ferramenta de captura — dono: `wyd-ops`/`wyd-client` |
| tabelas geradas (`Opcode`, `KEY_TABLE`) e sua reprodutibilidade | geração de `strdef.bin`, assets e patch do cliente — dono: `wyd-client` |
| versão de protocolo declarada e compatibilidade de build | negociação de versão na sessão — dono: `wyd-net` |

**Fronteira de honestidade:** o layout do fio é imposto pelo **cliente 7662**, que é binário externo
e proprietário. Afirmação sobre byte que não foi capturado, lido no cliente ou provado por golden é
`H`, com o ID da lacuna que a fecha. O legado `deprecated/crates/wyd-protocol/` é **referência**
(nível 4 de `AGENTS.md` §3), nunca fonte: ele inspira a pergunta, não responde por ela.

### 1.3 O que esta documentação **não** é

- Não é cópia do `Basedef.h` nem tradução do crate antigo: o catálogo novo é escrito uma vez, com
  proveniência por linha, e não é dimensionado pelo inventário do legado — 198 é piso de cobertura,
  não total (§9.1).
- Não é manual de jogo nem descrição de balanceamento.
- Não é registro de captura: o golden é artefato (`tests/`), a página descreve o que ele prova.
- Não é especificação de rede: o que acontece com pacote inválido na conexão é de `wyd-net`.

### 1.4 Contratos com os outros sistemas

| Sistema | O que recebe do `wyd-protocol` | O que fornece | Dono do contrato |
|---|---|---|---|
| `wyd-net` | `decode(buf) -> Frame`, `encode(frame) -> bytes`, erro tipado, política de checksum | buffer completo por conexão, política de descarte e limite de conexão | `wyd-protocol` (forma), `wyd-net` (reação) |
| `wyd-core` | payload tipado por família, já validado em faixa | semântica do campo e efeito no tick | `wyd-core` |
| `wyd-server` | frame decodificado e roteável por `Type` | composição do processo e ordem de despacho | `wyd-net` |
| `wyd-client` | nada (o crate não fala com o cliente) | captura de referência e `strdef.bin` para nome de mensagem | `wyd-ops` (processo de captura) |
| `wyd-bots` | mesmo codec do servidor, em modo cliente | cenário que exercita o fio | `wyd-protocol` |
| `wyd-docs` | páginas desta árvore e tabelas geradas | site, navegação e build estrito | `wyd-docs` |

Regra: payloads atravessam a fronteira **tipados e validados**; bytes crus não saem do crate. Se um
consumidor precisa de offset, o contrato está incompleto.

---

## 2. Público e perguntas
**Sem arquivo no repositório:** `arquitetura/camadas-de-codec.md`, `arquitetura/seguranca-do-fio.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Público | Pergunta que ele faz | Onde a resposta mora |
|---|---|---|
| **Desenvolvedor de `wyd-net`** | \"como enquadro, decodifico e o que faço com erro?\" | `guia-desenvolvedor/` + `arquitetura/camadas-de-codec.md` |
| **Desenvolvedor de `wyd-core`** | \"que campos este payload traz e o que eles significam?\" | `payloads/` + `specs/regras-de-negocio.md` |
| **Desenvolvedor novo** | \"como adiciono um opcode sem quebrar o fio?\" | `guia-desenvolvedor/como-adicionar-opcode.md` |
| **Pesquisador do cliente (RE)** | \"onde está a prova deste offset e o que falta capturar?\" | `goldens/` + `lacunas.md` do sistema |
| **QA** | \"o que precisa ser provado e com que bytes?\" | `guia-teste/` + `goldens/catalogo.md` |
| **Arquiteto** | \"por que o codec é assim e onde estão as fronteiras?\" | `arquitetura/` + `arquitetura/adr/` |
| **Segurança** | \"a ofuscação protege o quê e o que entra em log?\" | `arquitetura/seguranca-do-fio.md` |
| **Agente de IA** | \"qual o procedimento exato para esta tarefa?\" | `prompts/` + `skills/` |

**Medido em 2026-11-25:** das páginas citadas nesta tabela, existem `goldens/como-capturar.md` e
`goldens/catalogo.md`; `arquitetura/camadas-de-codec.md`, `arquitetura/seguranca-do-fio.md`,
`guia-desenvolvedor/como-adicionar-opcode.md` e as de `payloads/`, `guia-teste/` **não foram escritas**
— são teto de §4, não página publicada.


Nenhuma página existe sem um público e uma pergunta desta tabela.

---

## 3. Regras editoriais

Valem as regras de `AGENTS.md` §5 e as de leitura de `wyd-db/docs/convencoes.md`, com quatro
acréscimos que só fazem sentido no fio:

1. **Byte sem prova é hipótese.** Offset, tipo, tamanho e valor de constante exigem `V` com fonte
   (`golden` com ID, captura com data, leitura do cliente com endereço) ou `H` com `LPRT-nn`. Não
   existe \"provavelmente é `u16`\".
2. **Golden é a unidade de prova do fio.** Toda afirmação de layout aponta para um golden de
   `tests/golden/` (`G-<familia>-<nn>`) ou para o registro de captura que o originou. Golden sem
   origem declarada não vale como prova.
3. **Uma direção por afirmação.** Fluxo `client → game`, `game → client`, `game ↔ db` e `np` não se
   misturam na mesma tabela sem coluna de direção. Opcode que existe em duas direções tem duas
   linhas.
4. **O catálogo é gerado, a semântica é escrita.** `Opcode`, flags e `KEY_TABLE` vêm de tabela
   versionada e gerador; nome de mensagem, família e observação são texto. Página manual que repete
   valor de opcode é dívida declarada e reprova (§8).
5. **Sem conteúdo do legado por paráfrase.** Quando a única origem é
   `deprecated/crates/wyd-protocol/`, a página diz isso no campo `Base:` e marca `H` — o legado não
   empresta verificação.

---

## 4. Árvore completa

Legenda: `▲` a escrever · `◆` gerado por script · `■` artefato de teste (binário/JSON).

```text
planejamento-1.0.0/wyd-protocol/
├── wyd-protocol-1.0.0.md                     ▲ este plano diretor
├── lacunas.md                                ▲ registro único de lacuna e dono
├── docs/
│   ├── index.md                              ▲ porta de entrada
│   ├── convencoes.md                         ▲ como ler página de fio
│   ├── glossario.md                          ▲ vocabulário do protocolo
│   ├── arquitetura/
│   │   ├── index.md                          ▲
│   │   ├── visao-geral.md                    ▲
│   │   ├── contexto-c4.md                    ▲
│   │   ├── componentes-c4.md                 ▲
│   │   ├── camadas-de-codec.md               ▲
│   │   ├── fronteira-protocolo-rede.md       ▲
│   │   ├── fluxo-de-um-pacote.md             ▲
│   │   ├── versionamento-e-compatibilidade.md ▲
│   │   ├── erros-e-politicas.md              ▲
│   │   ├── seguranca-do-fio.md               ▲
│   │   └── adr/
│   │       └── index.md                      ▲
│   ├── specs/
│   │   ├── index.md                          ▲
│   │   ├── visao-e-escolpo.md                ▲
│   │   ├── requisitos-funcionais.md          ▲
│   │   ├── regras-de-negocio.md              ▲
│   │   ├── requisitos-nao-funcionais.md      ▲
│   │   ├── casos-de-uso.md                   ▲
│   │   ├── restricoes-e-premissas.md         ▲
│   │   ├── rastreabilidade.md                ▲
│   │   ├── fluxos/
│   │   │   ├── login.md                      ▲
│   │   │   ├── selecao-de-personagem.md      ▲
│   │   │   ├── entrada-na-cena.md            ▲
│   │   │   ├── movimento.md                  ▲
│   │   │   ├── combate.md                    ▲
│   │   │   ├── inventario-e-item.md          ▲
│   │   │   ├── comercio-e-npc.md             ▲
│   │   │   └── chat-e-ui.md                  ▲
│   │   └── estados/
│   │       ├── frame.md                      ▲
│   │       ├── conexao.md                    ▲
│   │       └── versao-de-protocolo.md        ▲
│   ├── wire/
│   │   ├── index.md                          ▲
│   │   ├── header.md                         ▲
│   │   ├── enquadramento.md                  ▲
│   │   ├── keyword-e-checksum.md             ▲
│   │   ├── tabela-de-opcodes.md              ▲
│   │   ├── flags-de-direcao.md               ▲
│   │   ├── limites-e-faixas.md               ▲
│   │   └── opcodes/
│   │       ├── index.md                      ▲
│   │       ├── autenticacao.md               ▲
│   │       ├── personagem.md                 ▲
│   │       ├── cena-e-mundo.md               ▲
│   │       ├── movimento.md                  ▲
│   │       ├── combate.md                    ▲
│   │       ├── inventario.md                 ▲
│   │       ├── comercio.md                   ▲
│   │       ├── quest-e-eventos.md            ▲
│   │       └── ui-e-chat.md                  ▲
│   ├── payloads/
│   │   ├── index.md                          ▲
│   │   ├── como-ler.md                       ▲
│   │   ├── tipos-e-encoding.md               ▲
│   │   ├── autenticacao.md                   ▲
│   │   ├── personagem.md                     ▲
│   │   ├── cena.md                           ▲
│   │   ├── movimento.md                      ▲
│   │   ├── combate.md                        ▲
│   │   ├── inventario.md                     ▲
│   │   ├── comercio.md                       ▲
│   │   ├── quest.md                          ▲
│   │   └── ui.md                             ▲
│   ├── goldens/
│   │   ├── index.md                          ▲
│   │   ├── como-capturar.md                  ▲
│   │   ├── catalogo.md                       ▲
│   │   └── como-adicionar.md                 ▲
│   ├── planos/
│   │   ├── index.md                          ▲
│   │   ├── plano-de-documentacao.md          ▲
│   │   ├── plano-de-testes.md                ▲
│   │   ├── metricas.md                       ▲
│   │   └── riscos.md                         ▲
│   ├── roadmap/
│   │   ├── index.md                          ▲
│   │   ├── fases.md                          ▲
│   │   └── WYDPRT-001..WYDPRT-032.md         ▲
│   ├── guia-desenvolvedor/
│   │   ├── index.md                          ▲
│   │   ├── setup-ambiente.md                 ▲
│   │   ├── mapa-do-crate.md                  ▲
│   │   ├── api-e-garantias.md                ▲
│   │   ├── como-adicionar-opcode.md          ▲
│   │   ├── como-adicionar-payload.md         ▲
│   │   ├── como-gerar-tabela.md              ▲
│   │   ├── erros-e-logs.md                   ▲
│   │   └── checklist-de-pr.md                ▲
│   └── guia-teste/
│       ├── index.md                          ▲
│       ├── piramide-e-alvos.md               ▲
│       ├── goldens-e-fixtures.md             ▲
│       ├── propriedades-e-fuzz.md            ▲
│       └── gates-e-anti-skip.md              ▲
├── tests/
│   ├── index.md                              ▲
│   ├── alvos/
│   │   ├── unitario-codec.md                 ▲
│   │   ├── contrato-catalogo.md              ▲
│   │   ├── golden-pacotes.md                 ▲
│   │   ├── propriedade-roundtrip.md          ▲
│   │   └── fuzz-decode.md                    ▲
│   └── golden/
│       └── README.md                         ▲
├── prompts/
│   ├── README.md                             ▲
│   ├── plan/analise-de-opcode.md             ▲
│   ├── execution/escrita-de-payload.md       ▲
│   ├── review/revisao-de-fio.md              ▲
│   └── test/captura-de-golden.md             ▲
└── skills/
    ├── README.md                             ▲
    ├── plan/levantar-opcode/SKILL.md         ▲
    ├── execution/implementar-codec/SKILL.md  ▲
    ├── review/revisar-layout/SKILL.md        ▲
    └── test/provar-golden/SKILL.md           ▲
```

**Totais planejados:** 1 plano diretor + 1 registro de lacunas + **3** páginas de raiz em `docs/` +
**11** em `arquitetura/` + **19** em `specs/` + **8** em `wire/` + **10** em `wire/opcodes/` +
**12** em `payloads/` + **4** em `goldens/` + **5** em `planos/` + **34** em `roadmap/` + **9** em
`guia-desenvolvedor/` + **5** em `guia-teste/` + **7** em `tests/` + **4+1** em `prompts/` +
**4+1** em `skills/`.

**Contagem e fonte:** os **34** de `roadmap/` são 2 páginas de seção + **32 issues** `WYDPRT-nnn`,
fatiadas em §9. As duas páginas de seção são `roadmap/index.md` (porta de entrada, que não conta
issue) e `roadmap/fases.md` (recorte das issues por versão, ligado a §9.1). O total autoral é **131
páginas** em `docs/`, e o número **medido** é de `planos/metricas.md`, escrito depois que as páginas
existirem — nenhum total desta seção é medição.
**Escrito:** `docs/index.md` e `docs/convencoes.md` (2026-11-19), `docs/goldens/como-capturar.md`,
`docs/roadmap/index.md` e `docs/roadmap/fases.md` (2026-11-20); os demais são teto.

**O que dá para escrever sem fechar lacuna:** as páginas de **método** (`guia-desenvolvedor/`,
`guia-teste/`, `planos/`, `prompts/`, `skills/`) descrevem procedimento e não dependem de byte; as de
**conteúdo** (`wire/`, `payloads/`, `goldens/`, `specs/fluxos/`) dependem de **captura**: a forma já
está decidida (`LPRT-01`, `04`, `10`) e o build-alvo também (`LPRT-09`, 2026-11-23); o que falta é a
prova (`LPRT-03`, `LPRT-06` e os goldens por família). Ver §9.

---

## 5. Papel de cada pasta

**Medido em 2026-11-25:** os nomes de página citados em §5.2–§5.5 dizem onde o conteúdo **deve** morar;
das páginas de `arquitetura/`, `wire/`, `payloads/` e `specs/`, só `goldens/como-capturar.md` e
`goldens/catalogo.md` existem — as demais ainda **não foram escritas**.

### 5.1 `docs/` raiz

| Arquivo | Responde | Conteúdo obrigatório |
|---|---|---|
| `index.md` | \"por onde começo?\" | o sistema em 5 linhas; mapa das 8 subpastas com estado; trilha por público (rede, core, RE, QA); 3 links de entrada; 1 diagrama de contexto |
| `convencoes.md` | \"como leio uma página de fio?\" | marcações `V`/`H`/`P` do projeto; **regras próprias**: proveniência de byte, ID de golden, direção obrigatória, catálogo gerado × semântica escrita |
| `glossario.md` | \"o que significa este termo?\" | CPSock, `HEADER`, `Size`, `KeyWord`, `CheckSum`, `Type`, `ID`, `ClientTick`, opcode, flag de direção, `FLAG_NEW`, golden, frame, payload, ofuscação, enquadramento |

### 5.2 `docs/arquitetura/`
**Sem arquivo no repositório:** `fronteira-protocolo-rede.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

- **Pergunta:** \"por que o codec é assim, onde termina o protocolo e começa a rede?\"
- **Fronteira:** desenho e decisão. Regra de campo fica em `payloads/`; reação a pacote inválido
  fica em `fronteira-protocolo-rede.md` (ainda **a escrever**), que enuncia a política **do crate** e
  aponta a decisão de `wyd-net`.
- **Regra:** toda página tem pelo menos um diagrama Mermaid e uma seção \"o que quebra se mudar\".

### 5.3 `docs/specs/`

- **Pergunta:** \"o que o crate precisa fazer, sob que regra e com que qualidade?\"
- **Fronteira:** comportamento exigido, não tabela de campo. Cada requisito tem ID, critério de
  aceitação verificável e destino na matriz de rastreabilidade.
- **Proibido:** requisito sem critério; fluxo sem passo alternativo; regra de faixa sem exemplo
  numérico em hexadecimal.

### 5.4 `docs/wire/`

- **Pergunta:** \"como um byte vira frame e um frame vira mensagem?\"
- **Fronteira:** forma do fio. Valor de enum de jogo não entra: `RemoveMob` descreve a mensagem, não
  a regra de morte.
- **Regra:** `header.md` (a escrever) é dono dos offsets do `HEADER`; `tabela-de-opcodes.md` (a
escrever) é dono do total de opcodes (as duas páginas ainda **não foram escritas**) — medido do gerador na versão, nunca herdado do legado — e da origem da tabela;
  `opcodes/*` só classifica por família e aponta para o payload.

### 5.5 `docs/payloads/`

- **Pergunta:** \"que campos esta mensagem carrega e o que cada um significa?\"
- **Fronteira:** layout e semântica do campo **na fronteira**. Efeito no jogo fica em `wyd-core`.
- **Regra:** uma página por família; toda tabela de campo tem colunas `offset`, `tipo`, `tamanho`,
  `faixa`, `obrigatório`, `proveniência`; campo sem proveniência reprova.

### 5.6 `docs/goldens/`

- **Pergunta:** \"que bytes provam este layout e de onde eles vieram?\"
- **Fronteira:** processo e catálogo de prova. O arquivo de golden vive em `tests/golden/`; a página
  diz o que ele prova, a origem da captura e o que ainda não cobre.
- **Regra:** `catalogo.md` é o dono único do total de goldens e da cobertura por família.

### 5.7 `docs/planos/`

- **Pergunta:** \"o que vamos fazer, em que ordem, como medimos e o que pode dar errado?\"
- **Fronteira:** plano e medida. `metricas.md` é o **dono único dos números** deste sistema;
  nenhuma outra página recalcula contagem.

### 5.8 `docs/roadmap/`

- **Pergunta:** \"qual é a próxima tarefa, o que ela entrega e como sei que terminou?\"
- **Fronteira:** uma página por issue `WYDPRT-nnn`, com DoD verificável. Issue sem DoD executável
  não entra na fila.

### 5.9 `docs/guia-desenvolvedor/` e `docs/guia-teste/`

- **Pergunta:** \"como eu mexo nisto sem quebrar o fio?\" / \"como provo o layout?\"
- **Fronteira:** procedimento e padrão; referência de campo fica em `payloads/`.
- **Regra:** todo comando é copiável e foi executado; todo teste aponta para capacidade e golden.

### 5.10 `tests/`, `prompts/` e `skills/`

- `tests/` descreve alvos e fixtures do crate, um arquivo por alvo do `Cargo.toml`, com o que cada
  um prova. `prompts/` diz **o que pedir**; `skills/` diz **como executar**, com pré-requisito,
  passos, verificação e critério de parada.

---

## 6. Conteúdo obrigatório de cada página

Formato: **pergunta** + **seções obrigatórias**. Seções a mais são permitidas; faltar uma reprova.

**Medido em 2026-11-25:** das páginas especificadas em §6.2–§6.8, existem `docs/goldens/como-capturar.md`
e `docs/goldens/catalogo.md`, além de `docs/index.md`, `docs/convencoes.md`, `docs/roadmap/index.md` e
`docs/roadmap/fases.md`. **As demais não foram escritas** — a tabela de cada seção é o molde da página
futura, não a página.

### 6.1 Cabeçalho e estado (todas as páginas)

Cinco linhas, iguais às do projeto (`wyd-db/docs/convencoes.md`, seção *Cabeçalho de página*):

```text
> Pergunta: "..."
> Público: <papel>
> Dono: equipe do wyd-protocol
> Base: <fonte consultada, com §>
> Revisado em: AAAA-MM-DD
```

Seguida de `**Estado:** proposto (AAAA-MM-DD)`. Página do `docs/` sem cabeçalho não é `aceito`.

### 6.2 `docs/arquitetura/*.md`
**Sem arquivo no repositório:** `camadas-de-codec.md`, `fronteira-protocolo-rede.md`, `fluxo-de-um-pacote.md`, `versionamento-e-compatibilidade.md`, `erros-e-politicas.md`, `seguranca-do-fio.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | \"que página de arquitetura responde à minha dúvida?\" | mapa das 10 páginas por pergunta; ordem de leitura; estado de cada uma |
| `visao-geral.md` | \"o que é o `wyd-protocol` em 10 minutos?\" | propósito; o que ele transforma e o que não transforma; 3 números com fonte e data; diagrama de contexto |
| `contexto-c4.md` | \"quem conversa com o crate?\" | C4 nível 1 e 2; atores (`wyd-net`, `wyd-core`, `wyd-bots`, RE, QA); o que cada um pode exigir; o que o crate não decide |
| `componentes-c4.md` | \"quais são as peças internas?\" | C4 nível 3 com `wire`, `codec`, `payload`, `error`, `generated`; tabela componente → responsabilidade → dependência permitida |
| `camadas-de-codec.md` | \"em que ordem os bytes são tratados?\" | pipeline `bytes → HEADER → enquadramento → deofuscação → checksum → Type → payload`; o que cada etapa pode rejeitar; diagrama; custo de cada etapa |
| `fronteira-protocolo-rede.md` | \"onde termina o protocolo e começa a conexão?\" | regra de ouro (codec não faz I/O); o que `wyd-net` decide e o que o crate devolve; tabela erro → reação sugerida → dono da decisão |
| `fluxo-de-um-pacote.md` | \"o que acontece com um pacote do início ao fim?\" | sequência encode/decode; pontos de rejeição; o que é validado e o que é só transportado; diagrama |
| `versionamento-e-compatibilidade.md` | \"como o protocolo muda sem quebrar cliente?\" | versão declarada; o que é congelado; como entra mensagem nova; o que acontece com cliente antigo; build-alvo decidido (`LPRT-09`, 2026-11-23) |
| `erros-e-politicas.md` | \"como o erro aparece e quem decide o quê?\" | taxonomia (`HeaderError`, `TransformError`, `PayloadError`); política estrita × tolerante; o que nunca causa panic; o que vai para log |
| `seguranca-do-fio.md` | \"a ofuscação protege o quê?\" | o que a transformação é (ofuscação, não criptografia); o que ela impede e o que não impede; limite de tamanho como defesa; o que nunca entra em log; ameaças consideradas |
| `adr/index.md` | \"que decisões estão registradas?\" | tabela `ADR-PRT-nnn | decisão | estado | data | substitui`; **vazia enquanto não houver decisão**, e diz isso |

**ADR:** não existe catálogo prévio. O `index.md` nasce sem linha e a primeira decisão aprovada cria
`ADR-PRT-0001`. Registrar decisão futura é proibido (`AGENTS.md` §5, item 3).

### 6.3 `docs/wire/*.md`
**Sem arquivo no repositório:** `header.md`, `enquadramento.md`, `keyword-e-checksum.md`, `tabela-de-opcodes.md`, `flags-de-direcao.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | \"qual página do fio eu leio?\" | mapa das 7 páginas; ordem (header → enquadramento → transformação → catálogo → limites); estado |
| `header.md` | \"quais são os campos do `HEADER` e em que offset?\" | tabela `offset | campo | tipo | tamanho | direção | proveniência`; exemplo hexadecimal anotado; o que muda entre cliente e servidor |
| `enquadramento.md` | \"como sei onde um pacote termina?\" | regra por `Size`; mínimo e máximo com origem; pacote parcial, pacote colado e lixo no buffer; o que o codec devolve em cada caso; diagrama de estados do buffer |
| `keyword-e-checksum.md` | \"como o payload é ofuscado e verificado?\" | quais bytes ficam em claro; semente e passo da transformação; cálculo do checksum com exemplo numérico de 8 bits; política estrita × tolerante e quem escolhe |
| `tabela-de-opcodes.md` | \"quantos opcodes existem e de onde vem a tabela?\" | total medido do gerador na versão, com data (o 198 do legado é baseline, não total); origem da tabela e do gerador; colunas `valor | nome | direção | família | payload | golden`; regra de valor novo |
| `flags-de-direcao.md` | \"o que os bits altos do `Type` significam?\" | uma seção por flag; como o valor de fio é composto; o que acontece com combinação inválida; quem roteia |
| `limites-e-faixas.md` | \"quais são os tetos do fio?\" | tamanho mínimo/máximo; tamanho de string e de vetor; profundidade e quantidade; o que fazer no limite; tabela com origem de cada teto |
| `opcodes/*.md` | \"quais mensagens existem nesta família?\" | tabela `opcode | direção | payload | golden | estado`; agrupamento por fluxo; o que a família não cobre |

### 6.4 `docs/payloads/*.md`
**Sem arquivo no repositório:** `tipos-e-encoding.md` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

**`index.md`** — mapa das famílias, total de payloads documentados com fonte, estado por família.

**`como-ler.md`** — anatomia de uma página de payload; o que é gerado × escrito; como ler offset e
faixa; o que fazer quando golden e página divergem (**prevalece o golden**; a página é defeito).

**`tipos-e-encoding.md`** — largura e sinal de cada tipo; ordem de bytes; string de tamanho fixo,
terminador e encoding; campo opcional; alinhamento e padding; como o crate lê sem `unsafe`.
A página **ainda não existe**: é teto de §4.

**Uma página por família** (`autenticacao`, `personagem`, `cena`, `movimento`, `combate`,
`inventario`, `comercio`, `quest`, `ui`) com, nesta ordem:

1. **Mensagens da família** e o fluxo a que pertencem.
2. **Tabela de campos por mensagem:** `offset | campo | tipo | tamanho | faixa | obrigatório | proveniência`.
3. **Regras de faixa** e o que o crate rejeita.
4. **Campos opacos** (`H`, com `LPRT-nn`) e o que falta para fechar.
5. **Goldens** que provam a família.

### 6.5 `docs/specs/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | \"o que há de requisito aqui?\" | mapa das 8 páginas; convenção de ID; estado por área; como requisito nasce, muda e sai |
| `visao-e-escolpo.md` | \"o que o crate resolve?\" | problema; escopo dentro/fora; premissa do cliente externo; o que a `1.0.0` entrega |
| `requisitos-funcionais.md` | \"o que o crate precisa fazer?\" | um bloco por `RF-PRT-nnn`: enunciado, entrada, saída, critério verificável, contrato, issue |
| `regras-de-negocio.md` | \"que regra do fio vale sempre?\" | um bloco por `RN-PRT-nnn`: enunciado, exemplo hexadecimal, onde é garantida, efeito da violação |
| `requisitos-nao-funcionais.md` | \"com que qualidade?\" | um bloco por `RNF-PRT-nnn`: métrica, limiar, como medir, reprovação (determinismo, limite, memória, tempo de decode, robustez) |
| `casos-de-uso.md` | \"quem faz o quê, passo a passo?\" | um bloco por `UC-PRT-nnn`: ator, pré-condição, fluxo principal e alternativo, pós-condição, requisitos cobertos |
| `restricoes-e-premissas.md` | \"o que assumimos e o que nos limita?\" | restrição técnica (cliente proprietário); premissa com o que a invalida; decisão pendente com ID e dono |
| `rastreabilidade.md` | \"quem atende este requisito?\" | matriz `RF/RN/RNF/UC → payload → golden → teste → issue`; itens sem cobertura explícitos |

### 6.6 `docs/specs/fluxos/*.md` e `docs/specs/estados/*.md`

Fluxo: ator, gatilho, diagrama de sequência, passo a passo com bytes trocados, falha prevista,
opcodes tocados. Estado: diagrama `stateDiagram-v2`, tabela `estado | evento | guarda | próximo
estado | efeito no buffer/conexão`, estados proibidos.

### 6.7 `docs/goldens/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | \"que prova existe hoje?\" | total de goldens com fonte e data; cobertura por família; o que não tem prova |
| `como-capturar.md` | \"como produzo um golden novo?\" | ferramenta e ambiente; o que capturar; como anonimizar dado de jogador; onde salvar; como registrar origem; o que nunca capturar |
| `catalogo.md` | \"o que cada golden prova?\" | tabela `ID | família | opcode | origem | data | o que prova` — **dono único** do total |
| `como-adicionar.md` | \"como um golden entra na suíte?\" | nome, pasta, teste que consome, revisão exigida, o que reprova |

**Medido em 2026-11-25:** `como-capturar.md` e `catalogo.md` existem (0 goldens aceitos); `index.md` e
`como-adicionar.md` são teto e ainda **não foram escritas**.

### 6.8 `docs/planos/*.md`, `docs/roadmap/*.md`, guias e `tests/`

Seguem o molde de `wyd-db-1.0.0.md` §6.9–§6.17, sem repetir texto: plano de documentação (ondas de
§9), plano de testes (alvos e matriz capacidade × alvo), `metricas.md` (dono dos números),
`riscos.md` (risco, sinal, antídoto, dono), roadmap (issue com DoD executável e fase, e o recorte por
versão de §9.1 em `docs/roadmap/fases.md`), guias (procedimento copiável) e `tests/` (um arquivo por
alvo, com o que prova e que dado usa).

---

## 7. Identificadores

| Prefixo | Uso | Dono |
|---|---|---|
| `RF-PRT-nnn` | requisito funcional | `specs/requisitos-funcionais.md` |
| `RN-PRT-nnn` | regra de negócio do fio | `specs/regras-de-negocio.md` |
| `RNF-PRT-nnn` | requisito não funcional | `specs/requisitos-nao-funcionais.md` |
| `UC-PRT-nnn` | caso de uso | `specs/casos-de-uso.md` |
| `ADR-PRT-nnnn` | decisão estrutural registrada | `arquitetura/adr/` |
| `LPRT-nn` | lacuna aberta | `lacunas.md` |
| `G-<familia>-<nn>` | golden de bytes | `goldens/catalogo.md` |
| `WYDPRT-nnn` | issue do roadmap | `roadmap/` |

Numeração é sequencial e **não se reaproveita**: item retirado mantém o número e ganha estado
`retirado`. Requisito novo entra no fim, mesmo que pertença a área antiga.

---

## 8. Quando uma página está pronta

Uma página é `aceito` quando:

1. tem o cabeçalho de §6.1 e o estado com data;
2. responde à pergunta do cabeçalho e não a outra;
3. toda afirmação de layout tem `V` (golden, captura ou cliente, com data) ou `H` com `LPRT-nn`;
4. toda tabela de campo tem as sete colunas de §5.5;
5. toda afirmação de direção nomeia o fluxo;
6. todo diagrama Mermaid renderiza no build;
7. não há placeholder, link quebrado, segredo nem captura com dado de jogador;
8. foi revisada por alguém que não escreveu.

**Defeito bloqueante:** byte afirmado sem prova; opcode inventado; total sem fonte — inclusive o
inventário do legado apresentado como total do catálogo novo; valor de opcode
digitado à mão onde o catálogo gerado existe; link para fora de `docs/` sem a marca \"planejamento\".

---

## 9. Ordem de produção (ondas)

| Onda | Escopo | Pré-requisito | Páginas |
|---|---|---|---:|
| 1 | método: `guia-desenvolvedor/`, `guia-teste/`, `planos/`, `prompts/`, `skills/` | nenhum | 32 |
| 2 | fio sem payload: `wire/header`, `enquadramento`, `keyword-e-checksum`, `limites-e-faixas`, `flags-de-direcao` | `LPRT-03` e prova de byte por golden (forma decidida em `LPRT-02`, `05`, `07`, `11`) | 5 |
| 3 | catálogo: `wire/tabela-de-opcodes`, `wire/opcodes/*` | build-alvo decidido (`LPRT-09`, 2026-11-23); escopo e gerador em `LPRT-01`, `16` | 10 |
| 4 | payloads: `payloads/*` | `LPRT-06` e golden por família (fonte de verdade em `LPRT-04`) | 12 |
| 5 | goldens: `goldens/*` e `tests/*` | capturas reais: processo aprovado em `LPRT-10`, nenhuma captura existe | 11 |
| 6 | requisitos e fluxos: `specs/*` | ondas 2–5 | 19 |
| 7 | arquitetura e ADRs: `arquitetura/*` | decisões tomadas | 11 |
| 8 | roadmap: 32 issues `WYDPRT-nnn` | fases aprovadas | 34 |

**Gate da Fase 3 do panorama:** o sistema tem fronteira, contrato com `wyd-net` e `wyd-core`,
requisitos com ID, plano de teste e roadmap com DoD; nenhum byte afirmado sem prova.

### 9.1 Fatias de versão — `0.1.0` → `1.0.0`

**Estado: `P` (planejado).** Nenhuma versão foi entregue, nenhum opcode foi gerado e a série abaixo é
ordem de execução, não estado medido. As ondas de §9 ordenam a **produção da documentação**; esta
seção ordena a **entrega do crate e do catálogo**. O dono declarou em 2026-11-20 que a série está
aberta na `0.1.0` e que o catálogo novo é completo: nada do inventário do legado fica de fora e nada é
descartado por não caber numa fatia — fatiar é entregar em partes, não reduzir o escopo (`LPRT-01`).

Regras da série:

1. **Cobertura integral na `1.0.0`:** todo opcode do inventário do legado tem linha no catálogo novo, e
   a `1.0.0` só fecha com todas as famílias provadas por golden.
2. **198 é piso, não total:** o número do legado (`lacunas.md` §0.4) mede o que precisa ser coberto,
   não o tamanho do catálogo. A contagem de cada versão é medida do gerador (`LPRT-16`) e registrada
   em `planos/metricas.md`, com versão e data — nunca herdada do inventário do legado nem transportada
   de uma página para outra.
3. **String antes de personagem, prova antes da família:** `LPRT-06` fecha antes de qualquer família
   cujo payload carregue texto — o nome de personagem já atravessa o fio como string —, e cada versão
   entrega família com payload tipado e golden aprovado pelo processo de `LPRT-10`; opcode sem golden
   migra para a versão seguinte em vez de entrar "para provar depois".
4. **Versão não remove escopo:** o que não couber numa fatia passa para a próxima; retirar opcode do
   catálogo exige decisão do dono e linha nova em `lacunas.md`.
5. **A fatia não é calendário:** é ordem de entrega. Nenhuma versão tem data neste plano.

| Versão | Escopo da fatia | Gate de saída |
|---|---|---|
| **0.1.0** | fio sem payload: `HEADER`, enquadramento, ofuscação e checksum, limites, flags; páginas de método; **captura do handshake (`LPRT-03`) e goldens do fio como pré-requisito** | captura datada do handshake com a origem da `KeyWord` registrada, mais goldens de header, enquadramento e transformação aceitos; nenhum opcode de payload |
| **0.2.0** | catálogo base: famílias `autenticacao` e `personagem`, só depois de `LPRT-06` fechar | golden por família; login e seleção de personagem com nome acentuado provados ponta a ponta |
| **0.3.0** | `cena-e-mundo` e `movimento` | golden por família; entrada na cena e movimento provados |
| **0.4.0** | `combate` | golden por família; ataque e dano provados no fio, sem regra de jogo |
| **0.5.0** | `inventario` e `comercio` | golden por família; item e troca provados |
| **0.6.0** | `quest-e-eventos` | golden por família; quest e evento provados |
| **0.7.0** | `ui-e-chat` | golden com texto acentuado em chat e em campo de nome; encoding já fechado antes de `personagem` |
| **0.8.0** | fluxos internos `game ↔ db`, `db ↔ np` e `np` (`LPRT-19`) | golden de cada fluxo, sem consumidor novo na arquitetura |
| **0.9.0** | fechamento: matriz de capacidade por variante (`LPRT-15`), fuzz agendado (`LPRT-14`), páginas `aceitas` | cobertura de capacidade de todas as variantes; nenhuma página de `wire/` ou `payloads/` reprovada |
| **1.0.0** | congelamento: catálogo completo medido do gerador, goldens por família, compatibilidade declarada (`LPRT-09`) | total do catálogo publicado com fonte e data; nenhuma lacuna de prova aberta |

**Pré-requisito de prova no `0.1.0`:** a captura de `LPRT-03` e os goldens do fio entram na primeira
fatia como pré-requisito, do mesmo modo que a onda 2 de §9 — o fio sem payload só fecha com o handshake
medido e o byte provado. Não é decisão nova: é a exigência de `LPRT-10` aplicada à primeira fatia.
**A1 autorizada em 2026-11-23 (resposta A):** peer da opção A, `ECAP` e CLI autorizados; captura não
executada; `LPRT-03` só fecha após a segunda leitura e `LPRT-06` continua nas capturas B/C.

**Onde o fatiamento vira issue:** `docs/roadmap/fases.md` liga cada issue `WYDPRT-nnn` à versão cujo
gate ela serve e registra as regras dessa ligação. Ele cita esta seção como dona do escopo e do gate,
e esta seção cita ele como dono do recorte — mudar uma fatia exige atualizar as duas páginas na mesma
entrega.

---

## 10. Decisões do time

**18 decisões foram tomadas pelo dono** — 17 em 2026-11-20 e `LPRT-09` em 2026-11-23 — e valem como
regra deste plano; 2 continuam abertas porque dependem de captura. `lacunas.md` é o dono do estado, do
dono e do prazo; aqui fica o texto da decisão que o restante da árvore precisa citar.

| # | Decisão | Lacuna | Estado e decisão |
|---|---|---|---|
| 1 | cobertura do catálogo e sua distribuição até a `1.0.0` | `LPRT-01` | **decidida:** catálogo completo — todo o inventário do legado, sem se limitar a ele; 198 é baseline/piso, não total; total publicado medido do gerador; entrega fatiada em `0.1.0`→`1.0.0` (§9.1) |
| 2 | política de checksum no servidor | `LPRT-02` | **decidida:** recusar divergente, sem modo tolerante |
| 3 | origem da semente de ofuscação | `LPRT-03` | **aberta:** o crate recebe a `KeyWord`, origem não assumida; falta captura do handshake |
| 4 | fonte de verdade do layout de payload | `LPRT-04` | **decidida:** cliente + captura; legado só `H` |
| 5 | `repr(C)` × leitura explícita por offset | `LPRT-05` | **decidida:** leitura explícita, sem `unsafe` |
| 6 | encoding e terminador de string | `LPRT-06` | **aberta:** codec não transcodifica e trata bytes de tamanho fixo; encoding e terminador dependem de captura |
| 7 | comportamento em frame no limite e acima dele | `LPRT-07` | **decidida:** aceitar `Size = 8192`, recusar acima com erro tipado |
| 8 | reação a opcode desconhecido | `LPRT-08` | **decidida:** codec devolve `UnknownType`; `wyd-net` decide a reação |
| 9 | versão-alvo do cliente e compatibilidade | `LPRT-09` | **decidida (2026-11-23):** alvo é o cliente 7662 com versão e SHA-256 completos; outro build não é suportado |
| 10 | processo de captura e aprovação de golden | `LPRT-10` | **decidida:** captura datada, anonimização, `G-<familia>-<nn>`, origem registrada e segunda leitura |
| 11 | semântica de `FLAG_NEW` e das flags de direção | `LPRT-11` | **decidida:** direção roteia; `FLAG_NEW` é marcador de versão |
| 12 | quem garante ordem entre mensagens | `LPRT-12` | **decidida:** `wyd-net` garante; codec é stateless |
| 13 | taxonomia de erro e reação na conexão | `LPRT-13` | **decidida:** três famílias com classe recuperável/fatal; crate classifica, `wyd-net` reage |
| 14 | escopo de fuzz e orçamento no CI | `LPRT-14` | **decidida:** propriedade curta no PR + fuzz agendado com teto de tempo |
| 15 | como medir 85% num crate com tabelas geradas | `LPRT-15` | **decidida:** gerado fora da métrica de linha; matriz de capacidade por variante |
| 16 | gerador de tabela: próprio e reprodutível | `LPRT-16` | **decidida:** gerador próprio, entrada versionada, saída determinística, diff no CI |
| 17 | declaração de segurança da ofuscação | `LPRT-17` | **decidida:** ofuscação ≠ criptografia; checksum não é MAC |
| 18 | payload tipado × bytes crus na fronteira com `wyd-core` | `LPRT-18` | **decidida:** payload tipado e validado; bytes crus não saem do crate |
| 19 | se os fluxos `db ↔ game` e `np` entram no escopo | `LPRT-19` | **decidida:** os três fluxos entram (`game ↔ db`, `db ↔ np`, `np`) |
| 20 | publicação do catálogo no site (gerado ou escrito) | `LPRT-20` | **decidida:** catálogo escrito à mão citando os valores do gerador; mantém a `LPDOC-10`; o total exibido é o medido do gerador, na versão |

---

## 11. O que este documento não decide

- Não decide o layout de campo de cada payload: a fonte de verdade foi decidida em `LPRT-04`
  (cliente + captura), mas cada campo continua exigindo golden — `LPRT-10` define o processo, não a prova.
- Não decide arquitetura de `wyd-net` nem reação a pacote inválido na conexão.
- Não decide regra de jogo nem balanceamento.
- Não cria ADR: decisão registrada nasce com \"como verificar\", depois do \"sim\" do dono.
- Não mede qualidade de código: não existe código de produção na `1.0.0`.
