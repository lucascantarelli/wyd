# Nível no fio: banco `N` → pacote `N − 1` (2026-11-14)

Pergunta do usuário: *o cliente trata o nível como base `[0,1]` enquanto o banco/legado usa base 1
— nível 1 no banco aparece como 2, 399 como 400, 0 como 1?*

## 1. Veredito

**Confirmado.** A proposição do usuário está certa na substância: o cliente 7662 desenha
`Level + 1`, então o nível que trafega é 0-based. O legado também é 0-based (as quests de nível
provam: `Level355` dispara em `cur == 354`) e manda o valor **sem conversão**. A ressalva é de
vocabulário: "banco" 1-based é escolha **deste porte**, não do legado — o banco do legado guarda o
mesmo `BaseScore.Level` 0-based do fio. Logo a conversão correta é sempre `banco − 1`, e sem ela o
cliente pinta um nível a mais — exatamente o "nível 1 aparece como 2" relatado.

| Camada | Base | Valor para o 1º nível | Fonte |
|---|---|---|---|
| `BaseScore.Level`/`CurrentScore.Level` do legado (e o que vai no pacote) | 0-based | `0` | `CMob::CheckGetLevel` (`CMob.cpp:1069-1115`) |
| `characters.level` deste porte | 1-based | `1` | `crates/wyd-db/src/model/character.rs:47` |
| `Selchar.Score[i].Level`, `CNFCharacterLogin`, `CreateMob` | 0-based (`banco − 1`) | `0` | `personagens::wire_level` |

Provas no legado:

- `Source/Code/Basedef.h:177`: `#define MAX_LEVEL 399`; `CMob.cpp:1075` para de subir em
  `BaseScore.Level >= MAX_LEVEL` — logo o teto interno é `399` e o desenhado é `400`.
- `CMob.cpp:1110-1111`: a quest `Level355` do Arch dispara em `cur == 354` e a `Level370` em
  `cur == 369`: o nível interno é sempre `desenhado − 1`.
- `Source/Code/TMSrv/Server.cpp:7910`: `SelChar.Score[slot].Level = pMob[conn].MOB.BaseScore.Level;`
  — nenhuma soma antes de mandar ao cliente.
- `Source/Code/TMSrv/_MSG_CharacterLogin.cpp:31`: o gate de entrada compara
  `SelChar.Score[slot].Level < FREEEXP` (`FREEEXP = 35`, `Server.cpp:50`) — outro consumidor do
  mesmo campo 0-based.

## 2. Onde a conversão vive (e onde estava faltando)

`personagens::wire_level` (`crates/wyd-server/src/character/snapshot.rs`) é o único ponto:

- `Selchar::from_roster` → `Mob::from_persisted` → `apply_character` (`self.level = wire_level(...)`);
  alimenta `_MSG_CNFAccountLogin` e `_MSG_CNFNewCharacter`.
- `Mob` de `_MSG_CNFCharacterLogin` (`base_score` e `current_score`).
- `SceneMob` do `_MSG_CreateMob` (`world/scene.rs`: `level: estado.level.saturating_sub(1)`), onde
  `LiveState.level` guarda o nível do banco.
- Volta de `_MSG_CreateMob` para o mundo (`ActorRefresh::from_score`, `world/mod.rs`): `fio + 1`.

Foi esse último par que faltava: antes, o `CreateMob` do personagem mandava o nível do banco e o
cliente desenhava "Nv 2" para nível 1.

## 3. Prova viva (canal `127.0.0.1:8281`, banco `wyd`)

Conta `qanivel01`, personagens gravados com níveis de borda e um criado pelo protocolo:

```console
$ WYD_LIVE_CPSOCK=127.0.0.1:8281 WYD_TEST_REQUIRED=1 \
  WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd \
  cargo test -p wyd-server --test sistema_nivel_wire -- --nocapture
running 1 test
PASS(criacao): banco=1 selchar=0 login=0 createmob=0 cliente=1
PASS(login): slot=0 banco=200 selchar=199 login=199 cliente=200
PASS(login): slot=1 banco=399 selchar=398 login=398 cliente=399
test nivel_do_banco_chega_ao_cliente_sem_salto ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.32s
```

`selchar` é o `Score.Level` decodificado do `_MSG_CNFAccountLogin` (`+20`) ou do
`_MSG_CNFNewCharacter` (`+4`) — o corpo cru do frame que o bot recebeu, não uma recontagem do
banco; `login` é o `CurrentScore.Level` do `STRUCT_MOB` do `CNFCharacterLogin`; `createmob` é o
`Level` do `_MSG_CreateMob` do próprio personagem.

Banco depois da execução:

```console
$ docker exec wyd-db-1 psql -U admin -d wyd -c \
  "select name, slot, level, exp from characters where name like 'QANIVEL%' and deleted_at is null order by slot;"
   name   | slot | level | exp
----------+------+-------+-----
 QANIVEL0 |    0 |   200 |   0
 QANIVEL1 |    1 |   399 |   0
 QANIVEL3 |    3 |     1 |   0
(3 rows)
```

Cliente desenhado = valor do banco em todos os casos (1→1, 200→200, 399→399).

## 4. Regressão

- `crates/wyd-server/tests/sistema/nivel_wire.rs` (`sistema_nivel_wire`): falha se qualquer um dos
  três pacotes (Selchar, `CNFCharacterLogin`, `CreateMob`) mandar o nível do banco cru.
- `character::snapshot::tests::nivel_do_banco_vira_nivel_menos_um_em_todo_pacote`: 1→0, 200→199,
  399→398 em `CurrentScore`, `BaseScore`, `wire_level` e `Selchar`.
- `world::tests::score_do_fio_vira_nivel_do_banco_no_refresh`: 398 no fio → 399 no mundo (pós
  level-up), 399 é o teto sem estouro.
- Teste de mutação: com `WIRE_LEVEL_OFFSET = 0` o teste unitário falha
  (`assertion left == right failed: CurrentScore banco 1`); restaurado para 1 em seguida.

## 5. NÃO verificado

- **Level-up ao vivo com pacote**: o abate no campo exige um monstro acessível e o canal não
  respawna (`GremlinDecoy`/`Porco` só voltam com restart); nesta rodada a caminhada chegou ao campo
  mas nenhum alvo ficou parado o bastante para o golpe. A conversão do refresh pós-level-up está
  coberta por teste unitário (`ActorRefresh::from_score`), e o estado pós-level-up é o mesmo que os
  casos 200/399 provam ao vivo: nível no banco → login → fio `N − 1`.
- **Tela do cliente 7662**: nenhuma captura visual; a prova é o corpo do pacote decodificado.
- **Nível 0 no banco**: o schema é 1-based (`MAX_DB_LEVEL = 400`, nível 1 mínimo); o caso "0 no
  banco" do relato não é representável aqui — o equivalente 0-based é o fio.
