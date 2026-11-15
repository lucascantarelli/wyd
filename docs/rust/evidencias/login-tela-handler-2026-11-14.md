# Tela de login do 7662: o que a recusa desenha, com texto literal

Data: 2026-11-14. Escopo: estabelecer **qual frame a etapa de login do cliente desenha e com que
texto**, e provar a cadeia do frame até a chamada de desenho. Nada de loja/NPC, combate, criação ou
nível.

**Resultado em uma linha:** a etapa de login só desenha `0x011C`/`0x011D`, e o texto é o registro
**12** de `client/UI/strdef.bin` (offset **0x600**, cp1252, NUL-padded):
**"Conexão anterior finalizada. Tente novamente."** O servidor emite **um** `0x011C` header-only
(12 B), sem roster e sem `CNFAccountLogin`.

> **Tipo de evidência:** **estática** (desassemble, cabeçalho PE, arquivo de tabela) + **bytes
> vivos** do canal + **captura do cliente real**. O texto desenhado foi lido da imagem e confere
> caractere a caractere com o registro 12: `target/login-recusa-1.jpg` (recorte ampliado em
> `target/login-recusa-texto.png`), com o cliente em LOGIN contra o canal Rust vivo, conta
> `cenamob1` recusada por senha (`event=login result=refused`). A ligação entre a imagem e o id
> `0x109aab8` continua sendo a cadeia estática abaixo: a tela de login só desenha texto por
> `0x011C`/`0x011D`, e os dois braços empurram o **mesmo** id.

## Método

```bash
objdump -d -M intel client/WYD.exe > tmp/wyd.asm   # 561.977 linhas, 30 MB
node tools/client/strdef.mjs                        # resolve id → literal
```

## Etapa de login — função `0x4b3e83` (termina em `0x4b412b`, `ret 0x8`)

| endereço | comparação | efeito |
|---|---|---|
| `0x4b3ec1` | `cmp ecx,0x101` | `call [obj+0x26e7c][+0x60] com (1)`, depois `[+0x2c]==0` → `call [+0x64] com (1)` — **só visibilidade**, não chama `0x404109` |
| `0x4b3f45` | `cmp edx,0x10a` | copia `0x348` B de `frame+0x20`, `0x400` B de `frame+0x368`, `push 5` + `[+0x50]` |
| `0x4b4071` | `cmp ecx,0x11d` (`0x011D`) | `push 0xfa0; push 0x109aab8; call 0x404109` + `[+0x88] com (1,1)` |
| `0x4b4082` | `cmp eax,0x11c` (`0x011C`) | idem, **mesmo id `0x109aab8`** |
| `0x4b4126` | — | `xor eax,eax` → **opcode ignorado** |

Não existe `cmp ...,0x119` nem `cmp ...,0x11b` nessa função. A etapa de seleção (função de
`0x4aea52`) é que desenha `0x011B` (`0x4aea5b`, id `0x109aeb8`) e `0x0119` (`0x4aed63`, id
`0x109af38`).

## Cadeia do frame até a chamada de desenho

```
4b4082  cmp eax,0x11c            ; Type@4 do frame recebido
4b4087  jne 0x4b4126             ; opcode fora da etapa: ignorado
4b40b7  push 0xfa0               ; 4000 ms
4b40bc  push 0x109aab8           ; id de mensagem (constante)
4b40c1  mov eax,[ebp-0x14]       ; janela
4b40c4  mov ecx,[eax+0xc4]       ; objeto de UI
4b40ca  call 0x404109            ; rotina de mensagem
 404113  mov ecx,[ebp+0xc]       ; duração
 404116  mov [eax+0x1f8],ecx
 40412d  mov ecx,[ebp-0x4]
 404130  mov [ecx+0x1f4],eax     ; tick inicial
 404136  push 0x0
 404138  push [ebp+0x8]          ; o id 0x109aab8
 40413f  mov ecx,[eax+0x1e4]     ; controle de texto
 404150  call [edx+0x80]         ; desenho com (id, 0)
```

O ponteiro do frame **não** é passado em nenhum ponto: os argumentos são constantes. O texto vem
da tabela do binário, nunca do payload.

## De onde vem o texto: `client/UI/strdef.bin`

O id é um **ponteiro para registro** numa tabela de mensagens:

