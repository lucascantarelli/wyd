# Nível no fio, medido nos bytes crus (2026-11-14)

Pergunta: *o cliente 7662 lê o nível em que offset, e o que este porte escreve lá bate com o
legado? O N+1 que aparece na tela vem do pacote ou de outro ponto da cadeia?*

A medição anterior decodificava com o codec do porte (concordava consigo mesma). Esta rodada
compara **struct por struct** com `Source/Code` (commit `f39aed3`) e lê os bytes crus capturados
com um parser que não importa nada do servidor (`tools/client/nivel-do-legado.mjs`).

## 1. Structs do legado × serialização do porte

Alinhamento natural do MSVC x86 (sem `#pragma pack`), tamanhos calculados campo a campo.

| Struct / pacote | Legado (`Basedef.h`) | Porte (`character/snapshot.rs`) | Veredito |
|---|---|---|---|
| `STRUCT_SCORE` | 48 B; `Level@0`, `Ac@4`, `Damage@8` | 48 B; `Level@0` | **igual** |
| `STRUCT_ITEM` | 8 B (`short` + `stEffect[3]` de 2 B) | 8 B | igual |
| `STRUCT_MOB` | 816 B; `BaseScore@44`, `CurrentScore@92`, `Equip@140`, `Carry[64]@268`, `GuildLevel@800` | 816 B; mesmos offsets | **igual** |
| `STRUCT_SELCHAR` | 840 B; `Score[4]@80` (48/slot), `Equip@272`, `Guild@784`, `Coin@792`, `Exp@808` | 840 B; mesmos offsets | **igual** |
| `0x010A` `MSG_DBCNFAccountLogin` | corpo: `HashKeyTable@0`, `Unknow_28@16`, `sel@20`, `Cargo[128]@860`, `Coin@1884`, `AccountName@1888` | corpo 1996 B; `sel@20`, `Cargo@860`, `Coin@1884`, `AccountName@1888` | **igual** |
| `0x0110` `MSG_CNFNewCharacter` | `_MSG` + `sel` → corpo 840, `sel@0` | corpo **844**, `sel@4` | **diverge do C**; bate com o **cliente**: `docs/cliente/pacotes.md` §"O que o build 7662 realmente trata" mostra `0x0110` copiando 840 B de `frame+0x10` (= corpo+4). O porte segue o binário, não o `.h`. |
| `0x0112` `CNFDeleteCharacter` | idem `0x0110` | idem `0x0110` | mesmo caso |
| `0x0114` `MSG_CNFCharacterLogin` | `short PosX,PosY` + `STRUCT_MOB` (alinhamento 8 → `mob@8`, corpo 1824) | corpo 1820, `mob@4` | **diverge do C**; é o layout que o cliente aceita no login real (nome/HP/EXP desenhados; se `mob` estivesse 4 B adiantado o nome sairia corrido) |
| `0x0364` `MSG_CreateMob` | `Score@136` no frame = 124 no corpo; 232 B | 232 B; `Score@124` | **igual** |
| `0x0336` `UpdateScore` | `STRUCT_SCORE@0` (48 B) | `STRUCT_SCORE@0` | igual |

**Nenhum offset do campo de nível diverge:** `Level` cai em `SELCHAR 80 + 48·slot` (seleção),
`MOB 44/92` (login) e `Score 124` (cena) nas duas fontes. O `sel@4` do `0x0110` é o único desvio
estrutural, e é o que o próprio cliente 7662 espera.

## 2. Parser independente sobre os bytes capturados

`tools/client/nivel-do-legado.mjs` lê `tmp/nivel-wire/*.txt` (uma linha por frame:
`type=… size=… hex=…`) e calcula o offset pelas structs acima — sem `crate`, sem decode do
servidor.

```
$ node tools/client/nivel-do-legado.mjs tmp/nivel-wire/login-slot0.txt --slot=0
arquivo=tmp/nivel-wire/login-slot0.txt slot=0 frames=74
  CNFAccountLogin: Level@frame+100 (frame+112) bytes[00 00 00 00 c7 00 00 00 04 00 00 00] = 199 -> desenhado 200
  CNFCharacterLogin: Level@frame+96 (frame+108) bytes[00 00 00 00 c7 00 00 00 19 00 00 00] = 199 -> desenhado 200
      BaseScore.Level@corpo+48 = 199 -> desenhado 200
```

```
$ node tools/client/nivel-do-legado.mjs tmp/nivel-wire/login-slot1.txt --slot=1
  CNFAccountLogin: Level@frame+148 (frame+160) bytes[00 00 00 00 8e 01 00 00 04 00 00 00] = 398 -> desenhado 399
  CNFCharacterLogin: Level@frame+96 (frame+108) bytes[00 00 00 00 8e 01 00 00 19 00 00 00] = 398 -> desenhado 399
```

