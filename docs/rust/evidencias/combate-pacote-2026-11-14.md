# Combate pelo pacote de ataque do cliente 7662 — 2026-11-14

> Missão: o jogador logado manda o pacote de ataque que o cliente 7662 realmente envia, o
> mob perde HP, morre, EXP entra e persiste. Medição viva nos bytes e no banco.
>
> Veredito do turno: **a cadeia inteira fecha pelo frame do cliente** — `_MSG_AttackOne` de
> 168 B mata o `Gremlin Decoy` (600 HP), o abate credita `exp=1313` (nível 1→3), a tabela de
> drop resolve a chave 451, o item entra no slot 20 **do banco**, o canal avisa o cliente com
> `_MSG_SendItem (slot=20, item=451)` e o item continua lá depois de logout **e** relogin.
> Antes deste turno o item já entrava no banco, mas o binário em execução no canal **não tinha**
> o aviso ao cliente (§7.1) — daí "o drop não aparece".

## 1. O que estava quebrado (medido, não deduzido)

O canal real registrava, para o personagem do jogador (conta `admin`):

```
level=warn event=attack result=refused account=admin reason=layout:tamanho de pacote inválido: esperado 68, recebido 168
```

O cliente 7662 manda o golpe básico como **`_MSG_AttackOne` (`0x039D`) com 168 bytes**: o
corpo do `MSG_Attack` de 13 alvos (164 B) mais **4 B de trailer**. O decode do porte exigia
exatamente `AttackOne::WIRE_LEN = 68` e recusava **todo** golpe antes de tocar no motor — daí
"o jogador não tem dano". Nenhum log de `event=attack result=hit` existia para conta de jogador;
só para os bots, que mandavam 68 B.

Duas divergências menores, também reais:

1. **Alvo resolvido só pelo tile.** O `MobAttack` do legado (`Source/Code/TMSrv/Server.cpp:9881`)
   lê `int Target = sm.Dam[i].TargetID;` — o alvo é o id do pacote, não o ponto clicado. O
   porte usava `packet.target_x/target_y` e ignorava `Dam[].TargetID`; um golpe com tile zerado
   dava `result=no_target` mesmo com o id certo no pacote.
2. **Golpe que mata perdia o id.** `attack_at` chamava `despawn` (que devolve o `MobID` ao pool)
   **antes** de montar o `AttackOutcome`, então `target_scene` saía `0` no golpe final: o eco
   `_MSG_AttackOne` e o `_MSG_RemoveMob` do abate iam com id 0.

## 2. Layout do legado e o que o porte decodifica

`Source/Code/Basedef.h` (commit `f39aed386dc3925e9083db34d0f17c932a9d10c6`):

```c
struct STRUCT_DAM { int TargetID; int Damage; };              // 8 B

struct MSG_Attack {                                          // sizeof = 164
    _MSG        Header;                                      // 12
    char        Unk_1[4];                                    // 12
    int         CurrentHp;                                   // 16
    char        Unk_2[4];                                    // 20
    long long   CurrentExp;                                  // 24
    short       unk0;                                        // 32
    short       PosX, PosY;                                  // 34, 36
    short       TargetX, TargetY;                            // 38, 40
    short       AttackerID;                                  // 42
    short       Progress;                                    // 44
    char        Motion, SkillParm, DoubleCritical, FlagLocal; // 46..49
    short       Rsv;                                         // 50
    int         CurrentMp;                                   // 52
    short       SkillIndex, ReqMp;                           // 56, 58
    STRUCT_DAM  Dam[MAX_TARGET];                             // 60, MAX_TARGET = 13
};

struct MSG_AttackOne { ... }  // 68 B: `struct` empacotado, com `CurrentMp@16`/`CurrentHp@52`
                              // trocados — os offsets usados (38/40/42/46/60) são os mesmos
```

