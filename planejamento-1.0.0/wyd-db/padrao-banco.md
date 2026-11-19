# Padrão de modelagem do banco — `wyd-db` 1.0.0

> **Estado: proposta normativa.** Nada aqui está implementado. Este documento **decide a forma**;
> o conteúdo de cada domínio continua nos documentos de sistema; do rascunho de identidade restam só
> os requisitos (`rascunho-accounts.md`, histórico).
>
> **Aprovado em 2026-11-17 (dono):** `P-01` — 4 schemas `content`/`game`/`portal`/`ops` — e `P-02` —
> tabela no singular. São restrição fechada: a série nasce assim e não há volta a `public` nem a
> plural. O resto do padrão segue **proposta** até a série existir e passar o corte vertical.
>
> **O que ele resolve:** a divergência entre dois modelos-alvo que hoje convivem no repositório —
> o dossiê (`.agents/prompts/análises/wyd-db/database/09`, 4 schemas, 31 arquivos, ~128 tabelas) e o
> rascunho de identidade (então com 1 schema `public`, 50 arquivos, 90 tabelas — hoje reduzido a
> histórico de requisitos). Este padrão fica com
> o **melhor dos dois**: os 4 schemas do dossiê e o DDL de identidade do rascunho (índice total,
> tombstone, vaga 1..5, papéis separados, suspensão em colunas).
>
> **O que ele não decide:** regra de produto (transferência de conta, cadastro dentro do jogo,
> loja/cash). Essas perguntas continuam no `lacunas.md` e mudam nomes de tabela, não o padrão.
>
> **Relação com o resto:** `plano-migrations.md` (a série do zero) assina este padrão. Onde o
> qualquer documento de planejamento divergir de nomes ou de schema, **vale este documento** — a
> dívida de atualização dos demais está listada em §12.

---

## 0. Regra de ouro

**O banco é lido por três consumidores com poderes diferentes** — servidor de jogo, portal e
operação/BI. O padrão existe para que essa diferença seja aplicada **pelo banco** (schema + role +
constraint), não por disciplina de código. Toda decisão abaixo serve a uma destas quatro perguntas:

1. O dado errado **pode existir**? Se não, é `NOT NULL`/`CHECK`/índice único.
2. Quem **pode escrever** nisto? Se não é todo mundo, é schema + `GRANT`.
3. Isso vai **crescer para sempre**? Se sim, tem retenção e, no limite, partição.
4. Isso é **regra de jogo** ou **regra de armazenamento**? Regra de jogo não entra em trigger.

**Enunciação (2026-11-18):** as quatro perguntas passaram a ser enunciadas na forma que o revisor usa —
pergunta, o que a resposta vira e onde a regra mora — em `docs/arquitetura/padroes-postgres.md` §1. Aqui
fica o **porquê** delas: três consumidores com poderes diferentes, e a decisão de que a diferença é
aplicada pelo banco, não por disciplina de código. Pergunta nova entra **aqui**, e a linha da página sai
daqui — não o contrário. Enquanto as duas existirem, mudança altera as duas na mesma entrega.

---

## 1. Decisões estruturais

