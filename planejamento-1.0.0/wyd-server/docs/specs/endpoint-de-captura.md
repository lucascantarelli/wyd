# ECAP — endpoint de captura (especificação)

> Pergunta: "como o `wyd-server` opera um listener local que grava o handshake do cliente sem se confundir com o endpoint de produção e sem interpretar frames?"
> Público: arquitetura, `wyd-server`, `wyd-client`, `wyd-protocol`, QA e operação
> Dono: equipe do `wyd-server` (endpoint); contrato de captura aprovado por `wyd-protocol`; operação pela CLI de `wyd-client`
> Base: D3–D6 (`wyd-client/lacunas.md`); `LPRT-10` e `goldens/como-capturar.md`; `X7`; `esqueleto-de-ponta-a-ponta.md` A1; `LCLI-04`; `AGENTS.md` §§5.1, 5.4, 5.6 e 5.8
> Revisado em: 2026-11-23

**Estado:** proposta (`P`, 2026-11-23). Esta página fecha o desenho técnico do endpoint de captura. O peer
que completa o handshake em A1 foi **decidido** em `Q-SRV-5`/`LSRV-05` como opção **C, com quarentena**
(servidor legado de `deprecated/` como harness local); a medição de 2026-11-23 **reprovou a condição**
dessa opção e A1 passa a depender da **opção A** (§9.1). Não há código, crate, binário, servidor, CLI,
captura ou golden produzido por esta página. A implementação do recorte A1 começa pela autorização de
2026-11-23; a spec permanece `proposta` como documento até a revisão editorial.

**A1 autorizada em 2026-11-23 (resposta A):** implementar o peer da opção A com handshake mínimo local, o
`ECAP` e a CLI, e executar a captura A1. A autorização **não** fecha `LPRT-03` nem `LPRT-06`: `LPRT-03`
só fecha após captura completa, segunda leitura e golden; `LPRT-06` continua dependente das capturas B/C
e do seed sintético.

## 0. Como ler: FECHADO e ABERTO

Esta página é uma proposta; "FECHADO" abaixo significa **fechado no desenho desta proposta**, não
`aceito` editorialmente. O que é herdado de D3–D6, `LPRT-10` ou `X7` não é reaberto. O que é `ABERTO`
não pode ser implementado por suposição.

| Marca | Significado | Efeito |
|---|---|---|
| **FECHADO (herdado)** | decisão já registrada em D3–D6, `LPRT-10`, `X7` ou no esqueleto | a implementação segue; esta página não redefine |
| **FECHADO nesta spec** | escolha técnica desta proposta, necessária para o endpoint existir | vira requisito após o aceite desta página |
| **ABERTO** | falta resposta do dono ou de outro sistema | bloqueia implementação ou captura; não se inventa valor |

| Seção | Estado | O que falta |
|---|---|---|
| 1. Nomes e fronteira | **FECHADO nesta spec** | nada; `EPROD` e `ECAP` são nomes distintos |
| 2. Topologia | **FECHADO nesta spec** | peer definido em `Q-SRV-5`, mas a condição de C foi reprovada (§9.1): vale o peer da opção A |
| 3. Listener e ciclo de vida | **FECHADO nesta spec** | nada de decisão; falta implementação |
| 4. Gravação | **FECHADO nesta spec** | falta implementar; formato e local estão definidos |
| 5. Segurança verificável | **FECHADO nesta spec** | falta provar em implementação; o peer é o da opção A, porque a condição da opção C foi reprovada (§9.1) |
| 6. Controle pela CLI | **FECHADO nesta spec** | adaptador do serverlist é implementação de `LCLI-04`; formato do serverlist em `LLCH-02`/`X11`; build-alvo fixado em `LPRT-09` (2026-11-23) |
| 7. Captura completa | **FECHADO nesta spec** | falta executar A1 depois de 2, 3 e 6 |
| 8. DoD e issue | **FECHADO nesta spec** | issue não criada; implementação não iniciada |
| 9. Peer de A1 | **DECIDIDO em C, condição reprovada** (medição de 2026-11-23) | §9.1: clone limpo não tem o peer, boot exige `MIGRATIONS_DIR` e o gate espera 12000; A1 passa a depender de A |

## 1. Nomes e fronteira

Dois nomes substituem o termo composto que vinha sendo usado. Não são sinônimos e não compartilham
processo, porta, imagem ou configuração.

