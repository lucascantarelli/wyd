# Roteiro de teste manual — cliente 7662 e portal (2026-11-14)

> **Para que serve:** a suíte automatizada prova o servidor, o protocolo e o banco; **não** prova o
> que o cliente 7662 desenha nem o que o jogador percebe. Este roteiro é o caminho para fechar essa
> lacuna com evidência registrada (print, log e consulta), sem inventar resultado.

## 0. O que está provado e o que não está

| Camada | Prova | Situação |
|---|---|---|
| Regras puras (`wyd-core`, `wyd-sim`) | `cargo test --workspace --all-targets` → **851 passed, 0 failed, 1 ignored**, exit 0 | ✅ automatizada |
| Servidor + banco real (`wyd-server`, `wyd-admin`) | suíte de sistema com Postgres (`WYD_TEST_REQUIRED=1`) → **300 passed, 0 failed**, 7 SKIPs só de `WYD_LIVE_CPSOCK` | ✅ automatizada |
| Fio CPSock sem cliente gráfico | testes de sistema em loopback (handshake, login, seleção, cena, refino, lotação) | ✅ automatizada |
| **Cliente gráfico 7662** | **nenhum teste automatizado** — `tests/sistema` usam socket próprio, não o `WYD.exe` | ⚠️ **só manual** |
| Funções sem pacote no fio | gema aplicada, skill, mix, trade, drop no chão, baú, party/guilda, PvP, teleporte, clima | ⛔ **não testável no cliente hoje** (não existe caminho) |

**Resposta curta:** não, não está tudo testado com evidência. O que dá para afirmar com prova está na
tabela acima; o que depende do cliente só fecha com este roteiro. E há funções que o roteiro **deve**
tratar como falha esperada, porque não foram implementadas (lista no §4).

## 1. Preparação (10 min)

### 1.1 Stack no ar

```bash
docker compose up -d --build --wait --wait-timeout 300   # make local-rebuild é o atalho
docker ps --format '{{.Names}}\t{{.Status}}'
```

Use o `--wait-timeout 300`: o `mkdocs` reconstrói 250+ páginas (~105 s) e, com o timeout padrão, o
`--wait` aborta com `exit 1` e `container wyd-mkdocs-1 is unhealthy` **sem** que a stack esteja
quebrada (observado em 2026-11-14: em seguida o container aparecia `healthy`). Sem `--wait-timeout`,
confira o estado real com `docker ps` antes de concluir que há falha.

Esperado: `db`, `gateway`, `portal`, `server-canal-1`, `server-canal-2` (e monitoramento) `healthy`.
Sanidade rápida (saídas observadas em 2026-11-14):

```bash
curl -s http://127.0.0.1:7600/health          # {"ok":true,"database":"up","schema":50}
curl -s http://127.0.0.1:3000/api/health      # HTTP 200 (contagem real do catálogo)
curl -s http://127.0.0.1/serv00.htm           # duas linhas = jogadores por canal, ex.: "1\n0"
```

O `serv00.htm` **não** é HTML: o corpo são dois inteiros (um por canal), servidos pela porta 80
do container do canal 1. Use `| head` só para encurtar; comparar com print da tela de canais.

### 1.2 Cliente apontando para o servidor local

- `client/serverlist.bin` deve apontar o canal 1 para `127.0.0.1:8281` e o status para a porta 80;
  canal 2 usa `127.0.0.2:8281` (o cliente fixa 8281).
- Valide o pacote do cliente antes de abrir: `cargo run -p wyd-launcher -- check client client/manifest.txt`
  (`exit 0` = íntegro; exit diferente = relatório por arquivo). Ele lê do disco e faz hash do `client/`
  inteiro, então leva minutos — não é travamento.
- `client/WYD.exe` deve ser o build 7662 (`SHA-256 04c9d2a1…27ddf8`). Versão diferente envia outro
  `ClientVersion` e o handshake recusa (`esperado 12000`).

### 1.3 Conta de teste