O porte (`crates/wyd-protocol/src/payload/combat.rs`) decodifica **os mesmos offsets**:
`TargetX@38`, `TargetY@40`, `AttackerID@42`, `Motion@46`, `SkillIndex@56`, `Dam[0].TargetID@60`,
`Dam[0].Damage@64`, `STRUCT_DAM` de 8 B. A divergência era **só de tamanho aceito**, não de
layout — e o decode agora aceita frame **maior** que o mínimo (o trailer de 4 B do cliente) e
escolhe a aridade de `Dam[]` por `(len − 60) / 8`, limitada a 1..13.

## 3. Arquivos alterados

| arquivo | mudança |
|---|---|
| `crates/wyd-protocol/src/payload/combat.rs` | `decode` aceita frame ≥ `WIRE_LEN`; `ATTACK_BODY_LEN` vira `pub`; teste `frame_do_cliente_7662_le_alvo_e_dano_nos_offsets_do_legado` |
| `crates/wyd-server/src/cpsock/action.rs` | `handle_attack` sem aridade fixa: escolhe `Attack::<N>` pelo tamanho do frame; alvo por `Dam[i].TargetID` com o tile como reserva; log `via=id\|tile` |
| `crates/wyd-server/src/world/{mod.rs,tick.rs,combat.rs}` | `Command::AttackScene` / `WorldHandle::attack_scene` / `Engine::attack_scene_at`; `strike` compartilhado; `target_scene` lido **antes** do `despawn` |
| `crates/wyd-server/src/character/crud.rs` | `SelectionStore::attack_scene` + `reportar_golpe` (EXP/loot/refresh comuns às duas rotas) |
| `crates/wyd-bots/src/bot/{mod.rs,combate.rs}` | `sent_frames()`, `attack_client_at` (168 B sob `0x039D`), `kill_client_at` no ritmo do canal, `attack_full_at` (`0x0367`) |
| `crates/wyd-server/tests/sistema/combate_pacote.rs` | teste vivo novo (dois alvos pelo frame do cliente) |
| `tools/client/ataque-do-legado.mjs` | parser independente dos frames de ataque (offsets literais do legado) |
| `docs/arquitetura/protocolo.md` | nota do frame real de 168 B e da aridade por tamanho |

## 4. Prova viva — canal real (`127.0.0.1:8281`), conta `qaatk01`

> **Saída deste turno** (substitui a rodada anterior, de dois `Porco`). O segundo alvo agora é
> a criatura **com tabela de drop**: `Gremlin Decoy` (6307), 600 HP, item 451.

Comando:

```bash
WYD_LIVE_CPSOCK=127.0.0.1:8281 WYD_TEST_REQUIRED=1 \
WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd \
cargo test -p wyd-server --test sistema_combate_pacote -- --nocapture
```

Saída (íntegra nas linhas que importam):

```
running 1 test
ANTES: banco nivel=1 exp=0 itens=[(0, 1, 1), (1, 1103, 1), (2, 1115, 1), (3, 1127, 1), (4, 1139, 1), (5, 1151, 1), (6, 861, 1), (16, 401, 120), (17, 406, 120), (18, 4130, 1), (19, 4144, 1), (79, 547, 1)]
Porco: ecos=4 [2274/20, 2274/20, 2274/20, 2274/10]
PASS(Porco): env Attack Size=168 TargetX@38=2113 TargetY@40=2046 Dam[0].TargetID@60=2274 | ecos=4 dano_total=70 HP=70 ultimo=10 RemoveMob=sim
PASS(porco): banco exp=295 nivel=1 (antes exp=0 nivel=1)
alvo 2: Gremlin Decoy id=3839 em (2076,2022) hp=600 item da tabela=451
Gremlin Decoy: ecos=100 [3839/6, 3839/6, ... 100 ecos ...]
PASS(Gremlin Decoy): env Attack Size=168 TargetX@38=2076 TargetY@40=2022 Dam[0].TargetID@60=3839 | ecos=100 dano_total=600 HP=600 ultimo=6 RemoveMob=sim
PASS(Gremlin Decoy): banco exp=1608 nivel=3 item=451 slots=[20] novos=["slot=20 item=451 qty=1"] SendItem=[(20, 451)]
PASS(save): apos logout nivel=3 exp=1608 item=451 presente
PASS(relogin): conta=qaatk01 personagem=QAATK01 item=451 presente exp=1608
test ataque_do_cliente_derruba_dois_alvos_e_persiste_exp ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 560.40s
```

