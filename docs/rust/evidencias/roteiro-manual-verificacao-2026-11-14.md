# Verificação do roteiro manual contra a stack real — 2026-11-14

> **Issue:** `QA-12` / `QA-13` (roteiro manual do cliente 7662)
> **Documento verificado:** [`docs/cliente/roteiro-teste-manual.md`](../../cliente/roteiro-teste-manual.md)
> **Ambiente:** Windows, `D:\dolorestec\wyd2`, banco `wyd` de desenvolvimento, `client/` local (não versionado).
> **O que NÃO foi feito:** nenhum passo de GUI do `WYD.exe` (login, seleção, cena, combate, loja,
> portal). Esses continuam dependendo do usuário e aparecem como tais no documento.

## 1. Objetivo

Provar que cada comando de preparação e cada consulta de observação do roteiro **executa** e devolve
saída não vazia contra a stack real; corrigir o documento onde ele estava errado (coluna, tabela,
porta, caminho, semântica). Não é prova de comportamento do cliente.

## 2. Preparação executada

| # | Comando | Resultado observado |
|---|---|---|
| 1 | `docker compose up -d --build --wait` | `exit 1` — `container wyd-mkdocs-1 is unhealthy`; o `mkdocs` ainda construía 256 páginas (~105 s) e o timeout padrão do `--wait` venceu antes |
| 2 | `docker compose up -d --build --wait --wait-timeout 300` | `exit 0`, todos os serviços `healthy`; `mkdocs` confirmado `healthy` depois |
| 3 | `docker ps --format '{{.Names}}\t{{.Status}}'` | 14 containers `Up` (db, gateway, portal, canal 1, canal 2, prometheus, grafana, loki, alloy, alertmanager, alert-sink, mkdocs, blackbox, node-exporter) |
| 4 | `curl -s http://127.0.0.1:7600/health` | `{"ok":true,"database":"up","schema":50}` |
| 5 | `curl -s http://127.0.0.1:3000/api/health` | HTTP `200` |
| 6 | `curl -s http://127.0.0.1/serv00.htm` | corpo `1\n0` — dois inteiros, jogadores por canal (não é HTML) |
| 7 | `curl -s http://127.0.0.1:9898/metrics` | HTTP `000`: a porta **não** é publicada no host; `docker port` mostra só 80/8281 |
| 8 | `curl -s 'http://127.0.0.1:9090/api/v1/query?query=wyd_ticks_total'` | `status":"success"`, séries com `canal="1"` e `canal="2"` |
| 9 | `curl -s 'http://127.0.0.1:9090/api/v1/query?query=wyd_packets_rejected_total'` | séries com rótulos `canal` e `reason` |
| 10 | `cargo run -p wyd-launcher -- check` | `exit 2` + uso da CLI: `check` exige `<dir-do-jogo> <manifesto>` |
| 11 | `cargo run -p wyd-launcher -- check client client/manifest.txt` | `manifesto build 7662: 7129 íntegros, 2 com problema` (`Config.bin: hash divergente`, `WYD.log: tamanho divergente`), `exit 1` |

## 3. Conta e personagem de teste

```text
POST /admin/v1/accounts/create  x-wyd-actor: 1        -> {"message":"x-wyd-actor inválido"}
POST /admin/v1/accounts/create  x-wyd-actor: 1:admin  -> {"result":1,"account":{"id":664,"login":"manual_teste_20261114","role":"player"}}
select id, login, role, pin_locked_until from accounts where login='manual_teste_20261114';
  664 | manual_teste_20261114 | player | (null)
```

Personagem sem cliente gráfico, pelo protocolo real do servidor:

```text
WYD_TEST_REQUIRED=1 WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd \
  cargo test -p wyd-server --test sistema_personagens_crud
  -> 2 passed; 0 failed  (9.57 s)

select id, name, slot, level, exp, gold, pos_x, pos_y, hp, mp from characters where account_id = 665;
  194 | K00665 | 0 | 1 | 0 | 5000000 | 2096 | 2096 | 83 | 46
  195 | F00665 | 1 | 1 | 0 | 5000000 | 2096 | 2096 | 61 | 68
  196 | B00665 | 2 | 1 | 0 | 5000000 | 2096 | 2096 | 71 | 57
  197 | I00665 | 3 | 1 | 0 | 5000000 | 2096 | 2096 | 77 | 61
```

## 4. Consultas de observação

| Consulta | Saída | Correção no documento |
|---|---|---|
| `accounts` | 1 linha, `pin_locked_until` nulo | — |
| `characters` (14 campos) | 4 linhas, todos os campos preenchidos | — |
| `character_items` | 12 linhas no personagem 197 (slots 0–6, 16–19, 79) | coluna é `item_id` (o roteiro dizia `item_index`) |
| `refine_pity` | `0 rows` | documentado: vazio até a primeira falha de refino |
| `character_quest` | `campo_de_treinamento`, `step = 1`, `progress = 0`, `completed_at` nulo | o roteiro pedia `stage/completed/reward_mask`; semântica real: `step = stage + 1`, `progress` = máscara |
| `character_session` | 374 linhas de histórico; vivas = `released_at is null` | o roteiro dizia “vazio após sair” |
| `audit_log` | 3 últimas linhas (`catalog.list_items`, ator `0:admin`) | — |
| `delivery_queue` | 2 linhas `kind=gold, status=claimed` com `idempotency_key` | — |
| `city_tax_ledger` | tabela existe (`to_regclass` não nulo) | — |
| `update accounts set pin_failures=0, pin_locked_until=null` (T4) | `UPDATE 1`; releitura zerada | — |
| `curl http://127.0.0.1/health` (canal 1) | `{"ok":true,"channels":2,"group":1,"channel":1,"maintenance":false,"schema":50}` | T16 passou a fixar a URL/saída |
| `docker logs --since 5m wyd-server-canal-1-1` / `wyd-gateway-1` | `event=tick ticks=19213 entities=6111` / `event=audit action=catalog.list_items` | — |

