> Pergunta: "o que o alvo `sistema_pg` prova contra Postgres real, e o que cada falha quer dizer?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../specs/capacidades.md` §4; `../specs/sistema.md`; `../../docs/guia-teste/testes-de-sistema.md`
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-24) — alvo executado inicialmente em 2026-11-24: **13 de 27** cenários verdes; 3 dependem de código de sistema que ainda não existe (§1.1). Reexecutado contra PG17 em 2026-11-25 e repetido no CI em 2026-11-27: **13/13** dos cenários implementados passaram (job `pg / schema` do run `45994297544`). `CAP-37` saiu deste alvo e está verde em `runner_pg`; `CAP-38` e `CAP-39` continuam sem implementação.

**Reorganização (2026-11-26):** o arquivo passou a `crates/wyd-db/tests/sistema/sistema_pg.rs`;
`cargo test --no-run` recompilou o alvo, mas a execução não foi repetida localmente (sem PG17 e
`WYD_DB_URL_TEST`); o job `pg` (`schema`) de `ci-db.yml` executou o alvo no PR #220 e passou.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre o alvo: `sistema_pg` está em `Cargo.toml` e em `tests/` desde
  2026-11-24, e 13 dos **27** cenários rodaram.
- **`H`** — dúvida aberta, dita e localizada em §6, com o dono que a fecha; **nenhuma** hoje — as duas que existiam fecharam em 2026-11-24 por `Q-BLK-24` = A.
- **`V`** — o que foi medido nesta árvore em 2026-11-25, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

**Alvo escrito e executado:** `sistema_pg` — arquivo `crates/wyd-db/tests/sistema/sistema_pg.rs`, declarado no
`Cargo.toml` do crate em 2026-11-24; 13 cenários passaram novamente em 2026-11-25 e no CI em 2026-11-27 (job `pg / schema` do run
`45994297544`). O nome do alvo é o de [../index.md](../index.md) §2.

O alvo de sistema: **27** capacidades que só existem contra PostgreSQL 17 real — transação, `CHECK`,
índice único parcial, FK, concorrência, ledger, pool, a série e as três medições de `RNF-DB-002`, `004` e `005`. Sem banco, ele **falha** nomeando
`WYD_DB_TEST_URL`; nunca passa em silêncio.

## 1. Capacidades cobertas

| `CAP` | Capacidade | Cenário |
|---|---|---|
| `CAP-09` | aplicar, reaplicar sem efeito e detectar drift no ledger | `apply_reapply_drift` |
| `CAP-10` | vaga de conta 1..5 e reutilização da fechada | `vaga_de_conta` |
| `CAP-11` | uma sessão viva por conta e por personagem | `sessao_unica` |
| `CAP-12` | saldo e moeda recusam fora do inteiro de 32 bits | `moeda_int32` |
| `CAP-13` | fechar pessoa anonimiza o e-mail e mantém unicidade | `anonimizacao` |
| `CAP-14` | cinco contas por pessoa **sem** barreira por pessoa ou IP | `cinco_contas_sem_barreira` |
| `CAP-15` | grant pendente impede apagar conta e personagem | `grant_pendente_impede` |
| `CAP-16` | um resgate diário por conta e dia | `resgate_diario` |
| `CAP-17` | quatro processos concorrentes terminam sem deadlock | `concorrencia_4` |
| `CAP-18` | a versão de conteúdo admite **uma** linha corrente e recusa a segunda | `boot_uma_versao` |
| `CAP-19` | o pool tem espera finita e reuso observável | `pool_espera_reuso` |
| `CAP-24` | login de conta aceita 1 a 16 caracteres e `login_fold` dobra caixa e borda | `login_tamanho_e_fold` |
| `CAP-25` | contador de falha de PIN e instante de bloqueio ficam na linha da conta e sobrevivem a conexão nova | `pin_falha_persistida` |
| `CAP-26` | cofre é da conta: FK recusa conta inexistente e apagar personagem não apaga o cofre | `cofre_da_conta` |
| `CAP-27` | bloqueio exige motivo não vazio, autor e instante, os três por `CHECK` | `bloqueio_com_motivo` |
| `CAP-28` | chave de idempotência repetida no donate não duplica o lançamento | `donate_idempotente` |
| `CAP-29` | token de reset só entra como sha256 hexadecimal de 64 caracteres | `token_como_hash` |
| `CAP-30` | papel de jogo é `player`, `moderator` ou `admin` | `papel_de_jogo_fechado` |
| `CAP-31` | senha de portal, senha de jogo e PIN só entram com o prefixo `$argon2id$` | `hash_argon2id` |
| `CAP-32` | bloquear conta não muda `game.account.status` | `bloqueio_nao_muda_status` |
| `CAP-33` | o limiar de PIN não vive no schema; contador negativo é recusado | `limiar_de_pin_e_politica` |
| `CAP-34` | autenticar no portal deixa sessões revogáveis e tentativa registrada | `autenticar_portal` |
| `CAP-35` | criar conta na vaga grava conta e carteira e recusa a sexta | `criar_conta_na_vaga` |
| `CAP-36` | abrir sessão de jogo grava `character_id` nulo e recusa canal inexistente | `abrir_sessao_de_jogo` |
| `CAP-37` | a segunda execução da série espera no advisory lock no máximo **10 s** | `espera_de_ddl` — provado em `runner_pg` (`alvos/runner-pg.md`), não por `psql` |
| `CAP-38` | o p95 da leitura de personagem é medido e registrado, sem limiar | `p95_leitura_personagem` |
| `CAP-39` | o tempo de boot até a primeira conexão aceita é medido e registrado | `tempo_de_boot` |