Leitura da saída:

| medida | antes | depois |
|---|---|---|
| HP do `Gremlin Decoy` (id 3839) | 600 | **0** (`RemoveMob=sim`, 100 ecos × 6) |
| EXP no banco | 0 | **1 608** (295 do `Porco` + 1 313 do decoy) |
| nível no banco | 1 | **3** |
| inventário | 12 linhas, sem 451 | + `slot=20 item=451 qty=1` |
| `_MSG_SendItem` no fio | — | **`(slot=20, item=451)`** |
| após logout | — | nível 3, exp 1 608, item 451 presente |
| após relogin | — | item 451 presente, exp 1 608 |

Conferência direta no banco **depois** da execução:

```
$ docker exec wyd-db-1 psql -U admin -d wyd -t \
    -c "select c.name, c.level, c.exp from characters c where c.name='QAATK01' and c.deleted_at is null;" \
    -c "select ci.slot, ci.item_id, ci.qty from character_items ci join characters c on c.id=ci.character_id
        where c.name='QAATK01' and c.deleted_at is null and ci.slot=20;"
 QAATK01 |     3 | 1608

   20 |     451 |   1
```

### 4.1 Bytes crus (dump `tmp/combate-pacote/combate.txt`, `raw_frames` de-ofuscados)

Ataque enviado pelo cliente (168 B, `Size=0x00A8`, `Type=0x039D`, `KeyWord=0x0B`):

```
a8000bfa 9d03 0100 07000000 0000...0000 1c08 e607 1c08 e607 0000 0000 01 00 00 00 0000 00000000 0000 0000 ff0e0000 00000000 ...
                                        |    |    |    |    |    |  |  |    |        |    |    |        |
                                        |    |    |    |    |    |  |  |    |        |    |    |        +- Dam[0].Damage@64 = 0 (cliente não credita)
                                        |    |    |    |    |    |  |  |    |        |    |    +- SkillIndex@56/ReqMp@58
                                        |    |    |    |    |    |  |  |    |        |    +- CurrentMp@52
                                        |    |    |    |    |    |  |  |    |        +- Rsv@50
                                        |    |    |    |    |    |  |  |    +- FlagLocal@49 / DoubleCritical@48
                                        |    |    |    |    |    |  |  +- SkillParm@47
                                        |    |    |    |    |    |  +- Motion@46 = 1
                                        |    |    |    |    |    +- Progress@44 = 0
                                        |    |    |    |    +- AttackerID@42 = 0
                                        |    |    |    +- TargetY@40 = 0x07E6 = 2022
                                        |    |    +- TargetX@38 = 0x081C = 2076
                                        |    +- PosY@36 = 2022
                                        +- PosX@34 = 2076
   Dam[0].TargetID@60 = 0x0EFF = 3839 (MobID do Gremlin Decoy)
```

Eco do canal (68 B, `HEADER.ID = 0x7530` = `ESCENE_FIELD`), o **dano do servidor**:

```
44000bec 9d03 3075 ... 1c08 e607 1c08 e607 0100 0000 01 ... ff0e0000 06000000
                             target 2076/2022   attacker=1  motion |  Dam[0] = 3839 / 6
```

`_MSG_RemoveMob` (16 B, `Type=0x0165`) — a criatura saiu da cena pelo abate:

```
10000be4 6501 590c 0000000000000000
```

`_MSG_SendItem` (24 B, `Type=0x0182`, `Slot@14`, item `@16`) — o aviso do drop ao cliente:

```
18000b52 8201 0100 000000000000 0000 1400 c301 0000 0000 0000 0000
                  |         |    |    |    |
                  |         |    |    |    +- item 451 (0x01C3) em `@16`
                  |         |    |    +- qty/resto do STRUCT_ITEM
                  |         |    +- slot 20 (0x0014) em `@14`
                  |         +- invType 0 em `@12`
                  +- HEADER.ID = 1 (id da conexão do dono)
```