| Nome | O que é | Onde vive | Estado |
|---|---|---|---|
| **EPROD** | endpoint de produção: listener TCP/CPSock do processo de canal que atende o cliente real; contrato `X7`, passo 0 do esqueleto | processo `wyd-server` de cada canal, no ambiente de jogo | **não implementado**; porta e deploy não são decididos nesta página |
| **ECAP** | endpoint de captura: listener TCP local que recebe o cliente, encaminha bytes ao peer e grava os dois sentidos; função de proxy gravador | processo local separado, binário de captura do `wyd-server`, nunca no ambiente de jogo | **especificado aqui; não implementado** |
| **PEER-A1** | peer de destino que completa o handshake do cliente em A1 | peer da opção A: listener local do `wyd-server` no passo 0, em loopback | **C (legado) reprovada na medição** (2026-11-23); vale A (§9.1) |
| **CLI** | ferramenta de captura em Rust de `wyd-client/tools/`, decidida em D4 | máquina do operador; inicia e controla o ECAP | **decidida em princípio; não implementada** |

**Fronteira fechada.**

1. `EPROD` é o único endpoint que recebe jogador em produção. Ele não grava bruto, não é loopback por
   definição e não é controlado pela CLI de captura.
2. `ECAP` é o único componente desta spec que escuta para captura. Ele não é `EPROD`, não é um modo de
   produção e não é publicado no compose de jogo.
3. `ECAP` não conhece conta, personagem, sessão, SQL, opcode, `KeyWord` ou checksum. Ele move bytes e
   grava o que move; qualquer semântica é do peer e do `wyd-protocol`.
4. `PEER-A1` é a única peça que responde ao CPSock. Sem ele, o ECAP grava bytes, mas não produz uma
   captura de handshake conclusiva.
5. A CLI configura o cliente e opera o ECAP; ela não implementa o proxy nem o servidor (D4 e `LCLI-04`).

**Fora desta página:** porta e implementação de `EPROD`, autenticação de jogador, schema, migrations,
login, seleção de personagem, protocolo, golden, segunda leitura e publicação. Esses itens continuam
nos sistemas donos.

## 2. Topologia

**FECHADO nesta spec**, com a ressalva do peer.

```text
WYD.exe  --TCP-->  ECAP 127.0.0.1:<porta efêmera>  --TCP-->  PEER-A1 127.0.0.1:<porta>
   ^                                                            |
   |                                                            |
   +------------------ dados capturados nos dois sentidos ------+

CLI  --stdin/stdout (processo filho)-->  ECAP
```

| Pergunta | Resposta desta spec | Estado |
|---|---|---|
| Quem escuta | `ECAP`, em `127.0.0.1`, TCP, com porta escolhida pelo sistema (`bind` na porta 0) | **FECHADO** |
| Em que endereço/porta | somente loopback; a porta efêmera é publicada no evento `ready`; não há porta fixa de captura | **FECHADO** |
| Quem é o peer | `PEER-A1`, obrigatoriamente loopback: o peer da opção A (servidor do `wyd-server`), porque a condição de C foi reprovada em 2026-11-23 | **C REPROVADA; peer por A** |
| Quem inicia | a CLI inicia o `ECAP` como processo filho e só depois inicia o cliente | **FECHADO** |
| Quem encerra | o operador pela CLI; o `ECAP` também encerra em erro fatal, EOF bilateral, timeout ou sinal | **FECHADO** |
| Conexões aceitas | exatamente uma conexão de cliente por processo; listener fecha após o primeiro `accept` | **FECHADO** |
| Duração | sessão máxima de 120 s; sem cliente em 30 s, encerra; ociosa por 20 s após o primeiro byte, encerra | **FECHADO** |
| Desconexão | meia-fechamento propagado; drena o outro sentido por até 5 s; registra `client_eof`/`peer_eof`; fecha | **FECHADO** |
| Segundo cliente | recusado e registrado; não reinicia a sessão; exige novo processo | **FECHADO** |
| Tráfego remoto | recusado: cliente e peer precisam ser loopback; sem DNS, sem endereço de rede | **FECHADO** |

**Regra de topologia.** O `ECAP` precisa ser o par TCP do cliente para que o primeiro byte seja lido
diretamente, sem depender de captura de pacote do sistema operacional. Ele conecta ao `PEER-A1` depois
de aceitar o cliente; os bytes que o cliente enviar nesse intervalo ficam no buffer do socket e são
lidos na ordem. O `ECAP` não usa `pcap`, `tcpdump`, proxy de sistema, redirecionamento de porta ou
privilégio de administrador.