| # | Decisão | Por quê |
|---|---|---|
| **P-01** | **4 schemas por domínio**: `content`, `game`, `portal`, `ops`. Nada de tabela em `public`. | `pg_dump -n game` não leva PII; `GRANT` por domínio vira fronteira executável (portal não escreve estado vivo); nomes curtos sem prefixo. FKs e transações atravessam schemas no mesmo banco, então o custo é só disciplina. `public` fica vazio e sem `CREATE` para `PUBLIC`. |
| **P-02** | **Nome de tabela no singular, `snake_case`**: `game.account`, `game.character`, `portal.person`. | É o nome da entidade no código Rust (`GameAccount`, `Person`), evita plurais irregulares (`person`/`people`), e o schema já diz a coleção. A série é reescrita inteira: a inconsistência com os 87 nomes atuais (plurais) morre agora, não depois. |
| **P-03** | **PK `BIGINT GENERATED ALWAYS AS IDENTITY`** em toda entidade; `PRIMARY KEY` nomeada (`pk_<tabela>`). | 8 bytes, inserção na borda direita do índice, sem `SERIAL` (não é padrão SQL e permite escrita manual do id). `GENERATED ALWAYS` fecha o bug clássico de sequence atrasada. |
| **P-04** | **PK composta** em tabela filha/junção (`game.character_item (character_id, slot)`, `game.guild_member (guild_id, character_id)`). Sem `id` sintético onde a chave natural é a identidade. | O par já é único e é o acesso; um `id` extra só pagaria índice redundante. |
| **P-05** | **Sem `UUID` na 1.0.** Id interno é `bigint`. Se um dia houver id público opaco, entra **coluna separada** (`public_id`), não troca de PK. | Escritor único (o banco), nenhuma geração no cliente. UUIDv7 resolveria a localidade, mas é 2× o tamanho em toda FK e o PG17 do compose não tem `uuidv7()` nativo. |
| **P-06** | **`TIMESTAMPTZ` sempre**; instante é `now()` do servidor, nunca do cliente. Proibido `timestamp` (sem fuso) e `time` para evento. | `timestamp` guarda "uma foto de relógio" sem fuso; `timestamptz` guarda um ponto no tempo. É a regra explícita do wiki oficial do Postgres ("Don't Do This"). |
| **P-07** | **Dinheiro em inteiro na menor unidade** (`coin BIGINT`, `gold BIGINT`, `cents BIGINT`); taxa em **bps inteiro** (`rate_bps`). Proibido `money`, `real`, `double precision` para valor. `NUMERIC` só para grandeza exata não monetária. | `money` depende de `lc_monetary`; `float` não soma exato. `Coin` já é `int` no fio (`STRUCT_MOB.Coin@28`) — o inteiro é o tipo do domínio, não uma otimização. |
| **P-08** | **Conjunto fechado = `TEXT` + `CHECK` nomeado**. Sem `ENUM` nativo. Conjunto **editável pelo negócio** = tabela de lookup com FK (`item_kind`, `game_system`). | `ENUM` não tem `DROP VALUE` (remover exige recriar o tipo e reescrever toda coluna, sob `ACCESS EXCLUSIVE`); `CHECK` é troca de constraint; lookup é `INSERT`. O conjunto decide, não o gosto. |
| **P-09** | **`JSONB` só para dado aberto/UNVERIFIED**, nunca para campo que se filtra ou ordena. O que é consultado é coluna tipada. | `jsonb` sem GIN varre; GIN em payload grande é caro de manter. `generator_data`, `extras`, `raw` são exemplos legítimos. |
| **P-10** | **`INET` para endereço IP**, `TEXT` só para exibição. | `INET` valida na escrita, ocupa menos e responde consulta de sub-rede (`<<`), que é exatamente o caso de auditoria de multibox por IP/hora. |
| **P-11** | **Coluna gerada** para chave derivada (`email_fold`, `login_fold`, `name_fold`), com função `IMMUTABLE`. Nunca derivar no app e confiar. | A dobra é uma só, no banco, indexável; dois escritores não podem divergir. Já provado em `content_fold`. |
| **P-12** | **`NOT NULL` por padrão**; toda nulidade tem motivo escrito na nota da coluna do catálogo. | A maioria das colunas de um bom desenho é `NOT NULL`; nulo sem motivo é regra de negócio escondida. |
| **P-13** | **Toda constraint nomeada**: `pk_`, `fk_`, `ck_`, `uq_`. Nenhuma constraint anônima. | Nome anônimo trunca em 63 bytes, colide e não aparece legível no erro que o jogador/staff vê. |
| **P-14** | **Unicidade condicional = índice único parcial** (`WHERE released_at IS NULL`, `WHERE status <> 'closed'`). Unicidade com nulo intencional = `NULLS NOT DISTINCT`. | Constraint não tem `WHERE`; índice parcial é o único jeito de dizer "um vivo por conta" no banco. `NULLS NOT DISTINCT` (PG15+) evita o truque do `coalesce`. |
| **P-15** | **Toda FK tem índice no lado filho**, salvo lookup somente-leitura. `ON DELETE` é escolha explícita: `RESTRICT` para dinheiro/auditoria, `CASCADE` só para filho possuído. | O Postgres indexa o lado **pai**, não o filho: sem índice, apagar/alterar o pai varre a tabela filha. |
| **P-16** | **Trigger só para invariante mecânica** — hoje: `updated_at`. Nunca regra de negócio, nunca auditoria por trigger. Preferir permissão (`REVOKE UPDATE`) a trigger de imutabilidade. | Trigger é invisível no código, difícil de testar e de depurar; `GRANT` é verificável e aparece no erro. |
| **P-17** | **View para leitura entre domínios** (`game.v_live_session`), vivendo no schema de quem lê. Sem `materialized view` até haver medição. | O portal precisa de presença e não deve escrever nem conhecer as tabelas internas do jogo; a view é o contrato. |
| **P-18** | **Papéis separados + `ALTER DEFAULT PRIVILEGES` por schema**; aplicação nunca conecta como dono. Sem RLS na 1.0. | Fronteira de escrita executável sem custo de RLS em servidor de jogo (um "tenant" só). RLS volta se houver multi-servidor por banco. |
| **P-19** | **Migration é arquivo imutável**, numerado, `up`/`down`, sem `ALTER TABLE` no `up`, um domínio por arquivo, seed em arquivo próprio e depois do DDL do domínio. | O histórico é o que explica o schema; editar arquivo aplicado é a mentira que quebra o ledger. `ALTER` no `up` é desenho corrigido por cima — já é reprovado pelo parser do catálogo. |
| **P-20** | **Sem soft-delete genérico.** Estado é explícito: `status` + carimbo coerente (`closed_at`), `deleted_at` só onde o jogo exige (personagem). | `deleted_at` universal contamina todo `WHERE`, todo índice e toda FK. O domínio já sabe quais linhas morrem. |
| **P-21** | **Toda tabela de estado vivo que cresce tem retenção declarada**; partição só acima de ~10 M linhas, por tempo (`RANGE (occurred_at)`). | Partição com 374 linhas é custo operacional sem ganho. Auditoria e sessão são os candidatos futuros. |
| **P-22** | **PII mora só em `portal`** (`person`, `person_profile`, `person_consent`). Jogo referencia `person_id`, nunca copia e-mail. | Backup do jogo sem PII, retenção e base legal separadas, e uma única fonte para o dado pessoal. |

