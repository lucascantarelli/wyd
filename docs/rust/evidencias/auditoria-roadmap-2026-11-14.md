# Auditoria → roadmap e estado real da suíte (2026-11-14)

> Origem: pedido de iniciar as análises de `docs/audit/*`, conferir se as issues “resolvidas”
> realmente têm prova e reorganizar o roadmap pela ordem de paridade proposta em
> `docs/audit/17-priorities-and-roadmap.md`. Nada de produto foi implementado nesta entrega.

## 1. O que foi medido (não inferido)

| Verificação | Comando | Resultado |
|---|---|---|
| Formatação | `cargo fmt --all --check` | exit **0** |
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** (1 aviso pré-existente: `unused dependency tracing` em `wyd-core`) |
| Suíte completa | `cargo test --workspace --all-targets` | exit **0** — **847 passed, 0 failed, 1 ignored**, 76 binários de teste |
| Gate de issues | `node tools/content/check-issue-status.mjs` | **58/58** issues ativas, índice × painel batem |
| Consistência de docs | `node tools/dev/check-docs-consistency.mjs` | 9 crates, 50 migrations, 252 docs, 382 links |

Log bruto: `tmp/test-all-2026-11-14.log` (fora do git). Comparação com o registro anterior:
`docs/audit/20-validacao-2026-11-13.md` media 811 testes verdes; a diferença vem dos testes de
unidade acrescentados em `config/service`, `gameplay/service`, `cpsock/limits`, `logs` e
`boot/env` (ver `cobertura-2026-11-14.md`).

### Suíte de sistema com banco real

Banco descartável `wyd_auditoria` no Postgres da stack (50 migrations, 89 tabelas, criado com
`gateway --migrate` e DSN próprio):

```bash
WYD_TEST_REQUIRED=1 WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd_auditoria \
  cargo test -p wyd-server -p wyd-admin --all-targets -j 1 --no-fail-fast -- --nocapture
```

| Resultado | Valor |
|---|---|
| Binários | 40 |
| Testes | **299 passed, 0 failed, 0 ignored** |
| Exit | **0** |
| SKIPs | 7, todos `WYD_LIVE_CPSOCK ausente` (canal vivo; não é skip silencioso) |

Log bruto: `tmp/sistema-final-2026-11-14.log`. O banco `wyd_auditoria` **não foi apagado**
(regra do projeto: sem `DROP` sem pedido); é descartável e pode ser removido pelo dono.

### Falha real encontrada e corrigida nesta rodada

A primeira execução com banco real derrubou **2 testes** de `sistema_selchar`
(`bonus_de_atributo_e_skill_chegam_ao_cliente_e_ao_banco` e
`maestria_do_bonus_tipo_1_vai_ao_banco_e_volta_no_relogin`) com timeout de leitura. Causa: o
braço `_MSG_ApplyBonus` exigia `entity: Some(..)`, e o processo sem mundo (harness) fica com
`entity = None` — o pedido virava `reason=not_in_game` e o teste ficava esperando frame. Correção
em `cpsock/action.rs`: o padrão aceita `entity` opcional e só chama `refresh_actor` quando há
entidade. Depois: **9/9** em `sistema_selchar` e **299/299** na suíte de sistema completa.
É exatamente o tipo de regressão que a suíte *live* pega e a suíte unitária não.

### Leitura honesta do “tudo está falhando”

- **A suíte não está vermelha.** 847 testes passam no commit atual, incluindo contrato, golden,
> integração e unidade. O que a auditoria chama de “não funciona” é o eixo **S→C/cliente**
> (render, seleção, entrada no mundo, mensagens de recusa), que **nunca teve prova visual** e
> cujas issues sempre estiveram ⬜/🚧/🟡 — não ✅.
- **Testes que exigem ambiente retornam cedo sem env.** `WYD_TEST_DSN` e `WYD_LIVE_CPSOCK` não
> estavam definidos nesta rodada; os binários de sistema/live contam como “ok” por skip interno.
> Portanto: 847 verdes provam **código e contrato**, não provam banco real nem canal vivo. A
> prova com banco descartável + stack é `make sistema-test`/`QA-14`.
- **As issues “concluídas” foram rebaixadas antes.** `PROC-25` retirou os vereditos de
  `20-validacao-2026-11-13.md` e desmarcou os checkboxes do roadmap. O que faltava — e agora foi
  feito — era **abrir o trabalho que a auditoria propõe** e amarrá-lo ao índice.