- base `0x109a4b8`, registros de **128 B**, `2000` registros (`0x3e800` B);
- `0x109aab8 − 0x109a4b8 = 0x600` → **registro 12**, **offset de arquivo `0x600`**;
- inicializador de leitura: `0x53a837` — `fopen("UI/strdef.bin","rb")` (strings em `0x622234`/
  `0x622238`, no arquivo em `0x222234`/`0x222238`), `fread(0x109a4b8, 1, 0x3e800, f)` em
  `0x53a890`, checksum `0x53a55b` comparado em `0x53a8cf`, decodificação `xor 0x5a` em
  `0x53a8f5–0x53a904`;
- contraparte de escrita (dump/regeneração): `0x53a758`, que usa `fopen(...,"wb")` com o mesmo
  nome e o mesmo tamanho `0x3e800` (`0x53a7c2`).

Arquivo no repositório: `client/UI/strdef.bin` (**255.992 B** = 1.999,9 registros; a última
entrada é truncada, o que casa com o `fread` curto). Texto em **cp1252**, NUL-padded:

| id | registro | offset | literal (cp1252) |
|---|---|---|---|
| `0x109aab8` (`0x011C`/`0x011D`) | 12 | `0x600` | **"Conexão anterior finalizada. Tente novamente."** |
| `0x109aeb8` (`0x011B`, exclusão) | 20 | `0xa00` | "Falha ao apagar o personagem." |
| `0x109af38` (`0x0119`, login de personagem) | 21 | `0xa80` | "Falha ao logar o personagem." |

Hex do registro 12: `436f6e6578e36f20616e746572696f722066696e616c697a…` (`Conex` + `e3` = `ã`).

**Validação cruzada:** o registro 20 é exatamente a frase que o jogador confirma ver quando erra a
senha ao apagar personagem — o que fecha a conta entre id, registro, arquivo e codificação sem
depender de suposição.

Reprodutível no repo: `node tools/client/strdef.mjs` (imprime os três ids com VA, índice, offset,
hex e literal) ou `node tools/client/strdef.mjs --index 12`.

## Por que o ponteiro não está no executável

Cabeçalho PE de `client/WYD.exe` (2.347.008 B = `0x23d000`): `ImageBase=0x400000`; `.data` tem
vaddr `0x20f000`, **vsize `0x2172351`**, raw `0x28000`. `0x109a4b8 − 0x400000 = 0xc9a4b8` cai no
trecho **virtual** de `.data` — é a tabela de 256.000 B que o inicializador acima preenche a partir
de `UI/strdef.bin` em tempo de execução. Não há string embutida a extrair do `.exe`; a origem é o
arquivo de dados, já extraído.

## Forma emitida pelo servidor (e a variante retratada)

`crates/wyd-server/src/cpsock/handshake.rs`, função de falha de credencial: **um**
`_MSG_AlreadyPlaying` (`0x011C`) header-only, 12 B. Sem `CNFAccountLogin`, sem roster, sem
`0x0119`, sem painel. O que o jogador vê é o literal do registro 12 acima, por 4 s.

**Retratação:** a rodada anterior enviou `CNFAccountLogin` de **roster vazio** antes de `0x0119` +
painel, medido como `s2c size=2008 hex=d8070b690a01 3275 00000000 …`. Rejeitada: o `0x010A` move
a tela para a etapa de seleção, tirando o cliente da tela de login. O servidor não emite mais isso.

## Frames reais do canal vivo

Canal `127.0.0.1:8281`, imagem reconstruída; despejo em `tmp/login-canal/recusas.txt`:

```
caso=senha_errada      c2s size=116 (AccountLogin 0x0BFF)
                       s2c size=12 hex=0c000bac1c01010000000000
caso=conta_bloqueada   s2c size=12 hex=0c000bac1c01010000000000
caso=conta_inexistente s2c size=12 hex=0c000bac1c01010000000000
```

`Type@4 = 0x011c`, 12 B, sem corpo; nenhum frame de painel (`0x0101`) e nenhum `0x010A` no
arquivo. Banco: contas do caminho de senha errada com **0 sessões e 0 personagens**.

## Tentativas de execução do cliente real (2026-11-14 22:50 → 23:05, elevadas)

### 1) Harness original (22:50)