### 1.1 Execução de 2026-11-24

Contra base vazia migrada (`postgres:17`, 28 arquivos de `../../migrations/` aplicados como
`wyd_migrator`), `cargo test -p wyd-db --test sistema_pg -- --test-threads=1` com
`WYD_DB_TEST_CONTAINER` apontando o container: **13 passed; 0 failed; 0 ignored**. Um cenário falhou na
primeira passada (`CAP-36`, `ck_server_channel_jogavel` recusou o canal de fixture) e foi corrigido no
cenário, não no DDL; rodou de novo e passou.

| Resultado | Quais | Por quê |
|---|---|---|
| **verde no alvo `sistema_pg`** | `CAP-24`–`CAP-36` | o schema recusa e aceita o que a capacidade enuncia; 13/13 cenários implementados passaram em PG17 em 2026-11-25 |
| **sem execução pelo código de sistema** | `CAP-38`, `CAP-39` | `CAP-38`: falta consulta de personagem no caminho `repo`/`pool`; `CAP-39`: falta binário/boot instrumentado. `CAP-37` foi provado pelo `PgAdapter` real em `runner_pg` (2026-11-25) |

O harness chama `psql` no container por `docker exec`; ele prova o **schema**, não a porta do crate.
A fronteira está decidida em [`../index.md`](../index.md) §5: `psql` é o oráculo de schema e `sqlx` é
a porta do runner. O ciclo `up → down → up` da série foi provado por `psql` em PG17 (§1.2); a porta, o
ledger e o advisory lock são provados por `runner_pg` (`alvos/runner-pg.md`), que cobre `CAP-37`.

## 2. Cenários

