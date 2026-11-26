# Esqueleto de ponta a ponta — login, entrada no mundo e um passo

> Pergunta: "qual é a menor fatia que prova, com bytes e com banco, que o cliente entra no mundo e anda?"
> Público: Produto, arquitetura, `wyd-server`, `wyd-protocol`, `wyd-net`, `wyd-core`, `wyd-db`, `wyd-client`, `wyd-bots` e QA
> Dono: equipe do `wyd-server` (composição); o fio é de `wyd-protocol`, a captura é de `wyd-client`, o dado é de `wyd-db`
> Base: `wyd-1.0.0.md` §7.3 e §7.7; `wyd-protocol-1.0.0.md` §9.1; `wyd-protocol/docs/goldens/como-capturar.md` §1, §2 e §7; `wyd-client/lacunas.md` (`LCLI-04`, D3–D6); `wyd-core/lacunas.md` (`LCORE-01`–`LCORE-04`); `wyd-db/migrations/0006_game_character.md` §6.1 e `0007_game_session.md` §6.2/§7; `docs/specs/endpoint-de-captura.md`; `LPRT-10`
> Revisado em: 2026-11-23

**Estado:** proposto (`P`, 2026-11-23). Define **qual** fatia prova o encaixe das fronteiras e **como** se reconhece
que ela passou. Não afirma byte, nome de mensagem nem valor de opcode: nenhum dos três está medido — não há
captura, não há golden e não há código de produção (`V`, `wyd-1.0.0.md` §7; `wyd-client/registro-do-cliente.md` §7).

## 1. O que a fatia prova — e o que fica fora

"Esqueleto" é o caminho mínimo que atravessa **cliente real → fio → processo do canal → mundo em memória →
banco**, sem nenhuma regra de jogo além de mover um passo. Ele existe para provar que as fronteiras se encaixam
antes de qualquer família de mensagem nova; não é demonstração, não é feature e não substitui o E2E de nenhum
sistema.

| Dentro | Fora, e por quê |
|---|---|
| handshake e enquadramento provados por captura | — |
| autenticação de conta de jogo | cadastro e login web: `wyd-portal` (auto-cadastro decidido em `A7`); a fatia usa conta sintética do seed de desenvolvimento |
| lista e seleção de personagem | criação/edição de personagem: não é necessária para entrar e andar |
| entrada na cena com estado inicial | combate, inventário, chat e comércio: famílias de `0.4.0`–`0.7.0` (`wyd-protocol-1.0.0.md` §9.1) |
| um passo com resultado observável | durabilidade da posição: `game.character` não tem coluna de coordenada atual — só `recall_x`/`recall_y` (`wyd-db/migrations/0006_game_character.md` §6.1, `V` 2026-11-22); `LCORE-02` decidiu que posição é durável eventual com checkpoint no release, mas a fatia não prova save e o schema da posição continua futuro |
| ciclo de vida da sessão: aberta e liberada | dois canais no mesmo gate; o segundo entra depois da primeira fatia (`LSRV-02`) |
| — | patch, launcher e serverlist: `X11` decidida em 2026-11-23 (JSON canônico + Ed25519), implementação pendente; carga e convivência: onda E, com ambiente e conta decididos em 2026-11-23 (`LBOT-01`/`LBOT-02`) |

A consequência prática: **a fatia não prova save**. Ela prova que o fio, a sessão e a entidade em memória
existem e que o cliente recebe o resultado do passo. Quem quiser provar persistência de posição precisa de
decisão de schema antes — e essa decisão não é desta página.

## 2. A sequência mínima

Oito linhas: a pré-condição 0 e sete passos, nesta ordem. Nenhuma delas precisa de família de mensagem fora
de §3.

| # | O que precisa acontecer | Contrato | Estado hoje | O que falta |
|---|---|---|---|---|
| 0 | um processo abre TCP no canal jogável e aceita a conexão do cliente | `X7` | composição, canais, `ready` e shutdown decididos (`LSRV-01`–`LSRV-04`) | nada de decisão; falta o processo |
| 1 | o cliente e o servidor trocam o handshake CPSock (enquadramento, ofuscação, checksum) | `X1` | forma decidida, semente aberta | `LPRT-03` e golden do fio |
| 2 | a conta de jogo autentica | `X3`, `X5` | payload aberto; build-alvo fixado (`LPRT-09`) | `LPRT-06` |
| 3 | o servidor devolve a lista de personagens da conta | `X3`, `X5` | payload aberto | `LPRT-06` (nome é texto) |
| 4 | a seleção grava `character_id` na mesma sessão | `X5` | DDL fechado (`0007`); C4 decidida (`LCORE-04`) | implementar a porta e o adapter; teste de contrato |
| 5 | o personagem entra na cena com estado inicial | `X3` | `LCORE-01` decidida: cena + estado inicial, sem inventário/combate/chat | implementar a entrada na cena |
| 6 | um passo muda a posição e o resultado chega ao cliente | `X2`, `X3` | `LCORE-03` decidida: sim calcula deslocamento puro; core valida, resolve colisão, aplica e emite | implementar a chamada pura; fórmula incluída em `LSIM-01`; local/formato do vetor decididos em `LSIM-03` (`Q-SIM-3` = A), fixture ainda não implementada |
| 7 | a sessão é liberada com `released_at` e motivo | `X5` | decidido (`A10`, `A14`) | nada de decisão; falta código |