**Desconexão e timeout em detalhe.**

- cliente fecha: o `ECAP` para de ler o cliente, meia-fecha o peer e continua lendo o peer até EOF ou
  até o limite de drenagem; a captura só é completa se o evento exigido por A1 já tiver sido marcado.
- peer fecha: o `ECAP` meia-fecha o cliente, registra `peer_eof` e encerra a captura como incompleta,
  salvo se o handshake já estava marcado e o cliente também já tinha EOF.
- estouro de tempo: registra `timeout` com o valor, fecha os dois lados, grava `END` incompleto e
  devolve erro à CLI; não gera golden.
- `Ctrl+C`/`SIGINT`: a CLI envia `stop`; o `ECAP` tenta marcar `capture_stop`, gravar `END` e fechar.
- falha de escrita no peer: registra `error`, fecha o cliente e encerra como incompleto; não descarta o
  que já foi gravado.

## 3. Contrato do listener e ciclo de vida

**FECHADO nesta spec.** O `ECAP` é um processo local, não um serviço HTTP. Não há endpoint de
controle em TCP, não há autenticação de jogador e não há TLS: o limite é o loopback e o processo filho.

### 3.1 Sequência

1. A CLI valida `--capture-root` fora do repositório, o caminho do cliente e o peer loopback.
2. A CLI inicia o `ECAP` com `--capture-root`, `--peer` e `--scenario`; o `ECAP` abre o listener em
   `127.0.0.1:0`.
3. O `ECAP` emite em stdout uma linha JSON `ready` com `listen`, `session_id`, caminho do bruto e
   versão. A CLI só continua depois de `ready`.
4. A CLI prepara uma cópia temporária da configuração do cliente apontando para o `listen` e inicia o
   `WYD.exe`; o arquivo original não é alterado.
5. O `ECAP` aceita a primeira conexão; fecha o listener; abre a conexão com o `PEER-A1`; começa a
   gravar e a retransmitir.
6. O operador marca `handshake_end` quando a tela mostra o fim do handshake; a CLI envia o comando de
   controle. O `ECAP` não infere esse fim por bytes.
7. A CLI espera 2 s de quiescência, envia `stop`, e o `ECAP` grava `capture_stop` e `END`.
8. A CLI espera o processo terminar, confere o hash do cliente e escreve/fecha a ficha.

### 3.2 Comandos de controle

O controle é **processo local**, por linhas JSON em stdin/stdout; não é API HTTP, não é socket de
controle e não é acessível pela rede.

| Comando | Efeito | Resposta |
|---|---|---|
| `status` | consulta estado | `{"event":"status","state":"listening\|proxying\|stopping\|closed",...}` |
| `mark <evento>` | grava uma marca de controle; eventos permitidos: `handshake_end`, `phase_note` | `{"event":"marked","name":"...","seq":N}` |
| `stop` | inicia encerramento gracioso | `{"event":"stopping"}` e depois `closed` |
| `abort` | fecha sem prometer captura completa | `{"event":"aborted"}` |

A CLI é a única cliente do controle. O `ECAP` não abre uma segunda porta para controle; o stdout é
exclusivo para eventos JSON e o stderr para diagnóstico. Bytes do jogo nunca passam pelo controle.

### 3.3 Códigos de erro

`E_BIND` (não conseguiu escutar em loopback), `E_SECURITY` (cliente ou peer fora do loopback),
`E_PEER` (peer inválido), `E_SECOND` (segunda conexão), `E_TIMEOUT`, `E_IO`, `E_CONFIG` (caminho ou
build inválido) e `E_ABORT`. O erro
fecha a sessão e aparece na ficha; não vira golden.

## 4. Gravação

**FECHADO nesta spec.** O `ECAP` é transparente: lê bytes do cliente e escreve os mesmos bytes no
peer; lê bytes do peer e escreve os mesmos bytes no cliente. Não decodifica `HEADER`, `Size`,
`KeyWord`, `CheckSum`, opcode, string ou frame. Não junta nem separa mensagens; a fronteira de frame é
do codec e do golden, não do gravador.

### 4.1 O que é gravado

- cada leitura bem-sucedida de um sentido vira um registro `DATA`, na ordem em que foi lida;
- cada evento de controle vira um registro `MARK`, com timestamp relativo e JSON sem bytes de jogo;
- o primeiro registro de controle é `capture_start`, escrito antes do `listen`;
- `client_connected`, `peer_connected`, `handshake_end`, `client_eof`, `peer_eof`, `capture_stop`,
  `timeout` e `error` são marcas; `handshake_end` só existe por comando da CLI;