---

## 2. Schemas e fronteiras

| Schema | Dono | Escrita | Leitura | Retenção |
|---|---|---|---|---|
| `content` | curadoria/seed | só migration/seed; runtime nunca | jogo (boot) e portal (exibição) | permanente, versionado por `content_version` |
| `game` | servidor de jogo | `wyd_game` | `wyd_game`; portal só por view | estado de jogador: enquanto a conta existir |
| `portal` | portal/BFF | `wyd_portal` | `wyd_portal`; jogo só por view de identidade | PII: prazo declarado por tabela (`person_consent`) |
| `ops` | operação | `wyd_migrator`, drenos, staff via portal | `wyd_read` | auditoria 2 anos; login 90 dias (proposta) |

Regras de fronteira:

- **Jogo não escreve em `portal`** e **portal não escreve em `game`** — o banco recusa por `GRANT`,
  não por revisão de código.
- Portal lê presença por `game.v_live_session`; jogo lê identidade por `portal.v_person_identity`
  (id, papel, status — **sem e-mail**).
- `public` existe vazio: sem tabela de domínio; o `CREATE` de `PUBLIC` ali é revogado no
  provisionamento, não na série (`src/provision/spec.md` §4).

---

## 3. Convenções de nome

| Objeto | Padrão | Exemplo |
|---|---|---|
| schema | domínio, singular | `game` |
| tabela | entidade, singular | `game.account` |
| coluna | `snake_case`, sem abreviação | `pin_locked_until` |
| PK | `pk_<tabela>` | `pk_account` |
| FK | `fk_<tabela>_<coluna>` | `fk_account_person` |
| CHECK | `ck_<tabela>_<regra>` | `ck_account_slot_range` |
| UNIQUE (constraint) | `uq_<tabela>_<colunas>` | `uq_skill_catalog_index` |
| índice | `ix_<tabela>_<colunas>[_<condição>]` | `ix_session_ip_opened` |
| índice único parcial | `ux_<tabela>_<regra>` | `ux_session_viva_por_conta` |
| view | `v_<assunto>` no schema de quem lê | `game.v_live_session` |
| função | `fn_<verbo>` | `ops.fn_set_updated_at` |
| trigger | `trg_<tabela>_<evento>` | `trg_person_set_updated_at` |
| coluna de instante | sufixo `_at` (`TIMESTAMPTZ`) | `closed_at` |
| coluna de chave estrangeira | sufixo `_id` | `account_id` |
| chave de busca derivada | sufixo `_fold` | `login_fold` |
| segredo | sufixo `_hash` (nunca o valor) | `password_hash` |
| taxa | sufixo `_bps` (inteiro) | `rate_bps` |
| contador | sufixo `_count` | `failed_login_count` |