Use uma conta **player** com senha e PIN conhecidos. Sem GUI, crie pelo BFF (mesma rota que o
portal usa; o token está no `.env`, nunca o imprima):

```bash
set -a; . ./.env; set +a
curl -s -X POST http://127.0.0.1:7600/admin/v1/accounts/create \
  -H "authorization: Bearer $WEB_API_TOKEN" -H 'x-wyd-actor: 1:admin' \
  -H 'content-type: application/json' \
  -d '{"login":"manual_teste_20261114","password":"Teste#12345","role":"player"}'
# observado: {"result":1,"account":{"id":664,"login":"manual_teste_20261114","role":"player"}}
```

O header `x-wyd-actor` é `ID:PAPEL` (a conta `admin` tem `id = 1`); só `1` devolve
`x-wyd-actor inválido`. Alternativa com GUI: `http://127.0.0.1:3000/login` (staff) → `/contas`.

O PIN é cadastrado no **primeiro login no jogo** (4–6 dígitos). Personagem de teste: sem GUI, rode
`WYD_TEST_REQUIRED=1 WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd cargo test -p wyd-server --test sistema_personagens_crud`
(cria conta própria e 4 personagens pelo protocolo; observado: `2 passed`, personagens nos slots
0–3 com 5.000.000 de ouro e posição `2096,2096`). No cliente, a criação é o passo T5.

### 1.4 Kit de evidência (crie a pasta antes de começar)

```bash
mkdir -p tmp/manual-7662-2026-11-14
```

| Fonte | Como coletar |
|---|---|
| Tela do cliente | print por passo (nome `T05-01-selecao.png`) |
| Log do cliente | `client/WYD.log` (copiar ao fim de cada bloco) |
| Log do canal | `docker logs --since 10m wyd-server-canal-1-1 > tmp/manual-7662-2026-11-14/canal1.log` |
| Log do gateway | `docker logs --since 10m wyd-gateway-1` |
| Banco | `docker exec wyd-db-1 psql -U admin -d wyd -c "<consulta>"` |
| Métricas | Prometheus (`:9090`), **não** `:9898` — essa porta não é publicada no host. Ex.: `curl -s 'http://127.0.0.1:9090/api/v1/query?query=wyd_packets_rejected_total'` |

Consultas úteis (rode antes/depois de cada bloco que muda estado):

```bash
docker exec wyd-db-1 psql -U admin -d wyd -c \
  "select id, login, role, pin_locked_until from accounts where login = '<LOGIN>';"
docker exec wyd-db-1 psql -U admin -d wyd -c \
  "select id, name, slot, level, exp, gold, pos_x, pos_y, hp, mp from characters where account_id = <ID>;"
docker exec wyd-db-1 psql -U admin -d wyd -c \
  "select slot, item_id, qty, refine, ancient, gem_effect, gem_value from character_items where character_id = <ID>;"
docker exec wyd-db-1 psql -U admin -d wyd -c \
  "select slot, pity from refine_pity where character_id = <ID>;"
docker exec wyd-db-1 psql -U admin -d wyd -c \
  "select quest_code, step, progress, completed_at from character_quest where character_id = <ID>;"
docker exec wyd-db-1 psql -U admin -d wyd -c \
  "select account_login, channel, opened_at, released_at from character_session where released_at is null;"
```

Semântica medida no banco: `character_items.item_id` é o índice do catálogo (não `item_index`);
`character_quest.step` é 1-based (`step = stage + 1`) e `progress` **é** o `reward_mask` (um bit por
etapa paga); `completed_at` não nulo = quest concluída. `refine_pity` só ganha linha depois de uma
falha de refino — vazio é o esperado em personagem novo. `character_session` guarda histórico:
só `released_at is null` é sessão viva, e uma queda sem logout pode deixar linha órfã.

### 1.5 O que grantar antes (portal → `/entregas`)