### 4.2 Parser independente (`tools/client/ataque-do-legado.mjs`)

```
$ node tools/client/ataque-do-legado.mjs
combate.txt c2s AttackOne size=168 ... Pos@34/36=(2076,2022) Target@38/40=(2076,2022) AttackerID@42=0 Motion@46=1 ... Dam@60=[3839/0]
combate.txt s2c AttackOne size=68  ... Target@38/40=(2076,2022) AttackerID@42=1 Motion@46=1 ... Dam@60=[3839/6]
OK: todo frame de ataque bate com o layout do legado
```

## 5. Log do canal (mesma execução)

```
level=info event=attack result=hit account=qaatk01 npc=6307 via=id at=(2076, 2022) damage=6 hp_left=0 killed=true exp_base=1313 exp_granted=1313 levels=2 client_damage=3839/0
level=info event=death result=ok account=qaatk01 npc=6307 template=GremlinDecoy exp_base=1313 exp_granted=1313 levels=2 at=(2076, 2022) seed=12734484425287104349
level=info event=loot result=resolved account=qaatk01 npc=6307 template=GremlinDecoy rolls=[(451, 485192)] drops=[LootDrop { item_index: 451, qty: 1 }]
level=info event=loot result=inventory account=qaatk01 npc=6307 slots=[20]
level=info event=loot result=send_item account=qaatk01 npc=6307 slot=20 ok=true
```

`via=id` = o alvo veio de `Dam[0].TargetID`; `hp_left=0 killed=true` = o abate saiu do golpe do
frame; `rolls=[(451, 485192)]` = a tabela do `GremlinDecoy` foi consultada e o roll (peso 1 000 000)
escolheu a chave 451; `slots=[20]` = item creditado; `send_item ... ok=true` = o cliente foi
avisado. O `Porco` da mesma sessão continua registrando o caminho de recusa:

```
level=warn event=loot result=refused account=qaatk01 npc=1715 template=Porco reason=template sem tabela de drop: Porco
```

## 6. Causas medidas (não presumidas) e o que foi corrigido

### 6.1 O binário em execução era anterior à entrega do drop ao cliente

Medição:

```
$ docker inspect -f '{{.Created}}' wyd-server-canal-1-1
2026-11-14T18:11:46Z
$ ls -la --time-style=+%Y-%m-%dT%H:%M crates/wyd-server/src/cpsock/action.rs
-rw-r--r-- ... 2026-11-14T16:58 crates/wyd-server/src/cpsock/action.rs
```

O contêiner era **mais antigo que o fonte**: a rota de morte gravava o item no banco
(`loot result=inventory slots=[20]`) mas o laço que emite `_MSG_SendItem` não estava no binário
em execução — o cliente nunca sabia do item. Depois de `docker compose build server-canal-1
server-canal-2` + `up -d`, o mesmo abate passou a registrar `result=send_item ... ok=true` e o
teste viu o frame (§4.1). Correção de origem: **redeploy do canal** (não havia bug de código na
entrega).

### 6.2 O bot de QA não guardava `_MSG_SendItem`

`Bot::recv` (`crates/wyd-bots/src/bot/mod.rs`) só empilhava em `raw_frames()` um conjunto fixo de
opcodes (login, `CreateMob`, `UpdateScore/Etc`, `Attack*`, `RemoveMob`). `SendItem` chegava e era
**descartado**, então nem o teste nem o dump conseguiam provar o aviso. Correção: `Opcode::SendItem`
entrou na lista; é o que permite o `PASS ... SendItem=[(20, 451)]` acima.

### 6.3 A criatura com tabela não vem no delta da sessão parada

O `Gremlin Decoy` existe na cena (é o mob do `sistema_campo_vivo`), mas o `_MSG_CreateMob` dele
só é enviado para quem entra na vista: parado no ponto de save ele não aparece, e um NPC que já
estava na vista antes da caminhada não é reenviado. Caminho legítimo usado pelo teste: andar até
o Campo de Treinamento, **relogar** (o login manda a cena cheia) e só então atacar. Sem inventar
spawn nem chamar função interna.