Limite: 63 bytes. Se o nome passar, corte o sufixo de colunas — nunca o prefixo do tipo.

---

## 4. Tipos por intenção

| Intenção | Tipo | Proibido |
|---|---|---|
| identificador interno | `BIGINT GENERATED ALWAYS AS IDENTITY` | `SERIAL`, `BIGSERIAL` |
| instante | `TIMESTAMPTZ NOT NULL DEFAULT now()` | `timestamp`, `time` |
| data de calendário | `DATE` | texto |
| dinheiro/saldo | `BIGINT` (menor unidade) + `CHECK` de faixa | `money`, `real`, `float8` |
| taxa/percentual | `INTEGER` em bps | `numeric` fracionário solto |
| quantidade curta (slot, nível) | `SMALLINT` + `CHECK` | `INTEGER` sem faixa |
| texto curto (login, nome) | `TEXT` + `CHECK (char_length(...) BETWEEN ...)` | `VARCHAR(n)` (limite não é tipo), `CHAR(n)` |
| texto livre | `TEXT` | — |
| booleano de estado único | `BOOLEAN NOT NULL DEFAULT false` | `CHAR(1)`, `INT 0/1` |
| conjunto fechado | `TEXT NOT NULL` + `CHECK (col IN (...))` | `ENUM` |
| conjunto editável | tabela + `FK` | `CHECK` com lista que o negócio muda |
| dado aberto | `JSONB` | `JSON` (sem índice), `TEXT` com JSON dentro |
| binário | `BYTEA` | base64 em `TEXT` |
| endereço IP | `INET` | `TEXT` sem `CHECK` |
| hash de senha/PIN | `TEXT` + `CHECK (... LIKE '$argon2id$%')` | hash em `BYTEA` sem prefixo verificável |
| token | `TEXT` com `CHECK (token_hash ~ '^[0-9a-f]{64}$')` | token em claro |

**Segredo com limite de fio:** hash não prova comprimento. O limite do protocolo (senha **12 bytes**,
PIN **6 bytes**, login **16 bytes** — `crates/wyd-protocol/src/payload/auth.rs`) é imposto por
**tipo de domínio na borda** (`GamePassword`, `NumericPin`, `GameLogin`), e o banco só guarda o hash.
Aceitar senha de 40 caracteres no banco é aceitar uma senha que o cliente 7662 nunca digita.

**Enunciação (2026-11-18):** o **critério** de escolha de tipo — as três perguntas que decidem — passou
a ser enunciado em `docs/arquitetura/padroes-postgres.md` §2, que aponta para cá em vez de reproduzir
esta tabela. Ela continua sendo a **lista fechada** intenção → tipo, com os `P-xx` de §1, e a casa do
valor que a página não repete: o limite de fio do parágrafo acima. Intenção nova entra **aqui**, com `P`
próprio, e não na página. Enquanto as duas existirem, mudança de decisão altera as duas na mesma
entrega; divergência é defeito a corrigir na página.