| Item | Índice | Para testar |
|---|---|---|
| Poeira de Oriharukon | 412 | refino +0→+6 |
| Poeira de Lactolerium | 413 | refino +9→+10 |
| Arma +0 de teste | 5017 (`Espada_Flamejante`, `req_level = 0`) | refino e combate |
| Ouro | — (personagem novo já nasce com 5.000.000) | loja e taxas |

Índices conferidos em `item_catalog` no dia 2026-11-14: 412, 413, 451, 452, 453, 481, 652, 682
existem. Atenção ao nome real: o catálogo grafa **`Poeira_de_Oriharucon`** (sem o “k”), então
buscar por “Oriharukon” na UI não encontra.

---

## 2. Blocos de teste (ordem recomendada)

Cada bloco tem **passos**, **esperado** e **onde provar**. A marca `[A]` indica que existe teste
automatizado cobrindo o mesmo comportamento (o manual confirma a ponta do cliente); `[M]` é só
manual; `[X]` é falha esperada hoje.

### T1 — Launcher e pacote `[A]`

1. `cargo run -p wyd-launcher -- check client client/manifest.txt` → relatório por arquivo
   (os dois argumentos são obrigatórios: `<dir-do-jogo> <manifesto>`).
   **Esperado em diretório já usado:** `exit 1` com **exatamente** `Config.bin: hash divergente` e
   `WYD.log: tamanho divergente`, porque o cliente reescreve os dois ao rodar e o manifesto os lista.
   Isso **não** é pacote corrompido: o mesmo comando num `client/` limpo dá `0 com problema`. Qualquer
   outra divergência (principalmente em `WYD.exe`) é falha real de integridade.
2. `cargo run -p wyd-launcher -- launch client client/manifest.txt client/serverlist.bin --dry-run`
   → imprime o plano (check → patch → spawn) sem abrir o jogo.
3. Abrir `APP`/launcher e clicar em **Jogar**.

**Esperado:** processo do cliente sobe, sem erro de hash. **Prova:** saída do comando + print.
**Se falhar:** o problema é o pacote do cliente, não o servidor (`CLIENT-03/CLIENT-07`).

### T2 — Seletor de canais e status `[A]`

1. Abrir o cliente → tela de canais.
2. Conferir o canal 1 (`127.0.0.1`) e o canal 2 (`127.0.0.2`).

**Esperado:** os dois aparecem; o status vem de `serv00.htm`. **Prova:** print + `curl serv00.htm`.
**Falha esperada:** se o `serverlist.bin` estiver com outro IP, o cliente não lista — corrigir o
arquivo, nunca a checagem de versão.

### T3 — Login: senha `[A]`

1. Digitar a conta e uma **senha errada**.
2. Depois a senha correta.

**Esperado:** a senha errada é recusada e **não** cria sessão (log `event=login result=password_denied`);
a correta leva ao PIN/seleção.
**Prova:** `canal1.log` (linhas `event=login`) + print da mensagem; `select pin_failures`.
**Falha esperada:** `BUG-01`/`DIV-01` — o 7662 pode não exibir mensagem de erro de senha; se a tela
ficar muda, registre como `BUG-01` (não como falha nova).

### T4 — PIN numérico `[A]`

1. Primeiro login → cadastrar PIN de 4 dígitos.
2. Sair e entrar com PIN **errado** uma vez.
3. Errar 5 vezes seguidas → conta bloqueada por 10 min (`pin_locked_until` no banco).
4. Esperar/limpar o bloqueio (`update accounts set pin_failures=0, pin_locked_until=null where login='<LOGIN>';`)
   e entrar com o PIN certo.
5. No portal, `/conta` → trocar o PIN (exige o atual, duas etapas); tentar o PIN antigo no jogo.

**Esperado:** PIN errado recusado sem sessão; 5 erros bloqueiam; PIN antigo deixa de valer.
**Prova:** prints + `select pin_failures, pin_locked_until, pin_hash is not null` + log
`event=pin result=...`.

### T5 — Personagens: seleção, criação, deleção `[A]`