- o último registro é `END`, com contagens, bytes por sentido, motivo e hash do prefixo.

**Não é gravado:** frame interpretado, opcode, texto decodificado, credencial, senha, sessão, SQL,
nome de personagem ou conteúdo de chat. O bruto contém bytes de fio e metadados de captura; a
anonimização e o frame entregue ao repositório são o passo seguinte de `como-capturar.md` §6.

### 4.2 Formato `capture.wydcap` versão 1

O arquivo é binário, little-endian, com um cabeçalho JSON e registros sequenciais.

```text
8 bytes   magic ASCII "WYDCAP01"
u16       versão do formato = 1
u16       tamanho do cabeçalho JSON
N bytes   cabeçalho JSON UTF-8
registros:
  u8      direção: 0=META, 1=cliente->servidor, 2=servidor->cliente
  u8      tipo: 1=DATA, 2=MARK, 3=END
  u16     flags = 0
  u64     seq global, começando em 0
  u64     timestamp relativo em nanossegundos desde capture_start
  u64     offset acumulado do sentido (0 para META)
  u32     tamanho do payload
  N bytes payload
  u32     CRC32 do registro sem o próprio CRC
```

O cabeçalho JSON contém, no mínimo: `session_id`, `scenario`, `started_at_utc`, `client_path`,
`client_sha256`, `client_size`, `ecap_version`, `peer_kind`, `peer_endpoint`, `listen_endpoint`,
`operator` e `capture_tool_version`. Caminho completo do cliente e operador ficam no bruto fora do
repositório; a ficha resumida usa apenas o identificador anonimizado.

O `payload` de `MARK` é JSON UTF-8 com `event`, `at_ns` e, quando aplicável, `reason`, `timeout_ms` ou
`note`. O `payload` de `END` contém `complete`, `reason`, `records`, `bytes_c2s`, `bytes_s2c`,
`first_seq`, `last_seq`, `sha256_prefix` e `client_sha256_after`. O `END` é o último registro; arquivo
sem `END` é incompleto.

### 4.3 Integridade, flush e fechamento

- cada registro é escrito com uma única escrita lógica; `DATA` faz flush após o registro;
- `MARK` faz flush imediato; `END` faz flush e `sync_all`;
- um arquivo truncado no meio de um registro falha a verificação de CRC e é incompleto;
- a CLI calcula `SHA-256` do `capture.wydcap` fechado e grava `capture.sha256`;
- o `END` contém o `sha256_prefix` de todos os bytes anteriores a ele; a CLI confere o prefixo e o
  arquivo inteiro;
- o `ECAP` não sobrescreve captura existente: usa `session_id` novo e recusa colisão.

### 4.4 Local restrito

O padrão é `%LOCALAPPDATA%\WYD\capture\<session_id>\capture.wydcap`, com `ficha.md` e
`capture.sha256` na mesma pasta. `--capture-root` pode apontar para outro diretório, mas a CLI recusa
qualquer caminho que resolva para dentro do repositório Git. O diretório é criado com permissão do
usuário; o bruto não é copiado para `planejamento-1.0.0/`, `tests/` ou qualquer pasta versionada.
Retenção e descarte são de D5: 30 dias após a segunda leitura, fora do repositório.

### 4.5 Correlação com a build

A CLI calcula `SHA-256` e tamanho de `WYD.exe` **antes** de iniciar o ECAP e os envia no cabeçalho. Ao
encerrar, calcula de novo; se o hash ou o tamanho mudou, a captura é incompleta e a ficha registra
`client_changed=true`. O hash é a identidade da build contra o alvo fixado em `Q-PRT-1` (7662, 2026-11-23);
a captura confere esse alvo, não promove outro build.

## 5. Segurança verificável

**FECHADO nesta spec.** A segurança não é uma frase de intenção; cada item tem uma prova de
implementação. Enquanto o item não tiver prova, o `ECAP` não é considerado seguro para A1.