## 2. Roadmap reorganizado pela auditoria

As 17 entregas sem issue correspondente viraram `AUD-01`…`AUD-17`, com objetivo, contexto,
passos, aceite e testes exigidos. As demais foram mapeadas para issues existentes.

| Item da auditoria (`17` §) | Entrega | Issue |
|---|---|---|
| 0.1 | Captura S→C do 7662 | `CLIENT-06` |
| 0.2, 0.3 | Frida/XTrap + dissector Wireshark | `AUD-01` |
| 0.4 | Direção de `CreateItem` no catálogo | `AUD-02` |
| 1.1 | Senha/recusa visível | `BUG-01` |
| 1.2, 1.3 | Offsets reais de `SELCHAR`/`MOB` | `AUD-03` |
| 1.4 | Colisão de jogador | `AUD-04` |
| 1.5 | Movimento no fio | `GAME-09` + `QA-13` |
| 1.6, 1.7 | Regen HP/MP e respawn de NPC | `AUD-05` |
| 1.8 | Cadência de ataque provada | `AUD-06` |
| 2.1 | Item no chão + pickup | `GAME-11` |
| 2.2 | Mover/equipar no fio | `GAME-05` |
| 2.3 | Loja de NPC completa | `AUD-07` |
| 2.4 | Trade atômica | `CLIENT-10` |
| 2.5 | Baú/armazém | `AUD-08` |
| 2.6 | Skill no fio | `AUD-09` |
| 2.7, 2.8 | Mix/compose e +12…+15/Anct | `AUD-10` |
| 3.1 | Chat/whisper | `GAME-25` |
| 3.2 | Party e EXP compartilhada | `AUD-11` |
| 3.3, 3.7 | Guilda e eventos | `GAME-24` |
| 3.4 | `ApplyBonus` no fio | `AUD-12` |
| 3.5 | Teleporte/portal | `AUD-13` |
| 3.6 | Quest genérica | `AUD-14` |
| 3.8 | PK/chaos e cor do nome | `AUD-15` |
| 4.1 | Escritor `.wyt` | `AUD-16` |
| 4.2 | Modo de vídeo/gerações | `CLIENT-12` |
| 4.3, 4.4 | Conversor de malha + editor de mapa | `AUD-17` (decompor antes) |
| 4.5 | Cliente independente | decisão pós-paridade, sem issue |

### Onde está no repositório

- Índice: `roadmap-rust/index.md` §“Roadmap de paridade — issues abertas pela auditoria”.
- Painel: `docs/rust/03-progresso.md` §“Roadmap de paridade” (status idêntico ao índice).
- Issues: `roadmap-rust/issues/AUD-01…AUD-17`.
- Auditoria: `docs/audit/17-priorities-and-roadmap.md` ganhou o ponteiro para o mapa.

## 3. Por que não há teste novo nesta entrega

Cada `AUD-NN` exige comportamento que **não existe** (loja no fio, baú, skill, party, portal,
quest, PK). Teste de issue inexistente vira teste de fachada. A regra do projeto é teste que
falha antes; abrir a issue com aceite e teste exigido é o passo correto antes de implementar.
O que já tem regra pronta e ganhou rastreio de teste no roadmap:

| Já implementado | Onde a regra vive | Teste que deve ser ligado ao fio |
|---|---|---|
| `ApplyBonus` (atributo, maestria, skill) | `character/crud.rs` | `AUD-12` (sistema loopback) |
| Refino/composição | `gameplay/service.rs`, `wyd-sim` | `AUD-10` (golden + sistema) |
| Colisão/posição autoritativa | `world::move_to` | `AUD-04` (fuzz de posição) |

## 4. Pendências declaradas

- Prova visual/live continua bloqueada por `CLIENT-06` (cliente 7662 sem input/render target).
- `make sistema-test` (banco descartável) e a suíte live **não** foram executados nesta rodada;
  sem eles, nenhuma issue de sistema deve ser marcada ✅.
- Itens 4.x ficam fora da fila até P1 fechar, como manda a própria auditoria.