1. Criar um personagem novo (nome, classe) → ele aparece na lista.
2. Entrar com ele, sair, voltar: os 14 campos da tela devem refletir o banco (nome, classe, nível,
   EXP, HP/MP e máximos, ouro, PvP, posição, slot, guilda).
3. Criar até o limite (4 slots) → a 5ª criação deve ser recusada.
4. Deletar um personagem: nome divergente recusa; nome correto faz soft delete e ele sai da lista.

**Esperado:** roster = banco a cada volta. **Prova:** print + `select name, slot, level, exp, gold,
pos_x, pos_y, hp, mp from characters where account_id=<ID>`; conferir `deleted_at`.
**Equivalente sem GUI:** `cargo test -p wyd-server --test sistema_personagens_crud` com
`WYD_TEST_DSN` apontando para o banco vivo (casos `cria_quatro_personagens...` e
`cria_e_deleta_personagem_pelo_protocolo`).
**Resíduo conhecido:** o layout de `STRUCT_SELCHAR` é `rust-v1` até a captura `CLIENT-06`; se algum
campo aparecer trocado, anote qual (não “conserte” o layout sem captura).

### T6 — Entrada no mundo `[A]`

1. Selecionar o personagem e entrar.
2. Não se mover por 10 s.

**Esperado:** personagem na **posição salva** (mesma do banco), HP/MP do banco, nome/level corretos,
barra de EXP coerente; a cena carrega sem “Falha ao conectar”.
**Prova:** print da tela + `select pos_x, pos_y, hp, mp, level, exp` antes/depois +
`canal1.log` (`event=enter`, `event=scene`). **Falha esperada:** render/`Error in Init Render Target
Texture` é ambiente gráfico (`CLIENT-12`), registre separado do conteúdo.

### T7 — Cena: NPCs e criaturas `[A]`

Na área inicial do Campo de Treinamento devem existir (dump herdado + seed `0046` para as
criaturas de prova; duplicatas removidas na `0051`, mapa 0):

| NPC | Coordenada |
|---|---|
| Treinador1 (merchant 36) | 2080, 2018 |
| Treinador2 (40) | 2142, 1990 |
| Treinador3 (41) | 2078, 1966 |
| Chefe_de_Treino (100) | 2121, 2040 |
| Unicornio_Puro (110) | 2121, 2047 |
| Ajudante (120) | 2130, 2035 |
| Gremlin Decoy | 2076, 2022 |
| Giant Crill | 2146, 1997 |
| Orc Sniper | 2078, 1971 |
| Guerreiro Orc | 2135, 1948 |

**Esperado:** cada nome aparece **uma vez** (não pode haver dois "Chefe de Treino" nem
`Unicorn`/`Cabuncle Wind` dentro do Campo); ao se aproximar/afastar, entram e saem da cena
(delta). **Prova:** prints com o nome visível + `select slug, template_name, pos_x, pos_y from
npc_definition where map_id = 0 and (template_name in ('Treinador1','Treinador2','Treinador3',
'Chefe_de_Treino','Unicornio_Puro','Ajudante') or slug like 'campo-%')`.
**Falha esperada:** se o NPC não for clicável/renderizado, é `GAME-09` (prova visual pendente) —
registre o print como evidência **negativa**, não como sucesso.

### T8 — Movimento e colisão `[A]` (autoridade) + `[M]` (colisão) + `[X]`

1. Andar em linha reta, na diagonal e em volta de um NPC.
2. Tentar atravessar uma parede/obstáculo.

**Esperado:** o servidor aceita deslocamento dentro do orçamento e recusa teleporte (o cliente não
“cola”); **atravessar parede hoje é possível** — colisão de jogador não existe (`DIV-10`,
`PAR-MOV-02`).
**Prova:** print/vídeo do caso da parede + log `event=move result=rejected` quando houver salto.
**Não reporte como bug novo:** já é `AUD`/`DIV-10`; registre como confirmação do achado.

### T9 — Combate, morte e EXP `[A]` + `[X]`

