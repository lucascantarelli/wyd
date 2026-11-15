# Autenticação: recusa visível no 7662 e PIN conferido no banco (2026-11-15)

Escopo: **só a perna de autenticação** do relato de 2026-11-14 23:41. Nada de loja, NPC,
combate ou nível foi tocado nesta rodada. Toda afirmação abaixo vem de medição nesta máquina:
bytes do canal real (`127.0.0.1:8281`), estado do Postgres do compose e captura do cliente
7662 com leitura automática do texto.

**Resultado em uma linha:** senha errada, conta bloqueada e conta inexistente devolvem **um**
`_MSG_AlreadyPlaying` (`0x011C`, 12 B) e o cliente 7662 **desenha** o texto do registro 12
(`semelhança 0.9767`, leitura automática); PIN errado devolve `0x0FDF` (motivo 2) + painel e
**não grava** personagem nem reescreve `pin_hash`; PIN correto passa e o canal abre sessão.

## 1. Forma do fio (HEADER de 12 B: `Size@0` u16, `Key@2` u8, `Checksum@3` u8, `Type@4` u16, `ID@6` u16, `Tick@8` u32)

Canal 1 vivo, imagem em serviço criada em `2026-11-15T01:34:44Z`, `docker inspect` →
`status=running health=healthy`, `/health` → `{"ok":true,"database":"up","channels":2,…,`
`"schema":52}`. Nenhuma fonte de `crates/wyd-server`/`crates/wyd-protocol` é mais nova que a
imagem (`find … -newermt` vazio) — **não houve rebuild** nesta rodada e o binário que respondeu
é o do código atual.

Dumps crus em `tmp/login-canal/*.txt` (gravados pelos próprios testes):

```
caso=senha_errada conta=qasenha23830400
c2s size=116 hex=74000bff2be7f5fd07edccf9a5568665…     _MSG_AccountLogin
s2c size=12  hex=0c000bac1c01010000000000
     Size@0=0x000c=12  Key@2=0x0b  Checksum@3=0xac  Type@4=0x011c  ID@6=1  Tick@8=0

caso=conta_bloqueada conta=qasenha23830400        (senha CORRETA numa conta banida)
s2c size=12  hex=0c000bac1c01010000000000         (bytes idênticos)

caso=conta_inexistente conta=qanada64007500
s2c size=12  hex=0c000bac1c01010000000000         (bytes idênticos — não revela existência)
```

Os três caminhos de falha de credencial emitem **o mesmo frame**, header-only, sem
`CNFAccountLogin`, sem roster e sem painel `0x0101`. É o único opcode que a etapa de login do
binário compara e desenha (`0x4b4082` → `push 0xfa0; push 0x109aab8; call 0x404109`, ver
[`login-tela-handler-2026-11-14.md`](login-tela-handler-2026-11-14.md)).

PIN, no mesmo canal:

```
caso=pin_errado conta=qapin86895000
c2s size=32  hex=20000b31fcf4f5fd08edccf96221482dbaf1f4fdfae4f8fcf2e40cfb1cf1f8fd
s2c size=13  hex=0d000bdedf0f01000000000002
     Size@0=13  Type@4=0x0fdf (_MSG_AccountSecureFail)  ID@6=1  motivo@12=0x02 (PinIncorreto)
s2c size=140 hex=8c000b5a01013075 00000000 50494e20696e636f727265746f2e2054656e7465206e6f76616d656e74652e00…
     Size@0=140 Type@4=0x0101 ID@6=0x7530 (30000=ESCENE_FIELD) texto@12="PIN incorreto. Tente novamente."
roster_depois=0

caso=pin_correto conta=qapin86895000
c2s size=32  hex=20000b31fcf4f5fd08edccf964254e35baf1f4fdfae4f8fcf2e40cfb1cf1f8fd
s2c size=12  hex=0c000bacde0f010000000000      Type@4=0x0fde (_MSG_AccountSecure)
s2c size=856 hex=58030bbb10013175…            Type@4=0x0110 (CNFNewCharacter), Level@96=0
personagem_criado=Pcpin868

caso=selecao_sem_pin conta=qapin86895000        (PersonagemLogin antes do PIN)
c2s size=36  hex=24000b112de7f5fd09edccf934f118fd0a616466681c2e34f2e40cfb1cf1f8fd5ef194fd
s2c size=13  hex=0d000bdedf0f01000000000002   + banco sem `Pppin868`
```

