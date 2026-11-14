# Combate e loja de NPC provados ao vivo — 2026-11-14

> Medições feitas contra a stack local (`wyd-server-canal-1` em `127.0.0.1:8281`,
> Postgres `wyd`), com o bot de protocolo do repositório. Cada linha abaixo sai de log do
> canal ou de consulta ao banco — nenhum valor foi estimado.

## 1. Dano do jogador: `BASE_GetDamage` do legado

**Defeito medido antes:** `event=attack damage=1 hp_left=597` contra o `GremlinDecoy`
(AC 40, HP 600). O `TemplateDamage` do canal fazia `damage_chain` com `base = attack` e
`defense = AC` **inteiro**, o que zerava qualquer golpe de nível 1 (5 − 40 → piso 1).

**Correção:** `world::combat::legacy_damage` implementa `BASE_GetDamage` (`Basedef.cpp:1265`,
transcrita em `docs/wyd/pesquisa-wyd2-missao-fase2-2026-11-10.md` §4.1): `dam - ac/2`, fator
percentual da arma em 100% (centro da faixa 99..110, sem `rand` neste modelo) e a escada de
pisos `<-50 → 0`, `-50..0 → (tdam+50)/7`, `0..=50 → 5*tdam/4 + 7`, mínimo 1.

**Prova (`cargo test -p wyd-server --test sistema_campo_vivo`, log do canal):**

```text
event=attack result=hit npc=6307 at=(2076, 2022) damage=5 hp_left=30 killed=false
event=attack result=hit npc=6307 at=(2076, 2022) damage=5 hp_left=5  killed=false
event=attack result=hit npc=6307 at=(2076, 2022) damage=5 hp_left=0  killed=true exp_base=1313 exp_granted=1313 levels=2
event=death  result=ok  npc=6307 template=GremlinDecoy exp_base=1313 exp_granted=1313 levels=2
event=loot   result=resolved npc=6307 template=GremlinDecoy rolls=[(451, 558185)] drops=[LootDrop { item_index: 451, qty: 1 }]
event=loot   result=inventory npc=6307 slots=[20]
```

E contra um mob de AC baixa (`Porco`, AC 2, HP 70, EXP 295) a mesma fórmula devolve 12:

```text
event=attack result=hit npc=1715 damage=12 hp_left=58 killed=false
event=attack result=hit npc=1715 damage=10 hp_left=0  killed=true exp_base=295 exp_granted=295
event=death  result=ok  npc=1715 template=Porco exp_base=295 exp_granted=295
event=loot   result=refused npc=1715 template=Porco reason=template sem tabela de drop: Porco
```

`PASS(exp): personagem=QACOMBATE01 exp=295` — EXP creditada no banco e lida de volta.

## 2. Cadeia completa da quest do Campo

`cargo test -p wyd-server --test sistema_campo_vivo` → `CAMPO=0`:

```text
PASS(passo 1): GremlinDecoy abatido em (2076,2022)
PASS(campo vivo): conta=qacampo01 personagem=QACAMPO01 id=216 quest=campo_de_treinamento step=2 progress=1 npc=6000
```

O teste confere no banco a chave 451 no inventário depois do abate, o `character_quest`
avançando para `step=1`/`reward_mask=1` e o kit 682 entrando no inventário na entrega.

## 3. Loja de NPC: lista e compra com ouro

`cargo test -p wyd-server --test sistema_loja_viva` → `LOJA=0`:

```text
PASS(cena): 58 mercadores em Armia
LOJA nome=Runas_Joias mob=5340 itens=15 primeiro=Some([128, 12, 0, ...])
PASS(loja): Runas_Joias entregou item 3200 no slot 6 da mochila
PASS(preco): compra com Coin fora do catálogo foi recusada
PASS(compra paga): Runas_Joias entregou item 3207 (invType 1) na mochila; ouro 5000000 -> 4974500
```

Log do canal para a mesma compra:

```text
event=shop result=list    npc=3751 name=Runas_Joias itens=15 cheios=15
event=shop result=bought  npc=5340 pos=0 coin=0 slot=6 item=3200
event=shop result=bought  ... pos=7 coin=25500
```

O `Coin` do pacote é o preço do servidor: divergência é recusada
(`result=refused ... preco divergente (cliente 100, catalogo 25500)`) e o item entra na
mochila com o ouro debitado no mesmo comando.

### 3.1 `Unicornio Puro` — o clique responde, o estoque é vazio no dado