1. Atacar o **Gremlin Decoy** (600 HP) com o ataque básico.
2. Observar dano por golpe, barra de HP do mob e, no golpe fatal, o desaparecimento do mob.
3. Conferir EXP/score e log de morte.

**Esperado hoje:** o golpe é aceito, o mob morre e sai da cena (`RemoveMob`), EXP/score atualizam.
**Falha esperada (GAME-19):** personagem nível 1 com equipamento inicial pode causar **1 de dano**
(o bônus de equipamento não entra no score). Registre o número exato por golpe e o item equipado —
esse número é a evidência que a issue precisa.
**Prova:** print do dano + `canal1.log` (`event=death`, `event=loot`) + `select exp, level from
characters where id=<ID>`.

### T10 — Loot e inventário `[A]` + `[X]`

1. Matar Gremlin Decoy (drop garantido da prova 451) e conferir a mochila.
2. Verificar se aparece **caixa no chão**.

**Esperado:** o item entra no inventário (`SendItem` por slot, empilha até 120) e o log mostra
`event=loot`. **Caixa no chão não existe** (handler `CreateItem`/`GetItem` é `GAME-11`; a direção do
0x026E foi corrigida em `AUD-02`, mas o handler não foi implementado).
**Prova:** print da mochila + `select slot, item_id, qty from character_items where
character_id=<ID>`.

### T11 — Refino com pó e piedade `[A]`

1. Equipar a arma +0 e usar a **Poeira de Oriharukon** (412) sobre ela.
2. Repetir até +5/+6; observar sucesso, falha e a mensagem de recusa no teto do pó.
3. Falhar de propósito algumas vezes e conferir a piedade; depois acertar e ver a piedade zerar.
4. Tentar refinar item selado/`EF_NOSANC` (se tiver) → recusa sem consumir.

**Esperado:** o item volta com novo refino via `SendItem` (0x0182, 24 B); falha **não** destrói;
`refine_pity` acumula por personagem/item e zera no sucesso; recusa não muda o item.
**Prova:** prints do antes/depois + `select slot, refine from character_items ...` +
`select * from refine_pity where character_id=<ID>` + log `event=refine`.

### T12 — Quest do Campo de Treinamento `[A]`

1. Clicar no **Treinador1** (2080, 2018) e aceitar a quest.
2. Matar Gremlin Decoy → pegar a prova 451; entregar ao treinador.
3. Repetir com Segundo (452/Giant Crill) e Terceiro (453/Orc Sniper).
4. Chefe de Treino (100 + grade 16 do `Equip[0]`) → recompensa final (682/481/652, teto de nível 35).

**Esperado:** cada etapa persiste (`character_quest.step` = etapa 1-based, `progress` = máscara de
1 bit por etapa paga), reentrega não duplica (`AlreadyRewarded`), a recompensa entra no inventário.
**Prova:** prints de cada etapa + `select quest_code, step, progress, completed_at from
character_quest where character_id=<ID>` (observado em personagem que aceitou: `campo_de_treinamento`,
`step = 1`, `progress = 0`, `completed_at` nulo).
**Falha esperada:** buff do Cabuncle e portão visível continuam pendentes (`GAME-18`).

### T13 — Portões com chave `[A]`

1. Ir até os portões (2075, 2016), (2143, 1986) e (2081, 1962).
2. Usar a chave certa (itens 451/452/453) em cada um; tentar a chave errada e uma segunda vez.
3. Esperar 30 s.

**Esperado:** abre só com o `EF_KEYID` certo, recusa a segunda chave, fecha sozinho em 30 s e a
chave é consumida.
**Prova:** prints + `select slot, item_id, qty from character_items` antes/depois + log
`event=gate`.

### T14 — Loja de NPC `[A]`

1. Clicar em um treinador/merchant (36/40/41/100) e abrir a loja.
2. Comprar um item barato; conferir ouro e item.
3. Tentar comprar sem ouro.