```
$ node tools/client/nivel-do-legado.mjs tmp/nivel-wire/criacao-QANIVEL3.txt --slot=3
  CNFAccountLogin: Level@frame+244 (frame+256) bytes[00 00 00 00 00 00 00 00 00 00 00 00] = 0 -> desenhado 1
  CNFNewCharacter: Level@frame+228 (frame+240) bytes[00 00 00 00 00 00 00 00 04 00 00 00] = 0 -> desenhado 1
```

```
$ node tools/client/nivel-do-legado.mjs tmp/nivel-wire/levelup.txt | grep UpdateScore
  UpdateScore: Level@corpo+0 (frame+12) bytes[00 00 00 00 00 00 00 00 19 00 00 00] = 0  -> desenhado 1   (login)
  UpdateScore: Level@corpo+0 (frame+12) bytes[00 00 00 00 01 00 00 00 04 00 00 00] = 1  -> desenhado 2   (level-up)
```

Os quatro bytes antes do `Level` no dump são o `Ac` (4 no personagem novo, 0x19 = 25 no nível
alto), o que confirma que a janela lida é o começo do `STRUCT_SCORE`, não outro campo.

## 3. Medição viva (canal `127.0.0.1:8281`, banco `wyd`, imagem `3e8c2c6f`, criada 17:11 UTC)

```
$ WYD_LIVE_CPSOCK=127.0.0.1:8281 WYD_TEST_REQUIRED=1 \
  WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd \
  cargo test -p wyd-server --test sistema_nivel_wire -- --nocapture
PASS(criacao): QANIVEL2 banco=1 Selchar[180]=0 login[96]=0 CreateMob[124]=0 desenhado=1
PASS(criacao): QANIVEL3 banco=1 Selchar[228]=0 login[96]=0 CreateMob[124]=0 desenhado=1
PASS(login): QANIVEL0 banco=200 Selchar[100]=199 login[96]=199 CreateMob[124]=199 desenhado=200
PASS(login): QANIVEL1 banco=399 Selchar[148]=398 login[96]=398 CreateMob[124]=398 desenhado=399
ALVO Porco id=2119 (2110,2046)
PASS(levelup): banco=2 exp=794 UpdateScore[0]=1 desenhado=2
PASS(save): banco apos logout nivel=2 exp=794
test result: ok. 1 passed; 0 failed; ...
```

Criação: dois personagens criados pelo protocolo (`_MSG_CreateCharacter` → `CNFNewCharacter` +
`CNFCharacterLogin`), banco nível 1, os três pacotes com `Level` 0 no offset do legado.
Login 200 e 399: `Selchar`, `CNFCharacterLogin` e `CreateMob` com 199/398; banco 200/399.
Level-up: 499 de EXP + `Porco` (295) = 794 → nível 2 no banco, `UpdateScore` com 1 no corpo+0.

Log do canal para o mesmo abate:

```
event=death result=ok account=qanivel02 npc=1718 template=Porco exp_base=295 exp_granted=295 levels=1 at=(2110, 2046)
```

## 4. Defeito encontrado no caminho do level-up e corrigido

O log acima mostrou `exp=794` no banco, mas **depois do logout** a linha voltava a `exp=499`
(o valor do login) com `level=2` — estado inconsistente. Causa: `ActorRefresh` (o refresh sem
relogin) levava nível/HP/MP/atributos, mas **não a EXP**; o save de saída (`LiveWorld::leave`)
grava `LiveState.exp`, que continuava o do login. Toda EXP ganha em abate era perdida no logout
(é o mesmo sintoma de "EXP não sobe" relatado).

Correção: `ActorRefresh.exp: Option<i64>` + `ActorRefresh::from_mob` (leva `mob.exp`), aplicado
em `scene::refresh_actor`; o abate agora atualiza o estado vivo **em todo golpe mortal**, não só
quando sobe de nível (`crates/wyd-server/src/character/crud.rs`). Prova: `PASS(save): banco apos
logout nivel=2 exp=794` (antes: 499).

## 5. Onde o N+1 ainda poderia nascer

1. **Binário do canal antigo.** O cliente só recebe pacote errado de um servidor construído antes
   da conversão. Checagem: `docker inspect -f '{{.Image}} {{.Created}}' <container>` e
   `docker image inspect -f '{{.Created}}' <imagem>`; a imagem desta rodada é `3e8c2c6f`
   (17:11 UTC). Imagem anterior a `15:39 UTC` manda `Level` cru.