| Requisito | Como o desenho garante | Como será provado |
|---|---|---|
| escutar somente loopback | bind fixo em `127.0.0.1`; não aceita `0.0.0.0`, `::`, hostname ou interface de rede | teste que tenta `--listen 0.0.0.0` e espera `E_BIND`; `netstat`/`ss` mostra só `127.0.0.1` |
| recusar cliente remoto | após `accept`, confere `peer_addr.ip().is_loopback()`; se falso, fecha e marca `E_SECURITY` | conectar de outro host/interface e receber fechamento; captura não é criada |
| recusar peer remoto | resolve o peer e exige IP loopback; DNS e endereço remoto são `E_PEER` | teste com IP de rede e com hostname; espera recusa antes de gravar |
| não confundir com produção | binário separado `wyd-capture-endpoint`; não entra na imagem/compose de jogo; sem porta publicada; banner `ECAP` e `mode=capture` | `docker compose config` de produção não contém o binário nem a porta; inspeção da imagem; teste de startup com perfil de produção recusa |
| não aceitar credencial de produção | o `ECAP` não tem DSN, cliente de banco, auth, portal ou configuração de produção; só retransmite para peer loopback | inspeção de configuração e teste de variáveis de produção rejeitadas; nenhuma conexão de rede fora do loopback |
| conta sintética | o peer e o banco local usam seed de captura; o `ECAP` não classifica conta | checklist de ambiente: banco `wyd_capture`, seed sintética, peer local; a decisão de `Q-SRV-5` exige que o peer recuse outra origem |
| bruto fora do repositório | `--capture-root` validado; recusa caminho dentro do worktree; escrita só sob `<root>` | teste com raiz dentro do repositório espera `E_CONFIG`; `git status` não mostra bruto |
| não vazar dado em log | logs só com evento, sessão, tamanhos e caminho; nunca payload, credencial ou nome | inspeção de log de uma captura sintética; busca por bytes conhecidos não encontra |

**Limite honesto sobre "conta real".** O `ECAP` não vê credencial e não consegue provar que um login
é sintético. A garantia é de ambiente: não há DSN de produção, não há rota de rede, o peer é local e o
banco é o de captura com seed sintética. Se o peer escolhido em `Q-SRV-5` não conseguir recusar uma
conta fora desse seed, ele não serve para A1. Não se promete detectar "conta real" pelo nome; promete-se
não haver caminho para a identidade de produção.

**Prova de não confusão.** `EPROD` e `ECAP` têm nomes, binários, portas, perfis e caminhos distintos.
O `ECAP` não é um subcomando do servidor de produção: é um alvo de build separado, com banner e
configuração próprios. Um operador que tente usá-lo no compose de jogo recebe recusa de configuração,
não um servidor de captura silencioso.

## 6. Controle pela CLI

**FECHADO nesta spec**, com uma dependência externa declarada.

A CLI é um processo local, não um serviço. Ela inicia o `ECAP` como filho, fala por stdin/stdout em
JSONL, e não expõe API de controle. O canal de dados do jogo nunca passa pela CLI.

### 6.1 Comandos

| Comando | O que faz |
|---|---|
| `capture start --scenario A1 --client <WYD.exe> --capture-root <fora-do-repo> --peer 127.0.0.1:<porta> --ecap <binário>` | valida, calcula hash, inicia o ECAP, espera `ready`, prepara a cópia do cliente e inicia o cliente |
| `capture mark handshake-end` | envia `mark handshake_end` ao ECAP; não interpreta bytes |
| `capture stop` | espera quiescência, envia `stop`, espera `closed`, confere hash e fecha a ficha |
| `capture status` | mostra estado e caminho do bruto |
| `capture verify <capture.wydcap>` | confere magic, CRC, `seq`, offsets, marcas, `END` e hash; falha se incompleto |

### 6.2 Configuração do cliente

A CLI **não altera o cliente original**. Ela copia o diretório/configuração necessário para uma pasta
temporária, aponta a cópia para `127.0.0.1:<porta>` e executa o `WYD.exe` a partir dela; ao terminar,
remove a cópia. O adaptador que conhece o arquivo de servidor do build é escolhido por
`--client-config-adapter`; ele é implementação de `LCLI-04`, e o formato do serverlist é o do launcher
(`LLCH-02`/`X11`); o build-alvo está fixado em `LPRT-09` (2026-11-23).

**DEPENDÊNCIA DE IMPLEMENTAÇÃO (não é lacuna):** sem o adaptador do serverlist, a CLI não consegue
apontar o cliente para o ECAP. O build-alvo deixou de ser bloqueio em 2026-11-23 e `LCLI-01` está
fechada; o adaptador é trabalho de `LCLI-04`. A captura A1 não começa antes de o adaptador existir.

### 6.3 Saída da CLI

A CLI produz, no diretório da sessão: `capture.wydcap`, `capture.sha256` e `ficha.md` (campos de
`como-capturar.md` §7). Para o repositório, só pode seguir o frame anonimizado e a ficha resumida,
conforme D5 e §6 do procedimento; esta spec não versiona bruto.