**Esperado:** `ShopList` (236 B) com o catálogo do mercador, `Buy` debita o ouro e entrega o item;
sem ouro a compra é recusada.
**Prova:** prints + `select gold from characters` antes/depois + `select item_id, qty from
character_items`.
**Atenção:** o checklist ainda marca loja como ⬜ (`DIV-09`) — o código e os testes de sistema já
têm o fluxo; o print deste passo é o que reconcilia o documento.

### T15 — Logout e queda `[A]`

1. Sair pelo menu do jogo → volta à seleção/canais.
2. Entrar de novo e derrubar o processo do cliente (Task Manager).
3. Entrar de novo e ficar parado 120 s.

**Esperado:** os três caminhos liberam a sessão (`event=logout result=ok reason=explicit|socket|idle`),
e a mesma conta pode entrar em outro canal na sequência.
**Prova:** prints + log + `select account_login, channel, released_at from character_session where
released_at is null` (nenhuma linha da sua conta após sair; linha órfã de queda abrupta é achado, não
sucesso).

### T16 — Lotação, manutenção e limite por IP `[A]`

1. Subir o canal 1 com `WYD_SERVER_MAX_PLAYERS=1` (recriar o container) e tentar o 2º login.
2. Subir com `WYD_SERVER_MAINTENANCE=1` e tentar entrar.
3. Abrir 7 conexões do mesmo IP.

**Esperado:** recusa `Capacity`/`Maintenance` **antes** de abrir sessão, mensagem ao cliente (quando
o 7662 exibe), métrica `wyd_packets_rejected_total{reason="canal_lotado"|"manutencao"}` e
`/health` com `maintenance`; a 7ª conexão do IP é recusada (`WYD_MAX_CONNECTIONS_PER_IP=6`).
**Prova:** prints + `curl -s http://127.0.0.1/health` (observado: `{"ok":true,…,"channels":2,"channel":1,"online":0,"maintenance":false}`) + `curl -s 'http://127.0.0.1:9090/api/v1/query?query=wyd_packets_rejected_total'`
(observado: série por `canal` e `reason`, ex. `reason="rate"`; `:9898` não é publicado no host).
**Nota:** reiniciar o container **apenas** fora de um teste humano em andamento (`AGENTS.md` §0).

### T17 — Portal (staff) `[A]`

1. `/login` com staff → `/contas`: buscar conta, trocar papel, bloquear com motivo.
2. `/auditoria`: a ação acima aparece com ator, alvo e motivo.
3. `/economia`: números de `city_tax_ledger`/`delivery_queue`.
4. `/entregas`: conceder 250 de ouro → conferir o ouro no jogo em ≤ 5 s; repetir com a mesma chave
   (replay) → não duplica.
5. `/conta` (jogador): trocar PIN.

**Esperado:** tudo auditado, sem escrita direta no estado vivo (grants pela fila). **Prova:** prints
+ `select * from audit_log order by id desc limit 5` + `select * from delivery_queue order by id desc
limit 5`.

---

## 3. Matriz de evidência por bloco