### 2.1 Regra `core` × `sim` aplicada ao passo 6 (`LCORE-03`, 2026-11-23)

O `wyd-sim` calcula apenas o **deslocamento puro**: recebe entrada tipada e imutável (por exemplo,
velocidade, tick e modificador de terreno) e devolve saída tipada, sem ler sessão/mapa, sem mutar
estado, sem relógio, RNG ou I/O; o mesmo vetor produz o mesmo vetor. O `wyd-core` valida sessão e
personagem, resolve mapa/colisão, consome o `ClientTick`, aplica a posição resultante e emite o
resultado. A fórmula concreta **não** é decidida aqui: é a de `LSIM-01` (`Q-SIM-1` = A) e a aritmética
é a de `LSIM-04` (`Q-SIM-4` = A), ambas em 2026-11-23.

**Prova mínima:** vetor de entrada/saída no `sim`; teste do core com mundo falso mostrando que um
estado inválido rejeita o passo sem mover a posição. A inexistência de fórmula duplicada no core é
defeito de revisão, não uma segunda implementação.

## 3. Opcodes mínimos

**Regra desta seção:** nome e valor de opcode **não** aparecem aqui. O catálogo é gerado (`LPRT-16`) e a página
dona é `wire/opcodes/*`, com o total medido em `wire/tabela-de-opcodes.md` (a escrever, `LPRT-20`) — páginas do
`wyd-protocol` que ainda **não foram escritas**. Até o golden existir, cada
linha é `H` com a lacuna que a fecha — inclusive a quantidade, que é o **mínimo desta fatia**, não o total do
sistema.

| # | Família | Direção | Papel da mensagem | O que ela precisa carregar | Fecha com |
|---|---|---|---|---|---|
| 1 | `autenticacao` | cliente → servidor | pedido de login | login, credencial e a versão que o cliente declara | `LPRT-03`, `LPRT-06`; build em `LPRT-09` |
| 2 | `autenticacao` | servidor → cliente | resultado do login | aceito ou recusado, com motivo tipado | `0.2.0` |
| 3 | `personagem` | servidor → cliente | lista de personagens | vaga, nome (texto), classe e nível | `LPRT-06` e `0.2.0` |
| 4 | `personagem` | cliente → servidor | seleção | vaga ou identificador do personagem | `0.2.0` |
| 5 | `cena-e-mundo` | servidor → cliente | entrada na cena | identificador do personagem, mapa, posição inicial e estado visível | `0.3.0` |
| 6 | `movimento` | cliente → servidor | pedido de passo | direção ou destino, mais o `ClientTick` | `0.3.0` |
| 7 | `movimento` | servidor → cliente | resultado do passo | posição resultante, mais o `ClientTick` | `0.3.0` |

O **envelope** não é opcode e não entra na contagem: `HEADER` de 12 bytes, `Size`, `KeyWord` e `CheckSum`. Os 12
bytes vêm do baseline do legado (nota de abertura de `wyd-protocol-1.0.0.md`; `H` para o codec novo) e são
provados pelo golden do fio, não por esta página.
Se a captura mostrar que o handshake tem mais de uma mensagem, o mínimo sobe — a página registra o número de
partida, não um teto.

## 4. Goldens mínimos

**Cinco cenários, seis capturas**, porque o procedimento proíbe cenário misturado: um cenário por captura, os
dois sentidos, a build na ficha (`goldens/como-capturar.md` §2, itens 2, 3 e 5) e a segunda leitura (§1,
item 4). A captura A foi dividida em A1 (handshake) e A2 (mundo) para que `LPRT-03` não dependa de login e
cena que ainda não existem.

| Captura | Cenário | Família | Goldens de mensagem que alimenta | Pré-requisito |
|---|---|---|---:|---|
| A1 | abrir o cliente contra o `ECAP` e capturar o handshake; registrar a build na ficha antes de qualquer login | fio | 0 (alimenta os 3 goldens do fio) | `ECAP` e `PEER-A1` ([spec](../specs/endpoint-de-captura.md)); peer pela opção A, porque C foi reprovada em 2026-11-23 (§9.1); build fixada em `Q-PRT-1` (7662 + SHA-256); D3 (`wyd-client`) |
| A2 | repetir a sessão e chegar ao mundo, sem outra ação | fio | 0 (confirma o handshake no fluxo) | A1 aceita + passos 1–5 |
| B | um login aceito e um login recusado com conta sintética | `autenticacao` | 2 | `LPRT-06` |
| C | lista de personagens e seleção | `personagem` | 2 | `LPRT-06` fechada |
| D | entrada na cena | `cena-e-mundo` | 1 | `0.2.0` |
| E | um passo e o resultado recebido | `movimento` | 2 | `0.3.0` |