---

## 5. Integridade

1. **Estado e carimbo andam juntos.** `(status = 'closed') = (closed_at IS NOT NULL)`; suspensão exige
   motivo **e autor** e instante.
2. **Um vivo por chave** é índice único parcial, nunca `COUNT` no app.
3. **Recurso finito é vaga.** `slot BETWEEN 1 AND 5` + `ux_account_slot WHERE status <> 'closed'` —
   o limite morre no banco, inclusive em criação concorrente.
4. **Unicidade que não depende de estado** é índice **total** (e-mail, login), com anonimização
   explícita onde a linha morre (tombstone), não filtro de índice.
5. **Coerência entre tabelas irmãs** (bloqueio × moderação) é decisão de **um** dono: ou o estado é
   coluna, ou é tabela — nunca os dois. `game.account.status` guarda ciclo de vida
   (`active`/`closed`); **bloqueio disciplinar é `game.account_moderation`**.
6. **FK com `MATCH FULL`** só quando a chave é composta e parcialmente nula; preferir `NOT NULL`.
7. **Dinheiro nunca em `CASCADE`**: apagar conta não apaga razão; `RESTRICT` e fechamento explícito.

**Enunciação (2026-11-18):** as sete regras passaram a ser enunciadas em
`docs/arquitetura/integridade-e-constraints.md` — a política de FK, as famílias de `CHECK` e a
unicidade que delas derivam —, que aponta para cá em vez de reproduzir esta lista. As sete continuam
sendo a **lista fechada** (`P-08`, `P-14`, `P-15`, `P-20`), e vale a relação declarada em §4: regra
nova entra aqui, e as duas mudam na mesma entrega.

---

## 6. Índices

- Um índice por FK no lado filho (P-15).
- Índice parcial para consulta quente de subconjunto (`WHERE released_at IS NULL`, `WHERE blocked`).
- Composto na ordem **igualdade → faixa** (`(ip, opened_at DESC)`), não o contrário.
- Nenhum índice "preventivo": índice sem consulta medida é escrita mais lenta. Medir com
  `pg_stat_user_indexes` (`idx_scan = 0`) e remover.
- Sem BRIN/partição abaixo de ~10 M linhas; a maior tabela atual tem 6.706 linhas.
- `CONCURRENTLY` só existe para banco em produção; aqui (`do zero`, sem produção) o índice nasce no
  `CREATE TABLE`.

**Enunciação (2026-11-18):** o **critério** de índice — invariante × desempenho — está em
`docs/arquitetura/padroes-postgres.md` §5, que aponta para cá em vez de reproduzir estas regras. A
lista acima continua sendo a **lista fechada** (`P-14`, `P-15`, `P-21`), e vale a relação declarada em
§4: regra nova entra aqui, e as duas mudam na mesma entrega.

---

## 7. Triggers, funções e views na 1.0

| Objeto | Onde | Papel |
|---|---|---|
| `ops.fn_set_updated_at()` + `trg_*_set_updated_at` | `0001` | única trigger de negócio permitida: manter `updated_at` |
| `content.fn_fold(text)` (`IMMUTABLE`) | `0013` | normalização de busca usada em coluna gerada |
| `game.v_live_session` | `0029` | presença para portal/status, sem acesso às tabelas internas |
| `portal.v_person_identity` | `0029` | id/papel/status da pessoa para o jogo, **sem e-mail** |

Adicionar trigger/function/view exige: (a) linha nesta tabela, (b) teste que prova o efeito,
(c) entrada no catálogo gerado. Trigger sem essas três não entra.

---

## 8. Papéis e permissões

**Papéis** ficam fora das migrations (`db/roles.sql`, gerado por `wyd-db provision render` e aplicado
por `provision apply` uma vez por banco — papel é do cluster e não vai no `pg_dump`). **Schemas,
`GRANT` e `ALTER DEFAULT PRIVILEGES` dos quatro schemas ficam em `0001_bootstrap.sql`**, porque são
objetos que ele cria e precisam ser versionados com ele; o estado de `public`, que já existe antes da
série, vai no mesmo artefato (`src/provision/spec.md` §4). Ordem do ambiente: `provision render` →
`provision apply` → migrations.