| Cenário | Entrada | Resultado esperado | Falha esperada |
|---|---|---|---|
| `apply_reapply_drift` | banco vazio → provisionar → série → série de novo → alterar uma linha do ledger | segunda aplicação não escreve; drift é detectado e nomeado | `checksum divergente` ou `série já aplicada` |
| `vaga_de_conta` | cinco contas na mesma pessoa e uma sexta na vaga fechada | 1..5 aceitas; sexta recusada por vaga | `ck_account_slot_range` |
| `sessao_unica` | duas sessões vivas para a mesma conta, e duas para o mesmo personagem | a segunda recusada em cada caso | violação dos dois índices únicos parciais |
| `moeda_int32` | saldo em `2_147_483_648` e em `-1` conforme a regra | recusa com erro de `CHECK` | `ck_*_moeda` |
| `anonimizacao` | fechar pessoa e reabrir e-mail igual em outra pessoa | e-mail anonimizado; unicidade mantida | `ck_person_*` ou `ux_person_email` |
| `cinco_contas_sem_barreira` | cinco contas da mesma pessoa online e uma sexta de outro IP | as cinco abrem; nada bloqueia por pessoa ou IP | **nenhuma** — o esperado é não haver barreira |
| `grant_pendente_impede` | apagar conta e personagem com linha pendente em `ops.delivery_queue` | `DELETE` recusado | FK `RESTRICT` |
| `resgate_diario` | dois resgates da mesma conta no mesmo dia | o segundo recusado | índice único do resgate |
| `concorrencia_4` | quatro sessões aplicando escrita na mesma ordem trocada | todas terminam; sem deadlock | `deadlock detected` |
| `boot_uma_versao` | `INSERT` de uma segunda linha em `content.content_version` (`id = 2`) | a escrita é recusada; o boot encontra uma única versão | `23514` em `ck_content_version_linha_unica` |
| `pool_espera_reuso` | esgotar o pool e medir a espera até `acquire_timeout` | espera finita, erro de aquisição, conexão reusada depois | espera infinita |
| `login_tamanho_e_fold` | `INSERT` em `game.account` com `login` de 17 caracteres, de 16, e `' Bob '` | 17 recusado; 16 insere; `login_fold` de `' Bob '` é `'bob'` | `23514` em `ck_account_login_len`, ou fold que mantém caixa/borda |
| `pin_falha_persistida` | `UPDATE` de `pin_failure_count = 5` e `pin_locked_until = now() + interval '10 minutes'`; conexão nova | a linha devolve `5` e instante não nulo | estado só em processo, ou coluna ausente |
| `cofre_da_conta` | `INSERT` em `game.vault_item` com `account_id` inexistente; `DELETE` do personagem da conta com cofre | `23503` em `fk_vault_item_account`; o cofre permanece | FK para personagem, ou cofre apagado junto |
| `bloqueio_com_motivo` | três `INSERT` em `game.account_moderation`: motivo `''`, sem `blocked_by`, sem `blocked_at` | três `23514`, um por constraint | constraint ausente ou frouxa |
| `donate_idempotente` | dois `INSERT` em `portal.donate_credit` com a mesma `idempotency_key`; e `credits = 0` | segundo `23505` em `uq_donate_credit_idem`; `0` recusado por `ck_donate_credit_valor` | único ausente, ou valor não positivo aceito |
| `token_como_hash` | `INSERT` em `portal.person_token` com `token_hash = 'XYZ'`; e em `game.pin_reset` | `23514` em `ck_person_token_hash_forma`/`ck_pin_reset_token_forma`; 64 hexadecimais passam | forma não conferida |
| `papel_de_jogo_fechado` | `UPDATE game.account SET role = 'gm'`; depois `'player'` | `'gm'` recusado com `23514` em `ck_account_role`; `'player'` passa | `CHECK` ausente ou valor a mais |
| `hash_argon2id` | `INSERT` em `portal.person` e `game.account` com `password_hash = 'texto'`; depois com `$argon2id$...` | `23514` em `ck_person_senha_hasheada`/`ck_account_senha_hasheada`; o prefixo passa | hash em claro aceito |
| `bloqueio_nao_muda_status` | moderação `blocked = true` para conta `active`; `UPDATE ... status = 'suspended'` | `status` continua `active`; `'suspended'` recusado por `ck_account_status` | gatilho que muda `status`, ou `suspended` aceito |
| `limiar_de_pin_e_politica` | `pin_failure_count = -1`; e `= 6` com `pin_locked_until` preenchido | `-1` recusado por `ck_account_pin_falhas`; `6` aceito | `CHECK` que fixe o limiar de 5 no schema |
| `autenticar_portal` | duas sessões vivas para a mesma pessoa; `revoked_at` em uma; `ops.login_attempt` com `surface = 'portal'` e `result = 'bad_secret'` | as duas coexistem, uma revogada e a outra viva; a tentativa é gravada | índice único por pessoa, ou falta de `revoked_at` |
| `criar_conta_na_vaga` | conta e carteira na vaga 1; slots 1–5 ocupados e uma sexta; duas criações concorrentes na mesma vaga | conta e carteira na mesma vaga; sexta e concorrente recusadas em `ux_account_slot` | `23505` em `ux_account_login`, ou carteira faltando |
| `abrir_sessao_de_jogo` | `INSERT` em `game.session` com `character_id` nulo; e com canal inexistente | sessão aceita com `character_id` nulo; canal inexistente recusado em `fk_session_canal` | `character_id` obrigatório, ou FK ausente |
| `espera_de_ddl` | processo A segura o advisory lock da série; processo B aplica | B desiste em até **10 s** com erro nomeado, sem pendurar | espera infinita, ou sucesso silencioso |
| `p95_leitura_personagem` | fixture com volume declarado e 200 leituras da consulta de personagem | p95 registrado com data, volume e ambiente, **sem** afirmar limiar | publicar o número como limiar, ou trocar p95 por média |
| `tempo_de_boot` | processo novo, do início até o pool aceitar a primeira conexão | tempo registrado com data e ambiente, **sem** prazo | fixar prazo sem medição |