## 2. Estado do banco, medido com `psql` no mesmo instante

```sql
select a.login,
       (select m.blocked from account_moderation m where m.account_id = a.id) as bloqueada,
       (a.pin_hash is not null) as tem_pin,
       (select count(*) from character_session s where s.account_login = a.login) as sessoes,
       (select count(*) from characters c where c.account_id = a.id and c.deleted_at is null) as personagens
from accounts a where a.login like 'qasenha%' or a.login like 'qanada%' or a.login like 'qapin%';
```

```
      login      | bloqueada | tem_pin | sessoes | personagens
-----------------+-----------+---------+---------+-------------
 qapin86895000   |           | t       |       2 |           1
 qasenha23830400 | f         | f       |       0 |           0
 qapin28641000   |           | t       |       2 |           1
 qasenha18930300 | f         | f       |       0 |           0
```

`qanada64007500` **não existe** na tabela (o caminho de conta inexistente não cria nada). O
caminho de senha errada/bloqueada fica com **0 sessões e 0 personagens**; o caminho que passou
pelo PIN fica com sessão e personagem.

### PIN antes/depois com um cliente de protocolo (conta `qapinraw0001`, PIN `2468`)

```
antes:             pin_md5=353e0eadad647f19ff1e3db6df4b3c36  pin_failures=0  personagens=0  sessoes_abertas=0
bot --pin 0000  -> TransKnight: PIN incorreto. Digite os 6 dígitos do PIN e tente novamente.  (exit 1)
depois do errado:  pin_md5=353e0eadad647f19ff1e3db6df4b3c36  pin_failures=1  personagens=0  sessoes_abertas=0
bot --pin 2468  -> TransKnight: Personagem indisponível nesse slot…  (exit 1)
depois do certo:   pin_md5=353e0eadad647f19ff1e3db6df4b3c36  pin_failures=0  personagens=0
```

Leitura honesta do par acima: PIN errado **não** mexe em `pin_hash` (md5 idêntico), **não**
cria personagem e **não** abre sessão — a única escrita é `pin_failures`, que existe justamente
para o bloqueio por tentativas. PIN correto zera `pin_failures` e o canal segue; a recusa
seguinte é de personagem, não de PIN. Log do canal na mesma conexão:

```
event=login result=password_ok account=qapinraw0001 version=12000 channels=2
event=pin    result=ok account=qapinraw0001 mode=Verified change=0
event=session result=open account=qapinraw0001 group=1 channel=1 id=622
event=character result=refused action=select account=qapinraw0001 reason=personagem inexistente para esta conta
```

**Defeito de harness corrigido nesta rodada:** o bot de QA mandava `change_numeric = 1`
sempre, e `1` é o **passo 1 da troca de PIN** (arma a troca e fica esperando o passo 2, sem
abrir sessão). Com conta que já tinha PIN, o bot era recusado mesmo digitando o PIN certo
(`event=pin result=refused … reason=selecao_antes_do_pin opcode=_MSG_CharacterLogin`, medido).
Agora manda `0` (login normal); `crates/wyd-bots/src/bot/mod.rs`. Nenhum teste dependia do
valor antigo: todos os outros pontos usam `change_numeric: 0`.

## 3. Cliente 7662 real: o texto aparece (leitura automática, sem olho humano)

Harness `tools/client/captura-sessao.ps1 -SemListeners`, executado **elevado** (o `WYD.exe`
declara `requireAdministrator`), janela 806x629 em `0,0`, DPI 96.

### 3.1 Senha errada (`cenamob1` / `errada999`), 2026-11-15 00:34 local

```
clique em Conectar → conexoes ESTABELECIDAS na porta 8281 no login: 1
Enter no login → captura em 1,2 s
ocr target/login-senha-errada-recusa-1.png -> exit=0
{"ok":true,"strdef_indice":12,"strdef_offset":"0x600",
 "esperado":"Conexão anterior finalizada. Tente novamente.",
 "lido":"WYD CDK Conexao anterior finalizada. Ten e novamente.",
 "normalizado_lido":"wyd cdk conexao anterior finalizada ten e novamente",
 "semelhanca":0.9767,"minimo":0.85}
veredito_ocr=confere strdef=12 semelhanca=0.9767
```