## 7. Definição de captura completa

**FECHADO nesta spec.** "Completa" é uma propriedade técnica do arquivo; "aceita" é o resultado da
segunda leitura. Uma captura completa ainda pode ser inconclusiva para `LPRT-03`.

Uma captura A1 é **completa** somente quando todos os itens passam:

1. `capture_start` existe antes de qualquer `DATA` e o primeiro `DATA` de cada sentido tem `offset=0`.
2. Há pelo menos um `DATA` em cliente→servidor e um em servidor→cliente.
3. `seq` é contíguo, sem buraco; `offset` de cada sentido é contíguo e a soma dos tamanhos bate com o
   `END`.
4. Existe `handshake_end`, marcado pela CLI, antes de `capture_stop`; o ECAP não inferiu o fim.
5. Existe `capture_stop` e o `END`; não existe `timeout`, `error` ou `E_SECURITY` não tratado.
6. O `END` tem `complete=true`; o CRC de cada registro e o `sha256_prefix` conferem.
7. `capture.sha256` confere com o arquivo fechado.
8. O `SHA-256` e o tamanho de `WYD.exe` no fim são iguais aos do cabeçalho; `client_changed=false`.

**Prova de "desde o primeiro byte".** O ECAP é o par TCP do cliente, não um observador externo. Ele
abre o listener antes de o cliente conectar, aceita a conexão, começa a ler imediatamente e grava o
primeiro `DATA` com `offset=0`; o cliente não pode enviar bytes antes de a conexão TCP existir. O
`capture_start` é gravado antes do `listen`. Um arquivo sem `DATA` inicial, com offset inicial maior
que zero ou com `seq` faltando é incompleto, não "quase completo".

**Inconclusiva para A1** se: não houver `handshake_end`; um dos sentidos estiver vazio; o peer não
responder ao CPSock; o build/hash não estiver identificado; ou o `END` faltar. Nesses casos a captura
é registro, não prova, e `LPRT-03` continua aberta.

**Segunda leitura.** A captura completa não é aceita por quem capturou. QA designado (ou o dono do
`wyd-protocol` como suplente) refaz a medição a partir do bruto e da ficha, conforme `LPRT-10` e D6; o
nome do QA continua em `LCLI-03`. Só depois da segunda leitura o frame anonimizado pode virar golden e
`LPRT-03` pode ser considerada.

## 8. DoD, arquivo de saída e issue

**FECHADO nesta spec** como definição; **não executado**. A issue abaixo segue o formato de
`wyd-db/docs/roadmap/WYDDB-*.md` e não foi criada no GitHub.

### Issue proposta — "ECAP e CLI de captura A1"

**Fase:** 0 — fio e captura. **Prioridade:** P0, porque A1 destrava `LPRT-03` e a `0.1.0`.
**Contexto:** o endpoint de captura só existe como decisão de princípio; sem ele não há bruto, ficha,
segunda leitura nem golden do handshake.
**Entregável:** binário `wyd-capture-endpoint` (ECAP) com listener loopback, peer, gravação
`capture.wydcap` e verificação; CLI Rust de `wyd-client/tools/` que inicia, marca `handshake_end`,
para e produz a ficha.
**Depende de:** `Q-SRV-5` respondida (feito em 2026-11-23) e peer da opção A implementado, porque a condição de C foi reprovada; `LCLI-04` para a CLI e o adaptador do serverlist, com o build fixado em `LPRT-09`;
`LPRT-10`/D5/D6 para ficha e segunda leitura. Não depende de `LPRT-03` para gravar bytes; depende dela
para concluir a medição.

**DoD verificável.**

- [ ] A spec foi aceita pelo dono do `wyd-server` e pelo dono do `wyd-protocol`.
- [ ] O peer de A1 em uso é o da opção A, porque a condição de C foi reprovada em 2026-11-23 (§9.1); o legado não entra na captura.
- [ ] O ECAP recusa bind fora de loopback, cliente remoto e peer remoto, com os códigos de §3.3.
- [ ] O ECAP não entra na imagem/compose de produção e não publica porta; a prova está no pipeline.
- [ ] A CLI inicia e para o ECAP por processo local, sem API de controle, e não altera o cliente original.
- [ ] A1 produz `capture.wydcap`, `capture.sha256` e `ficha.md` fora do repositório.
- [ ] `capture verify` prova `seq`/offset/CRC/`END`, dois sentidos, `handshake_end` e hash do cliente.
- [ ] A ficha registra build, peer, operador, caminho do bruto, descarte e o que **não** foi medido.
- [ ] A segunda leitura é assinada por QA designado, nunca por quem capturou.
- [ ] `LPRT-03` só é considerada fechada com o golden do handshake e a segunda leitura; `LPRT-09`
      recebe o hash da build; `LPRT-06` continua dependente da captura de texto, não desta A1.