`Unicornio Puro` é comerciante no template (`Merchant` 110) e o clique nele devolve
`_MSG_ShopList` — com **zero** itens, porque o seed importado não traz estoque para ele.

```text
PASS(unicornio): clique respondeu _MSG_ShopList em Unicornio Puro (mob 2231): 0 itens
PASS(unicornio vazio): Unicornio Puro respondeu a lista com 0 itens e recusou a compra (sem estoque no seed)
event=shop result=list    npc=2891 name=Unicornio Puro itens=0 cheios=0
event=shop result=refused npc=2231 pos=0 coin=0 reason=loja recusada: mercador 2891 sem catalogo
```

Origem do vazio (dado, não hipótese):

- `crates/wyd-db/migrations/0020_seed_mundo.sql:4214` — `npc_definition` id **2891**
  `Unicornio_Puro`, `map_id = 0`, `merchant = 110` (é mercador).
- `select count(*) from npc_shop_item where npc_id = 2891` → **0**; a tabela tem **6 706**
  linhas e nenhuma é dele (o seed `origin/main:internal/migrations/seed_npc_shop_item.up.sql`
  registrado em `0020_seed_mundo.sql:13` não lista esse NPC).
- 231 dos 549 NPCs com `merchant <> 0` no mapa 0 estão na mesma situação (guardas/diálogo
  carregam o campo `Merchant` sem catálogo próprio).

Ou seja: o "não é possível comprar" no Unicornio era **loja sem estoque no dado de origem**,
ão clique quebrado. Criar estoque seria inventar item/preço — fora do escopo desta correção.

### 3.2 Onde mora o estoque de loja no legado (fonte, não hipótese)

A lista de loja do legado **é o inventário do próprio NPC**, não um arquivo `Merchant/Settings`:

- `Source/Code/TMSrv/_MSG_REQShopList.cpp:52-55` — o handler só chama `SendShopList` para
  `Merchant == 1` (ShopType 1) e `Merchant == 19` (ShopType 3); qualquer outro valor (110, por
  exemplo) **não recebe pacote nenhum**.
- `Source/Code/TMSrv/SendFunc.cpp:1477-1505` — `SendShopList` copia `pMob[MobIndex].MOB.Carry[]`
  para `MSG_ShopList.List[]`; não existe tabela de catálogo de loja no legado.

Medições no dump importado (blob `npc_template`, `Carry[64]` no offset 268, `Merchant` no 17):

| template | `Merchant` | slots ocupados do `Carry` | `npc_shop_item` |
|---|---:|---:|---:|
| `Unicornio_Puro` | **110** | **0** (blob 816 B) | **0** linhas |
| `Runas_Joias` | 1 | 15 (`[0,3200] … [8,3208], [27,3209], [28,5113], [29,5129]`) | 15 linhas `0:3200 … 14:5135` |

`Runas_Joias` casa item a item entre `Carry` e `npc_shop_item` (a tabela só renumera os slots
ocupados para 0..n), o que confirma a derivação. O `Unicornio_Puro` não tem `Carry` nenhum —
**não houve estoque para importar**, e o legado nem responderia ao clique dele.

> Divergência conhecida (não corrigida aqui): o nosso `handle_shop_list` responde `_MSG_ShopList`
> para **qualquer** `merchant > 0` (lista vazia no 110), enquanto o legado só responde para 1 e 19.
> Alinhar isso muda o comportamento de centenas de NPCs com `merchant` 16/64/… e precisa de
> decisão própria — não foi feito de passagem.

## 4. Dano derivado do equipamento (`EF_DAMAGE`/`EF_AC`)

**Fonte:** `Source/Code/Basedef.cpp:3014-3030` (`BASE_GetCurrentScore`):
`MOB.CurrentScore = MOB.BaseScore`, depois `Damage += BASE_GetMobAbility(EF_DAMAGE)` e
`Ac += BASE_GetMobAbility(EF_AC) + EF_ACADD`. `BASE_GetMobAbility` soma os pares `EF_*` de todo
o `Equip[16]`.

**Implementação:**
- `character_items`/`vault_items` passaram a trazer os três pares do catálogo
  (`item_catalog.effects`) na própria `ItemRow` (`ITEM_EFFECT_COLUMNS` em
  `crates/wyd-db/src/repo/pg.rs`).
- `Item::from_row_with_pity` escala cada valor por `(sanc+10)/10` — regra de
  `BASE_GetItemAbility` (`Basedef.cpp:1687`); sem refino o valor é o do catálogo.