## 5. Dados conferidos no seed carregado

- `npc_definition` 6301–6310 devolveu 10 linhas com coordenadas **idênticas** ao T7
  (treinadores 2077/2017, 2144/1989, 2082/1966; chefe 2122/2041; Gremlin Decoy 2076/2022 etc.).
- `item_catalog`: 412 `Poeira_de_Oriharucon`, 413 `Poeira_de_Lactolerium`, 451/452/453 chaves,
  481/652/682 recompensas — todos existem. O roteiro passou a citar a arma **5017
  `Espada_Flamejante`** (`req_level = 0`) em vez de “qualquer arma”.
- Personagem novo já nasce com 5.000.000 de ouro (observado no banco), então “grantar ouro” é opcional.

## 6. Suítes automatizadas

```text
cargo test --workspace --all-targets
  -> 851 passed, 0 failed, 1 ignored, 76 alvos, exit 0

WYD_TEST_REQUIRED=1 WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd_auditoria \
  cargo test -p wyd-server -p wyd-admin --all-targets -j 1 --no-fail-fast
  -> 300 passed, 0 failed, 0 ignored, 7 SKIPs (WYD_LIVE_CPSOCK), exit 0
```

Portal (`wyd-portal/`): `npm test` (vitest) — primeira execução `29/30` arquivos com
`[vitest-pool]: Timeout waiting for worker to respond` (falha de infraestrutura do worker no
Windows, nenhum teste vermelho); reexecutado: **30 arquivos, 163 testes, exit 0**.

Falha real encontrada e corrigida no caminho: `contas_reais_listam_sem_segredo`
(`crates/wyd-admin/tests/contrato/operacao.rs`) lia apenas a primeira página de 200 contas e falhava
com `conta criada aparece na lista` num banco com 313 contas — sem bug de produto. O teste agora
pagina por `id` até encontrar a conta criada (limite de 100 páginas) e mantém a asserção de que a
lista não expõe segredo.

## 7. Testes citados na matriz (arquivo/função real)

Todos rodaram verdes nas duas suítes acima. Funções conferidas por nome:

| Bloco | Alvo | Função |
|---|---|---|
| T2 | `sistema_status_udp` | `probe_de_status_volta_igual_ao_enviado`, `datagrama_de_outro_tamanho_nao_recebe_resposta` |
| T3 | `sistema_login` | `senha_errada_e_recusada_sem_sessao`, `cinco_pins_errados_bloqueiam_a_conta` |
| T4 | `sistema_pin_troca` | `troca_de_pin_exige_o_atual_e_invalida_o_antigo` |
| T5 | `sistema_personagens_crud` | `cria_quatro_personagens_em_slots_distintos_sem_sobrescrever`, `cria_e_deleta_personagem_pelo_protocolo` |
| T6 | `sistema_entrada_mundo` | `entrar_e_sair_salva_posicao_e_vitais_no_banco` |
| T7 | `wyd-server` lib | `world::tests::plan_scene_emite_entrada_saida_e_respeita_o_teto` |
| T8 | `wyd-core` `integracao/autoridade` | `teleporte_forjado_nao_move_nem_publica`, `passo_dentro_do_orcamento_e_gravado_pelo_servidor`; colisão de jogador **não** tem teste (`DIV-10`) |
| T9 | `integracao_campo_treinamento`, `integracao_config_gameplay`, `wyd-sim` `golden/formulas` | `fluxo_completo_do_campo_entrega_as_quatro_recompensas`, `loot_le_a_tabela_do_snapshot_e_cada_item_tem_seu_roll` |
| T10 | `integracao_campo_treinamento` | `loot_empilha_ate_o_teto_e_ocupa_slot_livre`, `loot_sem_slot_nao_e_inventado` |
| T11 | `sistema_refino_piedade`, `integracao_config_gameplay` | `refino_le_a_sancrate_do_snapshot_em_cada_chamada`, `refino_respeita_tetos_do_pano_e_passo_especial` |
| T12/T13 | `integracao_campo_treinamento` | `portao_abre_com_o_ef_keyid_da_chave_e_fecha_por_tempo`, `os_tres_portoes_tem_ef_keyid_distintos_e_chaves_do_campo`, `recompensa_repetida_e_recusada` |
| T14 | `sistema_loja_viva` | alvo verde na suíte de sistema |
| T15 | `sistema_selchar` | `logout_explicito_fecha_a_conexao_e_libera_a_sessao` |
| T16 | `sistema_lotacao`, `sistema_limite_ip` | `segunda_conexao_do_mesmo_ip_e_recusada_e_volta_ao_sair` |
| T17 | `contrato_transporte`, `contrato_painel`, `wyd-portal` vitest | `contas_reais_listam_sem_segredo` (corrigido), `grant_item_enfileira_com_idempotencia`, 163 testes do portal |

## 8. O que continua sem prova

- Todo o `WYD.exe`: telas, mensagens, layout, render, dano exibido, caixa no chão — §0 e §6 do
  roteiro continuam valendo como “não executado”.
- `WYD_LIVE_CPSOCK`: os 7 SKIPs da suíte de sistema seguem sem execução multi-canal real.
- Integridade do pacote: `Config.bin` e `WYD.log` divergem porque o cliente os reescreve; um
  `client/` limpo daria `0 com problema`, mas esse estado não foi reproduzido nesta execução.