**Mínimo da fatia:** 5 cenários (6 capturas, porque A tem A1 e A2) e **7 goldens de mensagem**, mais os **3
goldens do fio** que o gate da `0.1.0` já exige (`wyd-protocol-1.0.0.md` §9.1). Medido em 2026-11-23:
**0 capturas e 0 goldens** (`V`, `registro-do-cliente.md` §7). Nenhum ID `G-<familia>-<nn>` é alocado
aqui: o ID nasce na aceitação, no `goldens/catalogo.md`, que ainda não existe (`goldens/como-capturar.md` §7).

## 5. Contratos `X` envolvidos

| Contrato | Papel nesta fatia | Estado | O que falta |
|---|---|---|---|
| `X1` codec do fio | todo byte passa por ele, handshake inclusive | decidido (`LPRT-18`, `LPRT-08`, `LPRT-13`) | prova de byte pelos goldens |
| `X2` ordem por conexão | o passo não pode chegar antes da entrada na cena | decidido (`LPRT-12`) | teste de duas mensagens na mesma conexão |
| `X3` payload tipado | login, personagem, cena e movimento | forma decidida; payload aberto | captura por família (`LPRT-04`, `LPRT-06`) |
| `X5` leitura e persistência | conta, personagens e `game.session` | decidido (`LCORE-04`) | implementar C4 e o adapter; teste de contrato |
| `X6` estado durável × memória | **fora**: a fatia não prova save | decidido (`LCORE-02`) | schema da posição e checkpoint, fora desta fatia |
| `X7` composição do processo | um processo por canal, um canal nesta fatia | decidido (`LSRV-01`, `LSRV-02`) | nada de decisão; falta código |
| `X12` captura como prova | origem de todos os goldens | processo e D3–D6 decididos; `ECAP` especificado; peer pela opção A (C reprovada); 0 capturas | implementar `ECAP`, a CLI (`LCLI-04`) e o peer de A; capturar A1 |

Fora de `X`, a fatia depende de `N1`, `N2` e `N4` (`wyd-net`), `C1`, `C2` e `C4` (`wyd-core`) e `S5` (`wyd-server`);
todos estão citados em `wyd-1.0.0.md` §7.3 e nenhum é reaberto aqui.

## 6. Critério de prova de ponta a ponta

A fatia está provada quando **C1–C6 passam na mesma build e no mesmo ambiente**, sem `sleep` como sincronização,
sem `SKIP` e sem captura nova para o replay. Os comandos entram na issue de cada frente quando o código existir;
hoje `crates/*` tem só `.gitkeep` (`V`), então nenhum critério é executável.

| # | Critério | Como se observa |
|---|---|---|
| C1 | o fio está provado por captura | ficha datada com build e hash, segunda leitura registrada, `LPRT-03` fechada e os 3 goldens do fio aceitos |
| C2 | o handshake aceita o declarado e recusa o resto | replay do golden de login no servidor local cria sessão com `game.session.client_version` igual à versão da build capturada; byte alterado é recusado com erro tipado e **nada** é gravado (`N2`, `N3`) |
| C3 | o login cria exatamente uma sessão viva | `game.session` nasce com `account_id`, `group_id`/`channel` e `client_version`; credencial errada não cria linha; a segunda sessão viva da mesma conta é recusada por `ux_session_viva_por_conta` (`23505`) |
| C4 | a lista e a seleção são da conta autenticada | a lista é o conjunto de `game.character` da conta com `deleted_at IS NULL` (`A14`); depois da seleção, `game.session.character_id` deixa de ser nulo na **mesma** linha |
| C5 | entrar e andar produz o resultado esperado | a resposta do passo traz a posição resultante com o deslocamento esperado, e duas execuções do mesmo replay produzem o mesmo resultado; posição é de memória e a fatia **não** afirma persistência |
| C6 | a sessão é liberada com motivo | ao desconectar, a linha recebe `released_at` e `released_reason = 'disconnect'` (`A10`, `A14`); o reaper (60 s, órfã acima de 180 s) não é critério desta fatia |
| C7 | isolamento de canal (condicionado) | o mesmo fluxo no canal 2, sem estado compartilhado entre canais; `LSRV-02` decidiu 2 canais, mas C7 fica fora do gate da primeira fatia até o segundo ser implementado |