**Fora de escopo:** implementar `EPROD`, protocolo, login, personagem, cena, movimento, banco, DDL,
golden, catálogo ou issue no GitHub; definir a porta de produção; escolher o peer por conta própria.

**Risco:** usar o peer legado como se fosse verdade pode contaminar conclusão sobre o servidor; o
desenho o trata como harness e marca os bytes do peer como `H`. O outro risco é o adaptador do cliente
não existir para o build; nesse caso A1 fica bloqueada, não se inventa configuração.

**Evidência ao fechar:** `capture.wydcap` + `capture.sha256` + `ficha.md`; saída de `capture verify`;
registro da segunda leitura; issue e commit correspondentes (quando existirem).

## 9. Decisão registrada — peer de A1 (`Q-SRV-5` / `LSRV-05`)

**DECIDIDO em 2026-11-23: opção C, com quarentena — condição reprovada na mesma data.** A opção C
definiu o **servidor legado de `deprecated/`** como peer de A1, executado localmente e somente como
harness de captura. As opções A–D continuam abaixo como registro da comparação que levou à escolha, e
§9.1 registra por que a condição falhou e por que A1 passa a depender de A.

**Critério de escolha:** o peer precisa (a) completar o CPSock nos dois sentidos, (b) permitir capturar
o primeiro byte do cliente, (c) não exigir que o `wyd-server` já implemente o handshake que A1 tenta
medir, (d) ficar em loopback e fora de produção, e (e) poder ser descartado sem virar fonte de verdade.

| Opção | O que é | Vantagens | Custos | Riscos |
|---|---|---|---|---|
| **A — servidor local novo do `wyd-server`** | implementar o passo 0 e usar uma instância local do listener do servidor como peer; não é o `EPROD` de produção | mesmo código que será produção; caminho único; sem legado | depende de implementar o servidor antes de A1; exige banco/canal/configuração | circular: o handshake é justamente o que A1 precisa medir; pode atrasar o fio |
| **B — harness mínimo de teste** | processo pequeno que responde ao CPSock só para A1 | isolado; não arrasta banco/canal; controlável | precisa conhecer o handshake; qualquer resposta é hipótese | inventa comportamento e pode produzir captura inconclusiva; vira protocolo paralelo se não for descartado |
| **C — servidor legado de `deprecated/` como harness** | rodar o servidor legado localmente, apenas como peer, e capturar o cliente | é o único peer já presente no repositório; se executar, dá dois sentidos sem inventar handshake novo | código ignorado pelo git, possível dificuldade de build, versão e licença; executabilidade não verificada; não é fonte de verdade | contaminar conclusões com comportamento legado; ausência em clone limpo; precisa ser marcado como `H` e descartado |
| **D — modo de captura com feature flag no `wyd-server`** | o servidor novo ganha um modo `capture` que escuta loopback e grava | reutiliza o caminho do servidor; um binário | exige servidor e handshake; feature flag amplia superfície; risco de configuração de produção | confundir captura com produção; o modo pode vazar para deploy; mais código antes de A1 |

**Resposta do dono em 2026-11-23: C, com quarentena.** O critério que a sustenta é o (c): não criar
dependência circular. O peer legado entra somente como **harness de captura**: binário local, loopback,
versão/hash na ficha, sem copiar código, sem promover seus bytes a fato e sem concluir comportamento do
servidor a partir dele. A execução do legado foi medida em 2026-11-23 (§9.1): o binário sobe em
loopback, mas a condição da opção C **não se cumpriu**. A contingência declarada na proposta se aplica —
A1 passa a depender de **A** — e B continua recusada. A captura segue não executada; a medição muda o
estado do peer, não o da evidência.

### 9.1 Ficha medida do peer (2026-11-23)

Execução local do agente, para medir se o peer legado serve como harness de A1. **Não é captura**, não
produz golden e não foi feita com o cliente real. O operador da captura continua não designado
(`LCLI-03`).