```
powershell -NoProfile -ExecutionPolicy Bypass -File tools/client/captura-sessao.ps1
```

- `elevacao: True`; `WYD CDK` abriu, janela **806x629** em `0,0` (o script assume 1030x797);
- screenshots `target/etapa1-menu.png` … `target/etapa5-login.png`: **lista de servidores** com
  `WYD` preenchida e a coluna **Canal vazia**;
- `conexões ESTABELECIDAS na porta 8281: 0`; listener do harness `0 conexão(ões)`;
- `client/WYD.log`: `VertexShader HW Accel Enabled` e `Error in Init Render Target Texture`.

**Causa medida:** o harness subia um segundo listener em `0.0.0.0:80`
(`captura-live.mjs --port 80`) e o cliente consulta `http://192.168.100.3/serv00.htm` antes de
preencher a coluna de canais; o canal Rust responde certo (`200`, corpo `0\n0\n` = dois canais),
mas o listener do harness segurava a requisição esperando bytes de CPSock.

### 2) Harness corrigido: `-SemListeners` + screenshot pós-Enter (22:55 e 23:02)

Ajustes no harness (só ferramenta, nada do cliente nem do servidor):

```
# porta/rota: não subir listener local quando o servidor real deve responder
[switch]$SemListeners          # pula captura-live.mjs em :8281 e em :80
[double]$EsperaMensagem = 1.8  # screenshot logo após o Enter (a recusa dura ~4 s, 0xfa0)
function CliqueDuplo { ... }   # cliente D3D9 ignora o primeiro clique na lista
```

Resultado medido:

- a coluna **Canal passou a listar `WYD-1` e `WYD-2`** — confirma que o listener de `:80` era a
  causa da coluna vazia (`target/etapa3-canal.jpg`);
- `Conectar` foi clicado e **nenhum TCP foi aberto**: `conexões ESTABELECIDAS na porta 8281: 0` e
  `docker compose logs server-canal-1` sem `event=game`/`handshake` nessas janelas;
- o screenshot do harness (`PrintWindow`) saiu **preto**; a captura do desktop em paralelo
  (`tools/client/captura-tela.ps1`, `CopyFromScreen` a cada 700 ms, `target/tela/*.jpg`) mostra o
  que está de fato na tela: `tela-0095.jpg` e `tela-0102.jpg` são a **lista de canais** com
  `WYD-1`/`WYD-2` e o botão `Conectar` — ou seja, o clique não saiu da lista e o login não abriu.

### 3) Clique duplo com `mouse_event` (23:04)

Mesma medição: lista de canais na tela, `conexões ESTABELECIDAS na porta 8281: 0`, sem
`event=game` no log do canal. O clique simples/duplo de `captura-sessao.ps1`
(`SetCursorPos` + `mouse_event`) **não chega ao cliente**.

### 4) Captura obtida: `SendInput` absoluto + teclado por scan code (23:15 e 23:17)

O caminho de entrada foi medido e agora vive no harness `tools/client/captura-sessao.ps1`
(`-SemListeners -Conta <login> -Senha <senha>`); não altera cliente nem servidor. O que foi medido,
com o cliente já aberto e elevado:

```
tela: 1366x768
dpi=96  janela rect=0,0 806x629
foreground antes=9307328 (já era o cliente)  SetForegroundWindow=True  depois=9307328
SetCursorPos(465,318)=True  cursor lido=(465,318) exato=True
SendInput clique servidor/canal/Conectar enviados=1
```

Prova de que o caminho antigo não entregava o clique e o novo entrega:

| passo | arquivo | o que mostra |
|---|---|---|
| lista inicial | `target/diag-01-antes.jpg` | coluna Canal **vazia** |
| após clique SendInput no servidor | `target/diag-02-servidor.jpg` | `WYD-1` e `WYD-2` listados |
| após clique SendInput em `Conectar` | `target/diag-05-conectar.jpg` | **tela de Login** aberta |
| clique em `Fechar` com `mouse_event` | — | cliente **não** encerrou (input antigo não chegava) |

Sequência que funciona (12 B de diferença: `SendInput` com `MOUSEEVENTF_ABSOLUTE` em vez de
`mouse_event`, e `KEYEVENTF_SCANCODE` para teclado):