Canal, na mesma conexão (uma sessão TCP do cliente):

```
event=login result=refused account=cenamob1 reason=senha de login não confere
event=handshake result=rejected reason=autenticação: senha de login não confere
```

`client/WYD.log` dessa tentativa (copiado para `target/WYD-senha-errada.log`, CRLF preservado):

```
--------------------------------------------------------------------------
>>> Logging System turn on.
>>> mark
11/15 00:34:35 - VertexShader HW Accel Enabled
11/15 00:34:35 - Error in Init Render Target Texture
```

### 3.2 Conta banida com senha CORRETA (`qabanido0001` / `qa12345678`), 00:36 local

Conta criada copiando o `password_hash` de `qapin86895000` e bloqueada em
`account_moderation` (`blocked=true`, motivo `teste de canal: conta banida`):

```
ocr target/login-conta-banida-recusa-1.png -> exit=0
{"ok":true,"strdef_indice":12,"semelhanca":0.9767,
 "lido":"WYD CDK Conexao anterior finalizada. Ten e novamente."}
veredito_ocr=confere strdef=12 semelhanca=0.9767
event=login result=refused account=qabanido0001 reason=conta bloqueada pela moderação
event=handshake result=rejected reason=autenticação: conta bloqueada pela moderação
```

Conta desbanida ao fim (`delete from account_moderation …`).

### 3.3 Janela de vida do texto (medido, não estimado)

A segunda captura da mesma rodada, ~1,9 s depois da primeira (≈3,1 s após o Enter), **não**
mostra a mensagem: `semelhanca=0.2791` nas duas contas. O texto está na tela em 1,2 s e já
saiu em 3,1 s; o `push 0xfa0` (4000 ms) é o pedido do binário, não a duração observada.

## 4. Comparação automática com o `strdef.bin` (novo)

Antes o texto era lido à mão no recorte. Agora:

- `tools/client/strdef.mjs --index 12 --json` → `{"index":12,"offset":1536,`
  `"texto":"Conexão anterior finalizada. Tente novamente."}` (offset de arquivo `0x600`);
- `tools/client/ocr-texto.ps1` faz OCR offline (`Windows.Media.Ocr`, motor do Windows, sem
  rede), em **fatias horizontais sobrepostas** (o motor junta a faixa de mensagem com a arte de
  fundo quando recebe a janela inteira), amplia 3x, normaliza acento/caixa/pontuação e compara
  por distância de edição com o literal do registro; `exit 0` confere, `1` divergiu, `2` sem
  OCR na máquina;
- `captura-sessao.ps1` chama esse utilitário nas duas capturas e escreve no log a linha
  `veredito_ocr=confere|divergente`, além dos JSONs `target/<prefixo>-ocr-{1,2}.json`.

Controle negativo (prova de que a comparação discrimina, não "aprova qualquer coisa") — mesma
captura comparada com o registro **20** (`"Falha ao apagar o personagem."`):

```
{"ok":false,"strdef_indice":20,"strdef_offset":"0xa00","semelhanca":0.2857}  exit=1
```

## 5. De onde vinha a frase: **do cliente**, não do servidor

A tela de login do 7662 não aceita texto do servidor: os braços `0x011C` e `0x011D` empurram
uma constante (`0x109aab8` = registro 12) e **não** passam o frame para a rotina de desenho
(`0x404109`/`0x404150`). Até a medição de §1–4 o registro 12 dizia
`"Conexão anterior finalizada. Tente novamente."` — literal de `client/UI/strdef.bin`, não
mensagem nossa. O servidor responde (antes o cliente ficava mudo) e a recusa vale para os três
caminhos; a **palavra** foi corrigida no registro 12 em §7.

## 6. Gates