- `world::bonus_de_equipamento` soma `EF_DAMAGE` (2) e `EF_AC` (3) de todo o `Equip[16]`;
  o login soma isso ao `CurrentScore` e leva `attack`/`defense` na `ActorSkin`, que o `Stats`
  do mundo usa (`stats_from`).

**Medição viva** (`sistema_combate_vivo`, `COMBATE=0`), personagem nível 1 com a `Adaga`
(`EF_DAMAGE` 2) e as `Botas_de_Couro(A)` (5): `CurrentScore.Damage` = 5 (template) + 2 + 5 =
**12**; contra `Porco` (AC 2) a `BASE_GetDamage` dá 12 − 1 = 11 → `5*11/4 + 7` = **20**:

```text
event=attack result=hit npc=1720 template=Porco damage=20 hp_left=50 killed=false
event=attack result=hit npc=1720 damage=20 hp_left=30 killed=false
event=attack result=hit npc=1720 damage=20 hp_left=10 killed=false
event=attack result=hit npc=1720 damage=10 hp_left=0  killed=true exp_base=295 exp_granted=295
event=death result=ok npc=1720 template=Porco exp_base=295 exp_granted=295 levels=0
PASS(exp): personagem=QACOMBATE01 exp=295
```

Sem equipamento o mesmo golpe era 12 (5+0+0 de ataque contra AC 2) — a diferença de 8 pontos é
exatamente `EF_DAMAGE` de adaga + botas. O último golpe sai 10 porque o dano é limitado ao HP
restante (regra do canal, `DamageDealt` não ultrapassa o HP).

**Ainda FASE-10:** bônus por **atributo** (`Dex*k + Str*k` por classe×arma, `Basedef.cpp:3245+`)
só entra quando o personagem tem a maestria da arma (`LearnedSkill` bit 7 no TK "Confiança",
bit 15 no Trans). Sem skill aprendida o legado também não soma esses coeficientes.

## 5. Loja alinhada ao legado (só `Merchant` 1 e 19)

`cpsock::action::shop_type_do_merchant`: `Merchant == 1` → `ShopType` 1; `Merchant == 19` →
`ShopType` 3; qualquer outro valor **não** responde `_MSG_ShopList` — é o comportamento de
`_MSG_REQShopList.cpp:50-56`. O `_MSG_Buy` usa a mesma regra, senão um monstro com `Carry` de
drop venderia sem nunca abrir loja.

**Medição viva** (`sistema_loja_viva`, `LOJA=0`):

```text
PASS(cena): 38 mercadores com Merchant 1/19 em Armia (de 58 NPCs com merchant != 0)
event=shop result=list    npc=3751 name=Runas_Joias itens=15 cheios=15
event=shop result=bought  npc=2973 pos=7 coin=25500 slot=9 item=3207
PASS(compra paga): ouro 5000000 -> 4974500
event=shop result=refused npc=2891 name=Unicornio Puro merchant=110 reason=not_shop_merchant
event=shop result=refused npc=3880 name=Guarda_da_Sorte merchant=100 reason=not_shop_merchant
PASS(nao-loja): Guarda_da_Sorte (Merchant 100) não abriu lista nem vendeu
```

Os 20 NPCs de Armia com `merchant` 0/2/16/64/100 deixaram de responder loja; nenhum dado de
`npc_shop_item` foi apagado — para monstros (16/64) essas linhas são o `Carry` de drop, e para
guardas/quest não são mercadoria. Era exatamente o objetivo errado que o handler antigo dava a
eles.

## 6. O que NÃO foi provado

- **Tela do cliente 7662:** os testes falam CPSock; o desenho do dano/da loja na tela não foi
  olhado. O eco `_MSG_AttackOne` existe (teste `eco_do_ataque_leva_o_dano_do_servidor`) mas a
  confirmação visual depende do cliente do usuário.
- **Respawn:** NPC abatido **não volta** — não há regeneração de criatura no servidor; o
  `GremlinDecoy` só reapareceu depois de reiniciar o canal. A quest fica impossível para o
  segundo jogador até o restart.
- **Equipamento e atributos no `Stats`:** `stats_from` continua usando o `Damage`/`Ac` do
  template da classe (`FASE-10`). Não há caminho de equipar item no jogo (`_MSG_UpdateItem`
  sem layout confirmado), então arma comprada não muda o dano.