1. clique no servidor `WYD` (330,318) → coluna Canal popula;
2. clique no canal `WYD-1` (465,318);
3. clique em `Conectar` (330,504) → tela de **Login**.

Login submetido por scan code: `cenamob1` (conta existente, 8 chars) + TAB + `errada999` + ENTER.
Capturas no instante da mensagem (~4 s de vida): `target/login-recusa-1.jpg` (1,2 s depois do
ENTER) e `target/login-recusa-2.jpg` (3,0 s); formulário preenchido em `target/login-preenchido.jpg`;
recorte 4x do texto em `target/login-recusa-texto.png`.

**Texto lido da imagem** (recorte `target/login-recusa-texto.png`):

```
Conexão anterior finalizada. Tente novamente.
```

Comparação com o registro extraído do binário:

```
node tools/client/strdef.mjs --index 12
índice=12 file_off=0x600 cp1252="Conexão anterior finalizada. Tente novamente."
```

Conferem **caractere a caractere** (mesma pontuação, mesmos acentos `ã`, mesma caixa). O servidor
registrou a recusa desse login real:

```
event=login result=refused account=cenamob1 reason=senha de login não confere
event=handshake result=rejected reason=autenticação: senha de login não confere
conexoes ESTABELECIDAS 8281 no login: 1
```

Segunda captura, **conta inexistente** (`qxnova1122` + `errada999`, 23:22):

```
target/login-inexistente-recusa-1.jpg   -> "Conexão anterior finalizada. Tente novamente."
target/login-inexistente-texto.png      (recorte 4x, mesma leitura)
event=login result=refused account=qxnova1122 reason=conta inexistente
conexoes ESTABELECIDAS 8281 no login: 1
```

O caminho de entrada foi depois incorporado ao harness (`mouse_event` removido; `SendInput` absoluto +
scan code em todas as etapas) e reexecutado com o mesmo caso (23:36):

```
powershell -File tools/client/captura-sessao.ps1 -SemListeners -Conta cenamob1 -Senha errada999 -Prefixo login-unificado
foreground=5048782 igual_ao_cliente=True
conexoes ESTABELECIDAS na porta 8281 no login: 1
target/login-unificado-recusa-1.jpg -> "Conexão anterior finalizada. Tente novamente."
event=login result=refused account=cenamob1 reason=senha de login não confere   (23:37)
cenamob1 chars=2 sess=0
```

Achado colateral medido na primeira tentativa dessa rodada (senha de 1 caractere): o **cliente**
recusa sozinho, desenhando `Senha muito curta.`, **sem abrir TCP** (`conexões ESTABELECIDAS 8281 no
login: 0`). Ou seja, existe validação local de tamanho antes do login sair do cliente; essa captura
foi sobrescrita pela rodada final com senha válida, o número medido é o que fica.

Banco: `cenamob1 chars=2 sess=0` (nenhuma sessão aberta pela recusa). Observação de escopo: os
braços `0x011C` e `0x011D` empurram o **mesmo** id `0x109aab8`, então o literal é idêntico nos dois;
o servidor emite `0x011C` (hex na seção anterior) e o texto na tela é o do registro 12.

### Controles de viabilidade (o bloqueio não é rede nem porta)

```
curl http://192.168.100.3/serv00.htm      -> 200, corpo "0\n0\n"
TCP 192.168.100.3:8281 do Windows         -> TCP_OK
netstat -ano | grep :8281                 -> LISTENING (docker)
```

**Conclusão deste turno:** a prova por captura da tela de LOGIN do cliente real **foi obtida**. O
bloqueio não era rede, porta, foreground nem DPI (todos medidos acima): era a forma de injeção —
`SetCursorPos`+`mouse_event` não chega ao cliente, `SendInput` absoluto chega. Com a captura
`target/login-recusa-1.jpg` lida e comparada, o texto desenhado é o registro 12 do `strdef.bin`. A
ev Idência estática continua valendo onde sempre valeu: a cadeia `0x4b4082 cmp eax,0x11c` →
`push 0x109aab8` → `0x404109`/`0x404150` e o ponteiro de registro.

## Modelo derivado (não é prova)

`crates/wyd-bots/src/tela/router.rs` reproduz as duas etapas para regredir a forma de fio; é
**modelo derivado**, não prova do cliente. A prova é o assembly e a tabela acima.
