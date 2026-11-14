# GAME-08 — Entrada no mundo: personagem sem atributos, CP errado e nome vermelho

> **ID canônico:** `GAME-08`
> **ID histórico:** `GAME-08`
> **Estado 2026-11-13:** ⬜ · **reaberta** · ver `roadmap-rust/index.md`.

**Tipo:** Bug · **Esforço:** M · **Depende de:** `CLIENT-06` (captura do `STRUCT_MOB`/`UpdateScore` no 7662) · **Bloqueia:** onda 1 (jogável)

## Objetivo

O personagem entra no mundo com **os atributos, o CP e a aparência que estão no banco**, e o
cliente desenha exatamente isso. Hoje o relato do dono é o oposto: personagem sem atributos,
CP errado e **nome vermelho** já no primeiro login.

## Contexto / evidência

- **Relato do dono (teste manual no 7662, 2026-11-13):** “personagem sem atributos no cliente,
  com cp errado, nome vermelho”; “não cria o char direito”.
- Código: `Mob::from_persisted` (`crates/wyd-server/src/character/snapshot.rs`) carrega
  `character_items` em `equip`/`carry`, mas **não recalcula** `current_score` a partir do
  equipamento; `apply_progression` só sobrepõe nível/atributos persistidos.
- Cor do nome: `chaos_rate_de(cp)` = `CP + 75`; CP 0 = 75 (amarelo). Nome vermelho implica
  `cp <= -75` ou byte trocado no fio.
- Medição de 2026-11-13 no canal vivo: personagem nível 1 com equipamento inicial completo
  (slots 0–6, arma `1139`) causa **1 de dano** por golpe em `GremlinDecoy` (600 HP) —
  consistente com bônus de item não aplicado (`GAME-19`).
- A issue foi fechada em 2026-11-12 com testes de persistência (`entrada_mundo`, `pg`), que
  **não** olham o que o cliente desenha.

## Medição no cliente (2026-11-13, print do dono)

Personagem `Cantarelli` (classe TransKnight, conta `admin`). Banco: nível 1, HP 83/83,
MP 46/46, FOR/INT/DES/CON = 8/4/7/6 (`character_scores`), CP 0 (`character_chaos`).

| Campo no painel | Cliente | Banco / esperado | Veredito |
|---|---|---|---|
| Nível | **2** | 1 (fio 0) | divergente |
| HP | 83/83 | 83/83 | ✅ |
| MP | **0/0** | 46/46 | divergente |
| FOR / INT / DES / CON | **0 / 0 / 0 / 0** | 8 / 4 / 7 / 6 | divergente |
| Ataque / Defesa | 5 / 4 | template | a conferir |
| Resist (4) | **245** em todos | 0 esperado | lixo de template no fio |
| Crítico | **102,00%** | 0 esperado | lixo de template no fio |
| Nome | **vermelho** | CP 0 → `ChaosRate` 75 (amarelo) | divergente |

O HP correto mostra que o `STRUCT_MOB`/`Score` chega alinhado até `Hp` (offset 116 do mob);
MP (120) e atributos (124+) vêm errados. `Mob::from_persisted` preserva `Critical`,
`Resist` e `SkillBar` do template (comentário no código) — é a origem provável de 245/102%.
O caminho do `select` também precisa ser conferido contra `list_by_account`, que carrega
`character_scores`/`character_chaos` à parte.

## Passos

1. Capturar no 7662 o `_MSG_CNFCharacterLogin` e o `_MSG_UpdateScore` do primeiro login e
   comparar byte a byte com o `Mob` montado no servidor (`snapshot.rs`).
2. Conferir no banco: `level`, `hp_max`, `cp`/`chaos`, `character_items` e o `Score` esperado da
   classe (Str/Dex/Int/Con) — o cliente lê `CurrentScore` em offsets fixos.
3. Fechar a conta do CP: persistido, codificado (`CP + 75`) e lido pelo cliente; provar 75 para
   personagem novo.
4. Definir onde o bônus de equipamento entra no `Score` (cálculo no login/snapshot) e cobrir com
   teste de ouro do `STRUCT_MOB`.

## Critérios de aceite

- [ ] Personagem novo mostra no cliente os mesmos atributos/HP/MP do banco e da classe.
- [ ] CP de personagem novo = neutro (nome amarelo), provado por print.
- [ ] `STRUCT_MOB`/`UpdateScore` com golden de bytes e captura do 7662.
- [ ] Nenhuma correção por “ajustar até parecer certo” sem captura.