2. **Linha já deslocada no banco por versão antiga.** `characters.level` gravado como
   `wire + 1` por build antigo continua errado para sempre (o login não recalcula nível).
   Medição: `select name, level, exp from characters where exp = 0 and level > 1;` — nível > 1
   com EXP 0 é sinal de linha herdada (as atuais: `QAA55233724500` nível 5, `QACAMPO01` nível 3).
3. **Tela antiga em cache**: não medido; o cliente 7662 lê o nível do pacote a cada login/seleção.

## 6. Correção deste turno: `CreateMob` de **NPC** também 0-based

O `CreateMob` de jogador já ia `banco − 1`, mas o de **NPC** mandava o nível cru da definição
(`npc.spec.stats.level`, 1-based) no mesmo `STRUCT_SCORE` que o cliente desenha com `+1`. NPC de
nível 30 aparecia como "Nv 31". Corrigido em `crates/wyd-server/src/world/scene.rs`
(`scene_snapshot`), com teste novo em `world::create_mob_manda_nivel_zero_based_para_npc_e_jogador`:

```
cargo test -p wyd-server --lib world::
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 64 filtered out
```

O teste monta `NpcSpec` nível 30 e personagem de banco nível 10, tira `engine.scene_snapshot()` e
exige `CreateMob` com 29 e 9 — a definição/o banco seguem 1-based, só o fio é 0-based.

### 6.1 Repetição da medição viva (mesmo turno, canal real)

```
$ WYD_LIVE_CPSOCK=127.0.0.1:8281 WYD_TEST_REQUIRED=1 \
  WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd \
  cargo test -p wyd-server --test sistema_nivel_wire -- --nocapture
PASS(criacao): QANIVEL2 banco=1 Selchar[180]=0 login[96]=0 CreateMob[124]=0 desenhado=1
PASS(criacao): QANIVEL3 banco=1 Selchar[228]=0 login[96]=0 CreateMob[124]=0 desenhado=1
PASS(login): QANIVEL0 banco=200 Selchar[100]=199 login[96]=199 CreateMob[124]=199 desenhado=200
PASS(login): QANIVEL1 banco=399 Selchar[148]=398 login[96]=398 CreateMob[124]=398 desenhado=399
PASS(levelup): banco=2 exp=1089 UpdateScore[0]=1 desenhado=2
PASS(save): banco apos logout nivel=2 exp=1089
```

Bytes do dump desta rodada (`tmp/nivel-wire/`, offset do frame = corpo + 12):

| pacote | personagem | offset | bytes | valor |
|---|---|---|---:|
| `0x0114` `CNFCharacterLogin` (1832 B) | QANIVEL2 (banco 1) | 108 | `00000000` | 0 |
| `0x0364` `CreateMob` do próprio (232 B) | QANIVEL2 | 136 | `00000000` | 0 |
| `0x010a` `CNFAccountLogin` (2008 B) | slot 2 | 180 | `00000000` | 0 |
| `0x0114` | QANIVEL0 (banco 200) | 108 | `c7000000` | 199 |
| `0x0364` | QANIVEL0 | 136 | `c7000000` | 199 |
| `0x0114` | QANIVEL1 (banco 399) | 108 | `8e010000` | 398 |
| `0x0364` | QANIVEL1 | 136 | `8e010000` | 398 |
| `0x010a` | slot 1 | 160 | `8e010000` | 398 |

Banco no mesmo instante (`characters.level`): `QANIVEL2 = 1`, `QANIVEL3 = 1`, `QANIVEL0 = 200`,
`QANIVEL1 = 399`, `QANIVELUP = 2`. Ou seja: **nenhum** pacote carrega o nível cru do banco, e a
faixa 200/399 não estoura nem inverte sinal.

### 6.2 Antes × depois do `CreateMob` de NPC, no fio (canal real, imagem redeployada)

Mesmo login (`login-slot1.txt`), mesmo NPC, byte `Score.Level` em frame+136:

```
antes  Unicornio Puro  nivel_no_fio=3
depois Unicornio Puro  nivel_no_fio=2
antes  Porco           nivel_no_fio=3
depois Porco           nivel_no_fio=2
depois Chefe de Treino nivel_no_fio=2
```

Ou seja: o NPC de nível 3 da definição ia **cru** (3) e agora vai **2**; o jogador continua
`banco 1 → fio 0`, `banco 200 → fio 199`, `banco 399 → fio 398` nas mesmas PASS acima, medidas
com a imagem nova (`docker inspect -f '{{.Created}}' wyd-server-canal-1-1` → `21:17Z`).

## 7. NÃO verificado

- **Tela do cliente 7662**: nenhuma captura visual nesta rodada; a prova é o byte no offset.
- **`0x0114` com `mob@8`** (alinhamento do C): o porte usa `mob@4` e o login funciona no cliente
  real, mas não há desassemble do `0x0114` para citar o offset que o binário lê.