| Bloco | Automatizado equivalente | Falta no manual |
|---|---|---|
| T1 launcher | `cargo test -p wyd-launcher` | spawn real de N janelas (Windows do jogador) |
| T2 canais/status | `tests/sistema/status_udp.rs` | render da lista no 7662 |
| T3 senha | `tests/sistema/login.rs` | mensagem visível (`BUG-01`) |
| T4 PIN | `login.rs`, `pin_persistencia.rs`, `pin_troca.rs` | teclado numérico do cliente |
| T5 personagens | `selchar.rs`, `personagens_crud.rs` | offsets do `STRUCT_SELCHAR` (`CLIENT-06`) |
| T6 entrada | `cena_mobs.rs`, `entrada_mundo.rs` | render de posição/score |
| T7 cena | `world::tests::plan_scene_emite_entrada_saida_e_respeita_o_teto`, `cpsock::tests` | clique/render de NPC (`GAME-09`) |
| T8 movimento | `crates/wyd-core/tests/integracao/autoridade.rs` (`teleporte_forjado_nao_move_nem_publica`, `passo_dentro_do_orcamento_e_gravado_pelo_servidor`) | colisão (`DIV-10`) **não existe**; prova com dois clientes (`QA-13`) |
| T9 combate | `autoridade.rs` (`dano_aplicado_vem_do_modelo_do_servidor`), `wyd-sim` `tests/golden/formulas.rs`, `campo_treinamento.rs`, `config_gameplay.rs` | dano real exibido com equipamento (`GAME-19`) |
| T10 loot | `campo_treinamento::{loot_...}` | caixa no chão (`GAME-11`) — **não existe** |
| T11 refino | `refino_piedade.rs`, `config_gameplay.rs` | frame `SendItem` real |
| T12 quest | `integracao_campo_treinamento` | buff do Cabuncle, portão visível |
| T13 portões | `integracao_campo_treinamento` (portões) | porta como entidade visível |
| T14 loja | `tests/sistema/loja_viva.rs` | UI da loja no 7662 |
| T15 logout | `selchar.rs::logout_...` | — |
| T16 limites | `lotacao.rs`, `limite_ip.rs` | mensagem no cliente |
| T17 portal | `contrato_transporte`, `contrato_painel`, `npm test` (vitest) e `npm run test:e2e` em `wyd-portal/` | — |

## 4. O que o roteiro NÃO deve esperar funcionar (e por quê)

| Função | Estado | Issue |
|---|---|---|
| Caixa de item no chão + pickup | sem handler (`CreateItem`/`GetItem`/`DecayItem`) | `GAME-11` |
| Colisão de jogador | ausente no servidor | `DIV-10` / `PAR-MOV-02` |
| Regen de HP/MP e respawn de NPC | fórmula existe, não ligada ao tick | `N-4`, `N-5` |
| Trade jogador↔jogador | layout 156 B × 154 B sem captura | `CLIENT-10` |
| Party, guilda, chat de guilda, duelo | não implementados | `02-legacy-analysis` |
| PvP/PK/chaos | regra não implementada | 4.5 |
| Skill no fio, mix, baú, teleporte, clima | sem payload | 4.3, 5.6, 5.8, 3.6, 3.9 |
| Aplicar gema pelo cliente | só no core/quest, sem pacote | — (regra única em `wyd-sim`, ADR-0029) |

## 5. Como fechar o roteiro

1. Rode os blocos **na ordem**; cada um gera prints na pasta do §1.4.
2. Preencha `docs/rust/evidencias/manual-7662-<data>.md` copiando este roteiro em tabela:
   bloco → resultado (verde/vermelho/esperado-falhar) → arquivo de print/log → observação.
3. **Verde** só com print + log/consulta coerentes. Sem print, é “não executado”.
4. Falha esperada (§4) entra como confirmação do achado, com o número medido (ex.: dano por golpe).
5. Qualquer falha **fora** da lista §4 vira bug novo no roadmap com o print e a linha de log.

## 6. Verificação do roteiro (executada em 2026-11-14)

Tudo abaixo foi rodado **contra a stack real** neste host, com `client/` e banco `wyd` de
desenvolvimento. O que exige `WYD.exe` não foi executado (§0).