## Captura decodificada (2026-11-14)

Primeira captura real decodificada com o CLI Rust (`wyd-server/src/bin/decode-captura.rs`),
gravada pelo proxy do harness com um login completo (`INITCODE` → `AccountLogin` → `AccountSecure`
→ `CharacterLogin`) do personagem `QAVIVO01` (nível 10, HP 100/100, MP 50/50, FOR/INT/DES/CON
0/0/0/0 no banco; `character_scores` presente, CP 0).

```
#7 S 1832B 0x0114 CNFCharacterLogin id=30000 mob=QAVIVO01 nivel=9 hp=100/100 mp=50/50 for=0 int=0 des=0 con=0 dano=5 chaos=75 resist=[-11,-11,-11,-11] critico=255
#8 S  152B 0x0336 UpdateScore        id=1      nivel=9 hp=100/100 mp=50/50 for=0 int=0 des=0 con=0 chaos=75 dano=5 critico=255 resist=[245,245,245,245] curr_hp=100 curr_mp=50
```

Leitura:

- **Resist 245 e crítico 102% têm origem confirmada no servidor.** `Mob::from_character` herdava
  `Resist[4] = -11` e `Critical = 255` do `BaseMob`; no fio o resist sai `245` (u8) e o cliente
  desenha `255 / 2,5 = 102%`. Corrigido: personagem sai com `critical = 0` e `resist = [0; 4]`
  (equipamento é GAME-19), coberto por `personagem_nao_herda_critico_e_resist_do_template`.
- **MP e atributos do `CNFCharacterLogin`/`UpdateScore` chegam corretos** para um personagem com
  `character_scores`: `mp=50/50` e `curr_mp=50`, atributos iguais ao banco. O `mp=0/0` e os
  `FOR/INT/DES/CON=0` do print **não se reproduzem** nesse caminho — resta comparar com o
  personagem exato do print (há seis linhas vivas `Cantarelli`, ids 57/137/150/151/152/164) e
  capturar o mesmo login no 7662 para saber se o cliente lê outros offsets.
- Nível no fio é 0-based (`nível 10` → `9`), como o `wire_level` documenta.
- `CreateMob` do jogador continua com `mp=0/0` e atributos 0 porque `SceneMob` não carrega esses
  campos (o painel do próprio personagem vem do `CNF`/`UpdateScore`); fica registrado como
  limitação separada, não como causa do print.

Depois da correção, a mesma captura (novo login pelo proxy, canal 1 reconstruído) mostra:

```
#7 CNFCharacterLogin ... mp=50/50 chaos=75 resist=[0,0,0,0] critico=0
#8 UpdateScore        ... critico=0 resist=[0,0,0,0] curr_mp=50
```

Arquivos: `tmp/cliente/captura-bot.jsonl` (antes) e `tmp/cliente/captura-bot2.jsonl` (depois),
fora do git. Evidência de teste: `cargo test -p wyd-server --lib snapshot` 14/14.

O aceite visual do 7662 continua pendente (print + log do mesmo login) e a issue segue **⬜**.

### Comparação com o banco (2026-11-14)

O decodificador ganhou `--dsn`: ele pega conta (`AccountLogin`), personagem/slot (`CNFCharacterLogin`)
e o `STRUCT_SCORE` (do `CNF` e do `UpdateScore`, que substitui) e compara com `characters` +
`character_scores` — nível 1-based do banco convertido por `wire_level`, HP/MP, atributos e
`CP + 75`.

Saída real para `QAVIVO01` (canal 1 reconstruído):

```
banco: QAVIVO01 id=138 slot=0 nivel=10 (fio 9) hp=100/100 mp=50/50 for=0 int=0 des=0 con=0 cp=0
nivel: captura=9 banco=9 OK
hp: captura=100 banco=100 OK
hp_max: captura=100 banco=100 OK
mp: captura=50 banco=50 OK
mp_max: captura=50 banco=50 OK
forca: captura=0 banco=0 OK
...
chaos: captura=75 banco=75 OK
```

Zero divergências para um personagem com `character_scores`. Isso separa o que o servidor manda
(correto neste caso) do que o cliente desenha — o `mp=0/0`/atributos 0 do print continua sem
reprodução no fio e precisa do login do **mesmo** personagem no 7662.