### 6.4 Arquivos alterados neste turno

| arquivo | mudança |
|---|---|
| `crates/wyd-bots/src/bot/mod.rs` | `Opcode::SendItem` na lista de frames guardados em `raw_frames()` |
| `crates/wyd-bots/src/bot/combate.rs` | `kill_client_at` lê o socket durante todo o ciclo (só o eco de `RemoveMob` interrompe) |
| `crates/wyd-server/src/cpsock/action.rs` | log `result=send_item ... ok=` e `result=sem_linha` por slot entregue |
| `crates/wyd-server/tests/sistema/combate_pacote.rs` | alvo 2 vira `Gremlin Decoy` (caminhada + relogin), asserção de `SendItem`, do delta de inventário e do estado pós-logout/pós-relogin |
| `docs/rust/evidencias/{combate-pacote-2026-11-14.md,README.md}` | esta evidência |

### 6.5 Ritmo do abate (medido)

Com o frame de 168 B o canal aceita ~1 golpe a cada 5 s (medido nos logs: `hp_left` caindo de 6 em
6 a cada ~5,0 s, com o cliente enviando um golpe por ciclo e lendo o socket no resto do ciclo).
600 HP ÷ 6 = 100 golpes ⇒ ~560 s de execução. Não foi alterado nenhum valor de combate para
acelerar: o teste usa 24 janelas de 25 s.

## 7. Testes automatizados que travam isto

1. `crates/wyd-protocol/src/payload/combat.rs::frame_do_cliente_7662_le_alvo_e_dano_nos_offsets_do_legado`
   — monta o frame de 168 B campo a campo nos offsets do legado e exige `TargetX=38`,
   `TargetY=40`, `Dam[0].TargetID=60`, `Dam[0].Damage=64`; frame de 163 B continua `BadLength`.
2. `sistema_combate_pacote` (vivo) — envia o frame de 168 B pelo canal real, mata o
   `Gremlin Decoy` (600 HP), e **exige**: eco com dano, `RemoveMob`, EXP/nível no banco, item 451
   no inventário persistido **e** `_MSG_SendItem` com o índice 451 no fio, além do mesmo estado
   depois de logout e de relogin. Falha se o decode divergir (`Size` ou offsets) ou se o drop não
   vier.
3. `crates/wyd-bots` manda **o frame do cliente** (`attack_client_at`), não uma chamada interna.

## 8. Gates deste turno

O teste vivo acima e o dump `tmp/combate-pacote/combate.txt` foram gerados com o código final
desta rodada (canal reconstruído: `docker compose build server-canal-1 server-canal-2`).

```
cargo fmt --all --check                                → FMT=0
cargo clippy --workspace --all-targets -- -D warnings  → CLIPPY=0
cargo test --workspace --all-targets                   → TESTALL=0 (860 passed, 0 failed, 79 binários)
node tools/dev/check-docs-consistency.mjs              → DOCS=0
node tools/content/check-issue-status.mjs              → ISSUES=0
node tools/content/check-audit-coverage.mjs            → AUDIT=0
node tools/dev/check-skills.mjs                        → SKILLS=0
node --test apps/tools/test/*.test.mjs                 → TOOLS=0
```

Live (canal real) deste turno:

```
cargo test -p wyd-server --test sistema_combate_pacote -- --nocapture → ok (560.40s)
node tools/client/ataque-do-legado.mjs                               → EXIT=0 "OK: todo frame de ataque bate com o layout do legado"
```

## 9. O que continua aberto

- **Tela do cliente não verificada**: a prova é de bytes no canal + banco, não de render do
  inventário na GUI.
- O ritmo de ~1 golpe/5 s com o frame de 168 B não foi explicado neste turno (medido, não
  investigado a fundo); o bot de QA joga nesse ritmo.
- Golpes de skill (`SkillIndex != 0`) seguem aplicando só o dano físico.
- Nada foi commitado ou enviado; a árvore tem as mudanças desta rodada sem commit.