Os cinco papéis são `wyd_owner` (dono dos schemas), `wyd_migrator` (roda a série), `wyd_game`,
`wyd_portal` e `wyd_read` (um por fronteira); atributos de login e membership são de
`src/provision/spec.md`. O DDL do
`db/roles.sql` **não se copia aqui**: ele é **gerado** por `wyd-db provision render` a partir da fonte
única descrita em `src/provision/spec.md` (**planejamento**:
`planejamento-1.0.0/wyd-db/src/provision/spec.md`), que também é dona do golden byte a byte, dos
invariantes de idempotência. Remoção de papel é ato manual auditado, fora daquela página.
Provisionamento (`render|plan|apply|check`) e por que papel fica fora da série: ADR-DB-0001. Senha e
`pg_hba` não entram no artefato (`spec.md` §7).
Escrever o bloco `CREATE ROLE` à mão é divergência: o gate de CI compara `db/roles.sql` com `render`.

O DDL de schema e de permissão **não se copia aqui**: o dono do texto dos quatro schemas é
`ddl-confirmado.md` §3 e a página que o executa é `0001-bootstrap.md` §4 (efeitos tabelados em §7);
o do estado de `public` é `src/provision/spec.md` §4, aplicado por `provision apply`. O que este padrão
fixa é a **fronteira**, em prosa:

- os quatro schemas (`content`, `game`, `portal`, `ops`) pertencem a `wyd_owner`; `public` fica sem
  escrita de runtime, com uma única exceção declarada — a do ledger do runner, cujo nome e forma são de
  `src/migrate/spec.md` §3, cuja localização é a exceção de `ddl-confirmado.md` §3 e cujo privilégio de
  schema é de ambiente (`src/provision/spec.md` §4);
- `USAGE` no schema é **atravessar**; o direito de ler/escrever tabela vem dos *default privileges*, e os
  dois são necessários. `content` → `wyd_game`, `wyd_portal`, `wyd_read`; `game` → `wyd_game`,
  `wyd_read`; `portal` → `wyd_portal`, `wyd_read`; `ops` → `wyd_read` **mais** `wyd_game` e
  `wyd_portal`, que ali só têm `USAGE` — o suficiente para o `INSERT` append-only de `0002` valer, e
  **nenhum** `SELECT`, `UPDATE` ou `DELETE` de tabela;
- **leitura cruzada só por view**: `portal` não tem `USAGE` em `game` nem `game` em `portal`, e as views
  de `0029` nascem no schema de quem lê (`P-17`);
- tabela criada por `wyd_migrator` herda o privilégio do schema: escrita de runtime em `game` só para
  `wyd_game`, em `portal` só para `wyd_portal`, `SELECT` em `content` e `ops` — e **nenhuma** escrita de
  runtime em `content`.

Append-only é por **permissão**, não por trigger: `ops.audit_log` e `ops.login_attempt` ganham `INSERT`
explícito para `wyd_game`/`wyd_portal` no arquivo que os cria (`0002`), nunca `UPDATE`/`DELETE`. RLS não
entra na 1.0 (um servidor, um tenant); volta se houver multi-servidor no mesmo banco.

O formato do ator de `ops.audit_log` está **decidido** (`M18-A`, 2026-11-17): `actor_kind` +
`actor_id` + `actor_ref`, 3 CHECKs de coerência e **sem FK, por desenho** — auditoria sobrevive ao ator
e nunca bloqueia nem cascateia. O DDL, o índice e o porquê das alternativas rejeitadas têm um dono só:
`lacunas.md` §2.3. Nomear a coluna aqui sem repetir a decisão é deliberado.