| Gate | Resultado |
|---|---|
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test --workspace` | ver §6.1 |
| `node tools/dev/check-docs-consistency.mjs` | 0 — `61 issues, 9 crates, 10 membros, 52 migrations, 275 docs, 417 links` |
| `sistema_login_canal` (canal vivo) | 1/1 — seis casos da §1 |
| `sistema_painel_login_canal` (canal vivo) | 1/1 |
| `sistema_criacao_canal` (canal vivo) | 1/1 |
| `docker inspect wyd-server-canal-1-1` | `created=2026-11-15T01:34:44Z status=running health=healthy` |
| `node tools/client/strdef-patch.mjs --index 12 --texto "…" --dry-run` | `bytes_alterados=0`, `decodifica_xor=false`, exit 0 (idempotente) |

### 6.1 Suíte do workspace

`cargo test --workspace` (saída completa em `tmp/test-workspace-2026-11-15.log`):

```
passed=869  failed=0  ignored=1  (91 executáveis de teste)
TESTS_EXIT=0
```

Nenhuma linha `FAILED` no log. Os testes de canal/banco (`sistema_login_canal`,
`sistema_painel_login_canal`, `sistema_criacao_canal`) são os que mediram os bytes das §1–2,
rodados contra o canal e o Postgres do compose.

## 7. Registro 12 reescrito: **"Usuário ou senha incorretos."**

### 7.1 Formato e checksum, medidos no próprio binário

Carregador `0x53a837` (desassemble endereçado em `client/WYD.exe`):

```
53a83d  push 0x3e800 / push 0 / push 0x109a4b8 / call 0x5c5b80   ; memset(buf, 0, 256000)
53a866  fopen("UI/strdef.bin","rb")                             ; strings em 0x622234/0x622238
53a890  fread(buf, 1, 0x3e800, f)                               ; dados
53a8a4  fread(&valor, 1, 4, f)                                  ; checksum declarado, no fim do arquivo
53a8c1  computed = 0x53a55b(buf, 0x3e800)                       ; rotina de checksum
53a8cf  cmp eax,[ebp-0x14]  / je ok  / xor eax,eax  / ret        ; divergiu -> retorna 0 SEM decodificar
53a8f5  xor 0x5a em 0x3e800 bytes                                ; só no caminho que confere
```

O checksum (`0x53a55b`) acumula em 32 bits com uma operação por `índice % 7`, sobre **byte com
sinal** (`movsx`): `+v/2` , `+(v xor 0xff)` , `+3v` , `+2v` , `-v/7` , `-v` , `+v/3`. Replicado em
`tools/client/strdef-patch.mjs` (`checksum()`), que confere os números abaixo.

Quem chama o carregador (`0x5494ea`) **ignora o retorno** — a instrução seguinte é
`push 0x622cd4; call 0x438310` — e o gravador `0x53a758` (que re-codifica e anexa o checksum)
**não tem chamador** no binário. Não existe caminho que regrave o arquivo.

Estado medido do arquivo em `client/`:

```
tamanho            255.992 B  (0x3e7f8 — 8 B menos que 0x3e800)
checksum declarado ausente    (o 2º fread devolve 0 -> `valor` fica 0)
checksum calculado 10.705.530 (antes) -> 10.705.345 (depois do patch)  != 0
resultado          o `xor 0x5a` NUNCA é aplicado: a tabela é lida em texto claro
```

Ou seja: este arquivo **já** ficava com o checksum divergente antes de qualquer alteração nossa,
e o cliente desenha a tabela assim mesmo (provado nas §3–4, com o literal antigo). Reescrever o
registro em cp1252 aparece na tela **sem** atualizar checksum — não porque o cliente "aceita
qualquer coisa", mas porque o caminho de decodificação está desligado por divergência e o
retorno do carregador é descartado.

### 7.2 Bytes antes/depois (offset `0x600`, registro 12)

```
antes : 436f6e6578e36f20616e746572696f722066696e616c697a6164612e2054656e7465206e6f76616d656e74652e
        45 bytes  "Conexão anterior finalizada. Tente novamente."
depois: 557375e172696f206f752073656e686120696e636f727265746f732e0000000000000000…
        28 bytes  "Usuário ou senha incorretos."   (á = 0xE1, cp1252; 28/127 B do slot)