**Não são critério:** taxa de quadros, latência, número de jogadores, combate, balanceamento, aparência da tela.
Medir qualquer um deles aqui seria ampliar a fatia.

Uma pré-condição de banco que o critério C3 expõe: `game.session` tem FK composta para `game.server_channel`
(`fk_session_canal`, `23503`), e essa linha é **dado de instalação**, não seed
(`wyd-db/docs/arquitetura/contexto-c4.md`).
Sem provisionar o canal, nenhuma sessão nasce — e o erro aparece como falha de FK, não como falha de fio.

## 7. O que bloqueia a fatia hoje

| Bloqueio | ID | Dono | Efeito |
|---|---|---|---|
| origem da semente do handshake sem captura | `LPRT-03` | pesquisa + arquitetura | sem `KeyWord` provada, nenhum byte de login é afirmável |
| encoding e terminador de texto | `LPRT-06` | pesquisa + arquitetura | nome de personagem atravessa o fio como texto |
| build não registrada na ficha | `LPRT-09` decidido em 2026-11-23 (7662 + SHA-256) | `wyd-client` | sem a ficha, o `client_version` não é atribuível ao alvo |
| implementação do `ECAP` e da CLI | D3–D6 (`LCLI-04`); `docs/specs/endpoint-de-captura.md` | `wyd-server` + `wyd-client` | desenho do `ECAP` proposto; falta código e operação |
| peer de A1 | condição de `LSRV-05` reprovada em 2026-11-23 (§9.1 da spec) | `wyd-server` | A1 depende da opção A; falta implementar o peer no passo 0 |
| segundo canal para C7 | `LSRV-02` | produto + `wyd-ops` | a `1.0.0` tem 2 canais; o segundo só entra depois da primeira fatia |
| conta sintética do seed de desenvolvimento | `seeds/dev.sql` (`M19`) | `wyd-db` | o plano prevê o arquivo; ele não existe (`V`, `crates/` só tem `.gitkeep`) |
| senha do seed sob a política de senha | `B3`, `M19` | produto + arquitetura | conferir que a conta semeada autentica sob 15–128 chars sem reabrir `M19` |

Enquanto `LPRT-03` não fechar, C1 não existe; e sem `ECAP` e sem `PEER-A1` C1 não pode nem ser tentado. É o
mesmo bloqueio que `wyd-1.0.0.md` §7.5 registra: **a onda jogável não começa antes do handshake provado**.

## 8. O que esta proposta não decide

- Não fixa nome, valor nem total de opcode: catálogo gerado e `wire/opcodes/*` são os donos.
- Não fixa porta de `EPROD`; `ready` e encerramento estão decididos (`LSRV-03`, `LSRV-04`), e a sequência do shutdown é de `WYDDB-021`. O `ECAP` usa loopback e porta efêmera, separado de `EPROD`. Composição e canais já estão decididos (`LSRV-01`, `LSRV-02`).
- Não fixa a fórmula de movimento: a fronteira está decidida (`LCORE-03`), mas velocidade e colisão seguem a fórmula de `LSIM-01` e a implementação do core; a aritmética é a de
`LSIM-04`.
- Não decide schema de posição: a classificação e a frequência estão em `LCORE-02`/`X6`; a fatia não prova save.
- Não aloca ID de golden nem cria issue, ADR, prazo ou responsável nominal.
- Não muda o gate de nenhuma onda: a onda C continua com o gate de `wyd-1.0.0.md` §7.7, e esta página apenas
  detalha a fatia que o serve.
- Não fixa o próprio endereço: se o time decidir que a fatia pertence ao panorama ou a outro sistema, o conteúdo
  e os IDs citados continuam válidos — muda o endereço, não a prova.

## 9. Ordem de fechamento

| Etapa | O que fecha | Dono | Destrava |
|---|---|---|---|
| 1 | `ECAP` de captura (`D3`; `docs/specs/endpoint-de-captura.md`), CLI Rust (`D4`) e peer de A1 pela opção A | `wyd-server` + `wyd-client` | capturas A1 e A2 |
| 2 | captura A1 e os 3 goldens do fio; A2 confirma o fluxo | `wyd-client` captura; `wyd-protocol` aceita | `LPRT-03`, `0.1.0` |
| 3 | captura B e C, com `LPRT-06` antes da C | idem | `0.2.0`, passos 2–4 |
| 4 | capturas D e E | idem | `0.3.0`, passos 5–6 |
| 5 | replay dos 7 goldens no bot, em CI | `wyd-bots` + `wyd-ops` | C2–C6 como regressão |
| 6 | repetir o fluxo no canal 2 | `wyd-server` | C7, quando o segundo canal for implementado |

Cada etapa fecha em issue própria, com DoD copiado do critério que ela serve. Esta página não cria issue
(`AGENTS.md` §5, item 9).