**Enunciação (2026-11-18):** a tabela de **quem pode o quê** — os cinco papéis, com o que cada um pode
e não pode — passou a ser enunciada em `docs/arquitetura/seguranca.md`, na mesma página que enuncia o
hash, o que nunca é gravado, a auditoria e a ameaça × controle. Aqui fica o **modelo de permissão** que
aquela tabela não explica: `USAGE` × *default privileges*, leitura cruzada só por view, append-only por
permissão e não por trigger, a exceção declarada de `public` (o ledger) e a decisão de RLS fora da 1.0.
Papel novo, ou mudança de poder de papel existente, entra **aqui**, com decisão registrada, e a linha da
página sai daqui. O DDL do `db/roles.sql` continua sendo **gerado** de `src/provision/spec.md` — nenhuma
das duas o redigita. Enquanto as duas existirem, mudança altera as duas na mesma entrega.

---

## 9. Migrations

A série não se redigita aqui. As duas donas:

- **`plano-migrations.md` §2** — o **contrato** da série: nomenclatura e contiguidade, marcadores, o
  que o `up` não pode conter, um domínio por arquivo, seeds e `SCHEMA_VERSION`. O formato mecânico ele
  delega a `src/migrate/spec.md` §2.
- **`docs/arquitetura/migracoes.md` §2** — o **porquê** de cada uma, regra por regra; o contrato de
  execução é `src/migrate/spec.md`.

A única coisa que mora aqui é a **operação futura**, que nenhuma das duas carrega:

- em produção, se e quando existir, mudança de schema segue `expand → migrate → contract` — adiciona
  compatível, preenche, só depois remove — com **um** runner, cujo lock e ledger são de
  `src/migrate/spec.md` §5 e §3. Nesta série (sem produção) o arquivo nasce na forma final de uma vez,
  que é `P-19`.

**Enunciação (2026-11-18):** a lista copiada que ficava aqui foi **removida** — o contrato é que só a
dona redigita. Regra nova de série entra em `plano-migrations.md` §2; o porquê entra em `migracoes.md`
§2; e a linha de `expand`/`contract` fica aqui até existir produção ou até
`migrations/serie-e-regras.md` nascer — a página nova herda `plano-migrations.md` §2 **e esta linha**,
que não tem outra dona. Este ponteiro troca de destino então, e não recupera a lista.

---

## 10. Testes e gates

| Camada | Prova |
|---|---|
| contrato (sem banco) | série contígua; toda tabela derrubada no `down` do próprio arquivo; nenhuma FK para arquivo posterior; seed só em tabela já criada; **toda constraint nomeada**; nenhum tipo proibido (§4) |
| catálogo | `catalogo.rs` compara documento gerado ↔ migrations; `banco-catalogo.md` inclui schema, view, função e trigger |
| sistema (PG real) | `migrate-cycle` up → down → up com contagens idênticas e **0 tabela** ao fim; `verify-schema` por schema; cada `CHECK`/índice único tem um teste que tenta violá-lo e recebe o erro esperado |
| fronteira | `wyd_portal` recebe `permission denied` ao escrever em `game`; `wyd_game` não lê `portal.person` cru |

Regra: **constraint sem teste é comentário.** Cada invariante de §5 ganha um teste de violação.

---

## 11. Antipadrões proibidos

`schema public` como depósito · `SERIAL/BIGSERIAL` · `timestamp` sem fuso · `money`/`float` para valor
· `ENUM` nativo para status de produto · `VARCHAR(n)` como regra de negócio · `jsonb` para filtro
recorrente · `text` para IP · trigger com regra de negócio · auditoria por trigger · constraint ou
índice anônimo · FK sem índice no filho · `ON DELETE CASCADE` em dinheiro/auditoria · duas fontes
para o mesmo estado (status + boolean de bloqueio) · `deleted_at` universal · editar migration
aplicada · `ALTER TABLE` no `up` · seed de correção como migration · contagem de tabelas como meta ·
soft-delete como sinônimo de "não apagar".

A página `docs/arquitetura/padroes-postgres.md` §7 agrupa estes antipadrões em quatro famílias de
defeito — dado mal tipado, regra invisível, histórico mentiroso, duplicação de estado — para o revisor
reconhecer o sintoma no diff; ela declara a lista fechada aqui e não a reproduz. Incluir ou retirar
antipadrão continua sendo decisão desta seção.