| Comando | Saída observada | Correção aplicada |
|---|---|---|
| `docker compose up -d --build --wait` | `exit 1`, `container wyd-mkdocs-1 is unhealthy` — o `mkdocs` ainda construía 256 páginas (~105 s) | §1.1 passou a usar `--wait-timeout 300`; reexecutado: `exit 0` e todos `healthy` |
| `curl :7600/health` | `{"ok":true,"database":"up","schema":50}` | — |
| `curl :3000/api/health` | `200` | — |
| `curl :80/serv00.htm` | corpo `1\n0` (contagem por canal), não HTML | §1.1 documenta o formato |
| `cargo run -p wyd-launcher -- check` (sem argumentos) | `exit 2` + uso da CLI | §1.2/T1 corrigidos para `check client client/manifest.txt` |
| `cargo run -p wyd-launcher -- check client client/manifest.txt` | `manifesto build 7662: 7129 íntegros, 2 com problema`, `exit 1`: `Config.bin: hash divergente`, `WYD.log: tamanho divergente` | T1 agora explica que são arquivos mutáveis do cliente; só outra divergência é falha real |
| `POST /admin/v1/accounts/create` com `x-wyd-actor: 1` | `x-wyd-actor inválido` | §1.3 corrigido para `ID:PAPEL` (`1:admin`) |
| mesmo POST com `x-wyd-actor: 1:admin` | `{"result":1,"account":{"id":664,…}}`; conta no banco | — |
| `sistema_personagens_crud` com `WYD_TEST_DSN` no banco vivo | `2 passed`; conta 665 com personagens 194–197, ouro 5.000.000, posição `2096,2096` | §1.3 ganhou o caminho sem GUI |
| consultas de conta/personagem | 1 conta, 4 personagens com os 14 campos | — |
| `character_items` | 12 itens no personagem 197 | coluna é `item_id`, não `item_index` |
| `character_quest` | `campo_de_treinamento`, `step = 1`, `progress = 0`, `completed_at` nulo | §1.4/T12 trocaram `stage/completed/reward_mask` por `step/progress/completed_at` |
| `refine_pity` | `0 rows` (personagem novo) | §1.4 explica que vazio é o esperado até uma falha de refino |
| `character_session` | 374 linhas de histórico; vivas = `released_at is null` | §1.4/T15 trocaram “tabela vazia” pelo filtro |
| `npc_definition` do Campo | 10 linhas (6 do dump + 4 criaturas `campo-*`); 6301–6306 removidas na `0051` | T7 passou a listar as coordenadas herdadas |
| `item_catalog` 412/413/451/452/453/481/652/682 | todos existem; 412 grafa `Poeira_de_Oriharucon` | §1.5 fixa a arma 5017 `Espada_Flamejante` (`req_level = 0`) e avisa do grafia |
| `curl :9898/metrics` | HTTP `000` — porta não publicada no host | §1.4/T16 passaram a usar `:9090/api/v1/query` (observado: séries `wyd_ticks_total`, `wyd_packets_rejected_total{canal,reason}`) |
| `cargo test --workspace --all-targets` | **851 passed, 0 failed, 1 ignored**, `exit 0` | — |
| sistema com `WYD_TEST_REQUIRED=1` | **300 passed, 0 failed**, 7 SKIPs de `WYD_LIVE_CPSOCK`, exit 0 | teste `contas_reais_listam_sem_segredo` corrigido (paginação) — falhava em banco com >200 contas |
| `mkdir -p tmp/manual-7662-2026-11-14` | exit 0 | — |
| `curl :80/health` (canal 1) | `{"ok":true,"database":"up","channels":2,"group":1,"channel":1,"online":0,"maintenance":false,"schema":50}` | T16 fixa a URL e a saída |
| `docker logs --since 5m wyd-server-canal-1-1` | linhas `event=tick ticks=… entities=6111 players=0` e `service=status` | — |
| `docker logs --since 5m wyd-gateway-1` | linhas `event=audit actor=0 role=admin action=catalog.list_items detail="3220 itens"` | — |
| `update accounts set pin_failures=0, pin_locked_until=null …` (T4) | `UPDATE 1`; releitura com `pin_failures = 0` e bloqueio nulo | — |
| `npm test` em `wyd-portal/` | 1ª execução: `29/30` arquivos + `Timeout waiting for worker to respond` (infra); 2ª: **30 arquivos, 163 testes, exit 0** | — (flake de worker, sem teste vermelho) |

Achado de higiene de teste: `contas_reais_listam_sem_segredo` lia só a primeira página de 200 contas
e assumia “banco descartável começa vazio”; com 313 contas acumuladas ele falhava sem haver bug de
produto. Agora pagina por `id` até achar a conta criada, mantendo a asserção de que a lista não expõe
segredo.