```

Alteração contida: **41 bytes** diferentes, faixa `0x600..0x62c`; o resto da tabela idêntico
(1 registro tocado, 1999 intocados); tamanho inalterado em 255.992 B.

```
md5 antes  ce961eb69c10c4960aa75a1077f380d1
md5 depois 1f0afef7b120f7ceb168a58969e9cd96
backup     tmp/strdef.bin.orig-2026-11-15  (restaurar: cp tmp/strdef.bin.orig-2026-11-15 client/UI/strdef.bin)
```

Reaplicável: `node tools/client/strdef-patch.mjs --index 12 --texto "Usuário ou senha incorretos."
--backup <caminho>` (`--dry-run` mede sem gravar; recusa texto acima de 127 B ou fora do cp1252).

### 7.3 Cliente real, elevado, com leitura automática

O `strdef.mjs` passou a devolver `{"index":12,"offset":1536,"texto":"Usuário ou senha incorretos."}`
e o harness compara a captura com **esse** literal.

```
senha errada (cenamob1 / errada999), 00:58
  ocr ... -> exit=0  semelhanca=0.963
  {"esperado":"Usuário ou senha incorretos.","lido":"WYD CDK Usuario ou penha incorretos,"}
  veredito_ocr=confere strdef=12 semelhanca=0.963
  event=login result=refused account=cenamob1 reason=senha de login não confere

conta banida com senha correta (qabanido0001 / qa12345678), 00:59
  ocr ... -> exit=0  semelhanca=0.963
  {"lido":"WYD CDK Usuario ou penha incorretos,"}
  veredito_ocr=confere strdef=12 semelhanca=0.963
  event=login result=refused account=qabanido0001 reason=conta bloqueada pela moderação

controle negativo (mesma captura contra o registro 20), exit=1
  {"esperado":"Falha ao apagar o personagem.","lido":"Senha Lo in cenamobl Voltar Novo ID","semelhanca":0.2857}
```

`client/WYD.log` das duas tentativas (a mesma assinatura de sempre; nada da recusa vai para o
log do cliente):

```
--------------------------------------------------------------------------
>>> Logging System turn on.
>>> mark
11/15 00:57:25 - VertexShader HW Accel Enabled      (senha errada)
11/15 00:57:25 - Error in Init Render Target Texture

11/15 00:59:01 - VertexShader HW Accel Enabled      (conta banida)
11/15 00:59:01 - Error in Init Render Target Texture
```

Banco depois das duas rodadas (nenhuma sessão/personagem criado):

```
    login      | linhas_sessao | abertas |        ultima_abertura        | personagens
----------------+---------------+---------+-------------------------------+------------
 qabanido0001   |             0 |       0 |                               |           0
 cenamob1       |             6 |       0 | 2026-11-10 13:24:31.041407+00 |           1
```

As 6 linhas de `cenamob1` são históricas (última abertura em 2026-11-10), nenhuma aberta; a
recusa de hoje não abriu sessão.

### 7.4 O cliente continua usando a UI normal

- Etapas da mesma execução, lidas por OCR: lista de servidores, `Servidor`/`Canal`, `WYD-1`/`WYD-2`,
  `Conectar`/`Fechar` (`target/etapa3-canal.png`) e a tela de login completa
  (`Login`, `Senha`, `Voltar`, `Novo ID`, `target/etapa4-conectar.png`).
- Outro registro da mesma tabela foi acionado de propósito: senha de 1 caractere faz o cliente
  desenhar o registro **5** (`"Senha muito curta."`, offset `0x280`), **sem abrir TCP**
  (`conexões ESTABELECIDAS 8281 no login: 0`). OCR padrão (4x/10 faixas) leu
  `"enha mu o cur"` (0.7059 — abaixo do mínimo, a faixa corta o `S`); com 6x/20 faixas leu
  `"enha muío/curta/,"` e **0.8824**, confere. O limite é do OCR em frase curta, não da tabela:
  o texto é o registro 5 do arquivo, desenhado depois do patch.

## 8. O que segue aberto

- O texto certo aparece, mas por ~1,2–3,1 s (medido em §3.3); a duração é do cliente (`push 0xfa0`).
- O registro 12 é o **mesmo** para `0x011C` e `0x011D`; hoje o servidor só emite `0x011C` na
  recusa de credencial (`cpsock/handshake.rs`, único ponto), então a frase nova cobre o uso real.
  Se um dia `0x011C` for usado para "conta em uso", essa frase ficaria errada para o caso —
  aí a saída é outro registro, não outra mensagem no mesmo slot.
- A troca de frase é um **asset local do cliente** (`client/`, fora do git): quem reinstalar o
  cliente 7662 de origem volta a ver a frase antiga até reaplicar o `strdef-patch.mjs`.
- Entrada no mapa ("ao clicar em conectar nada acontece") é outra perna, não tocada aqui.