---

## 12. Dívida documental que este padrão cria (a pagar na mesma entrega)

| Arquivo | O que fica desatualizado |
|---|---|
| `rascunho-accounts.md` | **pago em 2026-11-17:** reduzido a histórico de requisitos; sem DDL, sem lista de arquivos, sem `SCHEMA_VERSION` |
| `lacunas.md` | `B2` (lista nominal, que aponta `plano-migrations.md` §3) e `M20` (medição depois da série); o número planejado da série não mora aqui — o dono é `plano-migrations.md` §3 |
| `wyd-db-1.0.0.md` | **pago em 2026-11-18:** a árvore de §4 e o §6.15 descrevem a série nova (29 arquivos) e os 29 slugs (`P8`, fechado em 2026-11-18); resta a escrita das páginas (onda 5) |
| `docs/arquitetura/modelo-banco.md` | ER e tabela por tabela continuam na forma antiga (`public`, 53 arquivos) |
| `docs/arquitetura/banco-catalogo.md` | gerado; só volta a bater depois do gerador schema-aware |

Nenhum desses é editado por este documento: ele **decide** e aponta a dívida.

---

## 13. Fontes

1. PostgreSQL 18 — Constraintes e `NULLS NOT DISTINCT`: <https://www.postgresql.org/docs/current/ddl-constraints.html>
2. PostgreSQL 18 — Índices únicos e parciais: <https://www.postgresql.org/docs/current/indexes-unique.html> · <https://www.postgresql.org/docs/current/indexes-partial.html>
3. PostgreSQL 18 — Schemas: <https://www.postgresql.org/docs/current/ddl-schemas.html>
4. PostgreSQL 18 — `ALTER DEFAULT PRIVILEGES`: <https://www.postgresql.org/docs/current/sql-alterdefaultprivileges.html>
5. PostgreSQL 18 — Identity columns: <https://www.postgresql.org/docs/current/ddl-identity-columns.html>
6. PostgreSQL Wiki — *Don't Do This* (timestamp sem fuso): <https://wiki.postgresql.org/wiki/Don%27t_Do_This>
7. PostgreSQL 18 — Tipos numéricos e monetários: <https://www.postgresql.org/docs/current/datatype-numeric.html> · <https://www.postgresql.org/docs/current/datatype-money.html>
8. PostgreSQL 18 — UUID (`uuidv7` é PG18): <https://www.postgresql.org/docs/current/functions-uuid.html>
9. PG17 — FK não indexa o lado filho: <https://www.postgresql.org/docs/17/ddl-constraints.html> · <https://backstage.payfit.com/of-postgresql-indexes-and-foreign-keys>
10. Enum × CHECK × lookup: <https://www.cybertec-postgresql.com/en/lookup-table-or-enum-type/> · <https://schemity.com/blog/postgres-enum-vs-check-constraint-vs-lookup-table>
11. PK: bigint identity × UUIDv7: <https://stitchdb.com/blog/postgres-uuid-vs-bigint> · <https://www.dbgorilla.com/learn/postgres/are-uuid-primary-keys-still-bad-uuidv7>
12. Nomenclatura e constraints nomeadas: <https://www.bytebase.com/blog/postgres-sql-review-guide> · <https://dbschema.com/blog/design/database-naming-conventions>
13. Schema por domínio em monólito modular: <https://milanjovanovic.tech/blog/how-to-keep-your-data-boundaries-intact-in-a-modular-monolith>
14. Auditoria append-only e partição por tempo: <https://supabase.com/blog/postgres-audit> · <https://oleg.is/blog/postgres-audit-event-partitioning>
15. Migrations imutáveis, checksum, advisory lock, expand/contract: <https://docs.bytebase.com/gitops/best-practices/migration-guidelines> · <https://ttoss.dev/docs/engineering/guidelines/postgres-migrations>
16. Dinheiro em inteiro na menor unidade: <https://www.crunchydata.com/blog/working-with-money-in-postgres>