| Campo | Valor |
|---|---|
| Binário | `deprecated/target/debug/wyd-server.exe` (build de depuração, Windows) |
| Versão declarada | `0.1.0` (log `event=boot … version=0.1.0`) |
| Bytes | 11.713.024 |
| Data do arquivo | 2026-11-15 18:39 |
| SHA-256 | `0758d4f3080e07a105da8c74ed888d806444d982ca52fb1b37ddfe656ff4e9ee` |
| Banco | `postgres:17-alpine` efêmero, `127.0.0.1:55432` |
| Migrations | `MIGRATIONS_DIR=deprecated/crates/wyd-db/migrations` |
| Binds | `127.0.0.1` no CPSock, no `/metrics`, no status TCP e no status UDP |
| Resultado | sobe em loopback; `applied=53` migrations em banco vazio; `/health` responde `200` com `{"ok":true,…,"schema":53}` |
| Repetição | segundo boot em banco novo: `applied=53`, CPSock escutando e `/health` 200 |

**O que reprovou a condição da opção C — três fatos medidos.**

1. **Clone limpo não tem o peer.** `deprecated/*` está no `.gitignore` e `git ls-files deprecated`
devolve **0 arquivos**: nem o binário nem o código existem em clone limpo. A reprodução vale só na
árvore local, que não é versionada — não é reproduzível por terceiro.
2. **O boot não é o padrão do binário.** Sem `MIGRATIONS_DIR`, o processo encerra com `exit 2` e
`fatal reason=schema: série de migrations ilegível: /migrations` — caminho de container. O boot só
aconteceu com a variável apontada manualmente.
3. **O gate de versão recusa o cliente do repositório.** O binário compila `PROTOCOL_VERSION = 12000`
(`deprecated/crates/wyd-protocol/src/lib.rs:40`) e o boot usa `VersionGate::from_protocol_version()` com
política estrita, sem variável de ambiente que a afrouxe; o log confirma `event=gate
expected_version=12000`. O cliente medido declara 7640/7662, então o handshake seria recusado e A1 não
completaria os dois sentidos. É dedução a partir do gate estrito e das versões declaradas, **não** um
teste com o cliente real — a CLI de captura não existe (`D3`/`D4`).

**Consequência registrada.** A condição da opção C não se cumpriu: **A1 passa a depender da opção A** —
peer do `wyd-server` no passo 0, com o `ECAP` e a CLI. Isso não reabre `LSRV-05`: a decisão continua
registrada em C, com a condição reprovada por medição datada, e B segue recusada. Executar A é trabalho
de implementação, não nova decisão.

## 10. O que esta especificação não decide

- Não implementa `EPROD`, não define sua porta, seu deploy ou sua exposição.
- Não implementa `ECAP`, CLI, binário, crate ou teste.
- Não implementa nem executa o peer de A1; a escolha está registrada em `Q-SRV-5`, com a condição de C
reprovada em 2026-11-23, e o peer de A1 passa a ser o da opção A.
- Não afirma `KeyWord`, encoding, terminador, opcode, frame ou comportamento do servidor.
- Não define o adaptador do serverlist (implementação de `LCLI-04`) nem o nome do QA (`LCLI-03`); o build-alvo está
  decidido em `LPRT-09` (2026-11-23).
- Não fecha `LPRT-03` ou `LPRT-06`; A1 produz a evidência que a segunda leitura e o golden usarão e
  registra a build de `LPRT-09`.
- Não cria issue, ADR, label, milestone, commit ou publicação.
- Não altera D3–D6: D3 continua sendo a decisão de princípio do proxy gravador local, D4 a CLI Rust,
  D5 o bruto fora do repositório e D6 a segunda leitura.

## 11. Rastreabilidade

| Item | Onde está | O que esta spec usa |
|---|---|---|
| decisão do proxy local | `wyd-client/lacunas.md` D3 | `ECAP` é a forma técnica do proxy gravador; não reabre D3 |
| CLI Rust | D4; `LCLI-04` | §6 |
| bruto fora do repositório e retenção | D5 | §4.4 |
| segunda leitura | D6; `LPRT-10`; `LCLI-03` | §7 e §8 |
| procedimento de captura | `wyd-protocol/docs/goldens/como-capturar.md` | ficha, anonimização, segunda leitura |
| A1 | `esqueleto-de-ponta-a-ponta.md` §4 | cenário e prova |
| endpoint de produção | `X7`; passo 0 | `EPROD`, não confundir com `ECAP` |
| decisão do peer | `proposta-decisoes-server.md` `Q-SRV-5`; `lacunas.md` `LSRV-05` | §9 |