**`cinco_contas_sem_barreira` prova uma ausência** — e é o cenário mais fácil de escrever errado: se ele
falhar, o defeito é uma constraint ou gatilho a mais no banco, não dado de teste.

**Dois cenários provam estado, não política:** `pin_falha_persistida` e `limiar_de_pin_e_politica` escrevem
contador e instante na linha da conta e conferem que sobrevivem a uma conexão nova; o limiar de cinco falhas
em dez minutos é do código (`RN-DB-007`), e o schema só recusa contador negativo
(`ck_account_pin_falhas`). Dizer que o limiar é provado aqui seria mentira.

**`p95_leitura_personagem` e `tempo_de_boot` medem sem limiar:** o resultado é a série datada, nunca um teto —
limiar de latência e de boot não existe
(`../../docs/specs/requisitos-nao-funcionais.md` §4–§5). As **200** leituras são tamanho de amostra do harness,
não limiar.

## 3. Pré-requisito de ambiente

| Item | Regra |
|---|---|
| PostgreSQL | **17**, mesma major da imagem de produção (`../../docs/guia-administrador/instalacao.md` §1) |
| preparo | banco vazio → `provision` → série; nunca banco "já quente" |
| DSN | `WYD_DB_TEST_URL` (papel `wyd_migrator`); ausente, falha nomeando a variável |
| paralelismo | uma base por binário; dentro do binário, chave própria por teste |
| limpeza | o banco morre com a execução |

## 4. Como interpretar a falha

- **`permission denied` inesperado:** o cenário usou a DSN errada; fronteira de papel é
  `sistema_sessoes`.
- **`deadlock detected` em `concorrencia_4`:** ordem de aquisição divergente no código, não no teste.
- **Violação que não veio:** a constraint não existe ou o `CHECK` está mais frouxo que o DDL fechado.
- **Cenário de ausência que falha:** há barreira onde a regra diz que não deve haver (`RN-DB-003`).
- **Pool que não reusa:** conexão vazando — fechar antes de medir.

## 5. Tempo esperado

**Não há tempo medido.** Os **27** cenários contra banco real têm custo dominado pela subida e pela série, e
esta página não estima: o tempo entra quando houver execução registrada, com data e ambiente.

## 6. Dúvidas abertas

**Nenhuma em 2026-11-24.** As duas que esta página registrava fecharam pela decisão `Q-BLK-24` = A
(`../../pendencias-de-fechamento.md` §6.6.26), e ficam aqui para o rastro:

| Dúvida fechada | Como fechou |
|---|---|
| `CAP-17` usa quatro processos, mas qual tempo limite conta como falha? | **tempo não é a falha:** o que reprova é o `SQLSTATE` **`40P01`**, limiar **0** de `RNF-DB-006` (`../../docs/specs/requisitos-nao-funcionais.md` §6). O teto de tempo do harness segue sem número aprovado (`../../docs/guia-teste/testes-de-sistema.md` §"Tempo limite") |
| `CAP-18` precisa de duas `content_version` correntes para provar a ambiguidade; o DDL permite? | **não.** `0001` fecha a tabela em uma linha (`pk_content_version` + `ck_content_version_linha_unica CHECK (id = 1)`, prova 9 de `../../migrations/0001_bootstrap.md` §9). A entrada do cenário passou a ser a recusa de `id = 2`, e a propriedade ganhou requisito próprio: `RF-DB-017` |

**O que continua sem número:** o teto de tempo de execução de `concorrencia_4`, como parâmetro de harness.
Não é limiar de requisito, e a primeira execução o mede.

## 7. O que este alvo não prova

- **Forma da série e paridade de catálogo:** os dois alvos de contrato.
- **Privilégio por papel e leitura por view:** `sistema_sessoes`.
- **Anti-skip e DSN nomeada:** `sistema_stack_caps`.
- **Carga e capacidade:** não há volume nem limiar aprovado (`../../docs/planos/plano-de-testes.md` §6).
- **Regra de jogo:** `RF-DB-015` e `RF-DB-016` são de `wyd-server`.
