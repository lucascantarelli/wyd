> Pergunta: "qual é a ordem da série 1.0.0 e como o runner registra, não repete e recusa uma aplicação divergente?"
> Público: Desenvolvedor do `wyd-db` que vai implementar o runner e quem revisa a série
> Dono: equipe do `wyd-db`
> Base: `../src/migrate/spec.md` §2–§8 e §11; `../plano-migrations.md` §2–§3; `../migrations-checklist.md` §5; `../docs/arquitetura/adr/adr-db-0002-runner-de-migrations-proprio.md`; `../docs/arquitetura/migracoes.md`
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25)

# Série 1.0.0 — ordem, ledger, idempotência e drift

Esta é a nota transversal do **ato 2** do `wyd-db`. Ela reúne, numa leitura só, as quatro regras que
amarram a série ao runner: **ordem**, **ledger**, **idempotência** e **drift**. Não é uma 35ª migration,
não entra na contagem de 34 arquivos/109 tabelas e não é um ato de aceite de página numerada; por isso
seu nome não começa por `NNNN`.

A página não substitui os donos: a lista de arquivos e a numeração continuam sendo de
`../plano-migrations.md` §3; o contrato de descoberta, ledger, checksum, transação e comandos continua
sendo de `../src/migrate/spec.md`. Se esta nota divergir de qualquer um dos dois, vale o dono, e a
divergência é defeito desta página. O runner foi implementado em **2026-11-25** (`crates/wyd-db/src/migrate/`) e o **ato 3** foi fechado
no escopo de biblioteca/portas falsas; a série completa de 34 arquivos ainda **não foi materializada**
(há 28 arquivos parciais) e `SCHEMA_VERSION` continua **53**. A verificação contra PostgreSQL 17 real e
o boot ainda são o próximo ato; o registro exato está em §8 e em `../src/migrate/spec.md` §15.

## 1. Ordem: uma sequência só, crescente e contígua

A série planejada é **34 arquivos**, de `0001` a `0034`, sem buraco e sem duplicata. A ordem abaixo é
uma projeção da tabela de `../plano-migrations.md` §3 em 2026-11-25; ela existe para o implementador
enxergar o fluxo inteiro, não para criar uma segunda autoridade.

```text
0001-bootstrap.sql
0002-ops-audit.sql
0003-portal-person.sql
0004-portal-access.sql
0005-game-account.sql
0006-game-character.sql
0007-game-session.sql
0008-game-skills.sql
0009-game-inventory.sql
0010-game-guild.sql
0011-game-economy.sql
0012-ops-delivery.sql
0013-content-taxonomy.sql
0014-content-item.sql
0015-content-skill.sql
0016-seed-taxonomy.sql
0017-seed-item-catalog.sql
0018-seed-skill-catalog.sql
0019-content-world.sql
0020-seed-world.sql
0021-content-rules.sql
0022-content-overrides.sql
0023-seed-rules.sql
0024-content-maps.sql
0025-content-combat.sql
0026-content-quests-events.sql
0027-seed-quests-events.sql
0028-game-quests-events.sql
0029-views.sql
0030-portal-content.sql
0031-portal-mfa.sql
0032-admin-grants.sql
0033-ops-idempotency.sql
0034-portal-market.sql
```

Os seis arquivos de seed (`0016`, `0017`, `0018`, `0020`, `0023` e `0027`) fazem parte da mesma
sequência; não são uma segunda série nem um passo opcional. Eles continuam bloqueados por dado de
origem não verificado (`../plano-migrations.md` §4 e `../migrations-checklist.md` §3), mas a posição
deles na ordem já está fixada.

### 1.1 Como a ordem é descoberta e validada

O adapter de filesystem lista `*.sql` e devolve os arquivos na ordem do diretório. Quem manda é o
`MigrationSet`, que:

1. ordena pelo `stem` com quatro dígitos fixos — nessa forma, ordem lexicográfica é ordem numérica;
2. exige que a primeira versão seja `1`;
3. exige que cada versão seguinte seja `anterior + 1`;
4. recusa duas entradas com a mesma versão ou o mesmo slug;
5. recusa série vazia;
6. calcula o checksum do conteúdo cru de cada arquivo uma única vez.

Falha em qualquer regra é `SetInvalid` (código de saída `4`), não "aplicar o que der". A ordem de
iteração é parte do contrato: `iter()` devolve versões crescentes; não existe um `HashMap` público que
permita ao chamador escolher outra ordem.

### 1.2 O que a ordem protege

A sequência não é alfabética por acaso. Ela segue dependências reais:

| Bloco | Versões | Razão da posição |
|---|---|---|
| terreno e auditoria | `0001`–`0002` | `0001` cria os quatro schemas, a função de `updated_at` e os privilégios padrão; `0002` cria a auditoria antes de qualquer tabela que audite, sem FK que bloqueie escrita |
| identidade | `0003`–`0007` | `portal.person` e o acesso vêm antes de `game.account`; o jogo referencia a pessoa, não copia e-mail |
| estado do jogador e entrega | `0008`–`0012` | habilidades, inventário, guilda, economia e fila dependem da conta/personagem já existentes |
| conteúdo e seeds | `0013`–`0027` | o conteúdo não tem FK de saída para `portal`/`game`; cada seed vem imediatamente depois do DDL que o recebe |
| estado de jogo que lê conteúdo | `0028` | quests, eventos e arena têm FK para tabelas de `content`, por isso fecham depois dele |
| views e lotes finais | `0029`–`0034` | as views precisam de tudo que leem; `0030`–`0033` não têm FK de saída e `0034` depende de `0006`/`0009`; entram no fim, sem renumerar os anteriores |

Consequência para quem escreve: não se cria FK para arquivo posterior, não se parte uma tabela entre
dois arquivos e não se usa `ALTER TABLE` para "completar" um arquivo anterior. A série é de instalação,
não de evolução (`../plano-migrations.md` §2).

### 1.3 Sentido de execução

`apply` percorre as versões **pendentes em ordem crescente**. `down --to <v>` percorre as aplicadas em
**ordem decrescente** e para em `v`, que permanece aplicada. `down --to 0` desce até o fim, uma
transação por arquivo, apagando a linha correspondente do ledger; é o reset de desenvolvimento, nunca
`DROP SCHEMA … CASCADE`.

`SCHEMA_VERSION` só passa a **34** na troca de série; enquanto a série herdada roda, a constante
continua **53**. O runner não muda a constante por conta própria.

## 2. Ledger: `public.schema_migrations`

O ledger é criado **pelo runner**, não por um arquivo da série. Ele precisa existir antes de `0001`,
mas `0001` não o cria: o privilégio de `wyd_migrator` em `public` e o estado desse schema são de
ambiente, entregues por `provision` (`../src/provision/spec.md` §4 e `ADR-DB-0001`). O nome e a
localização em `public` são exceção declarada em `../ddl-confirmado.md` §3.

### 2.1 Forma exata

```sql
CREATE TABLE public.schema_migrations (
    version    INTEGER     PRIMARY KEY,
    slug       TEXT        NOT NULL,
    checksum   TEXT        NOT NULL,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT ck_schema_migrations_version  CHECK (version > 0),
    CONSTRAINT ck_schema_migrations_checksum CHECK (checksum ~ '^[0-9a-f]{64}$')
);
```

| Coluna | Papel | Invariante |
|---|---|---|
| `version` | identidade da migration | inteiro, chave primária, maior que zero; duas linhas para a mesma versão são impossíveis |
| `slug` | nome lógico do arquivo aplicado | não nulo; renomear arquivo já aplicado é drift, não atualização de linha |
| `checksum` | conteúdo do arquivo | não nulo; 64 hexadecimais minúsculos, `sha256` do arquivo normalizado |
| `applied_at` | quando o banco registrou | `now()` do servidor, não relógio do processo; não entra na decisão de plano |

O ledger **não** guarda o `down`, a duração nem o texto do SQL. O `down` não é executado em produção;
duração é observabilidade (span), não estado. O checksum cobre o arquivo inteiro, inclusive o `down`,
porque alterar a reversão também muda o comportamento de um arquivo já aplicado.

### 2.2 `prepare`: criar uma vez, validar sempre

`prepare` roda **sob o advisory lock**, antes do primeiro `BEGIN` de uma aplicação:

- se `public.schema_migrations` não existe, executa o `CREATE TABLE` acima;
- se existe, lê o catálogo e compara a **forma**: as quatro colunas (nome, tipo canônico e nulidade), a
  chave primária em `version` e as duas `CHECK` **pelos nomes** `ck_schema_migrations_version` e
  `ck_schema_migrations_checksum`;
- qualquer diferença → `LedgerError::Shape`, com detalhe legível, e código de saída `2`.

Não existe `CREATE TABLE IF NOT EXISTS` no caminho. `IF NOT EXISTS` esconderia uma tabela errada por
baixo: uma coluna faltando, um tipo trocado ou uma `CHECK` removida passariam calados e só apareceriam
depois como drift de conteúdo. A validação de forma é do `prepare`; a porta de leitura `Ledger` só lê e
não tenta consertar o ledger.

`prepare` é **idempotente**: chamá-lo duas vezes deixa uma única tabela e a segunda chamada é um no-op
verificado. Quem chama é `apply` ou `down`; `status` e `apply --dry-run` não chamam `prepare`.

### 2.3 Tabela ausente é ledger vazio

`status` e `apply --dry-run` prometem **não escrever**. Num banco virgem, `public.schema_migrations`
ainda não existe; a porta de leitura devolve `Ok(vec![])`, e o plano lista as 34 versões como
pendentes. Isso não é erro de forma e não cria tabela.

A distinção é deliberada:

| Situação | Resultado | Código |
|---|---|---:|
| tabela ausente, em `status`/`dry-run` | ledger vazio; todas pendentes; nada é criado | `0` no `status` sem problema |
| tabela existente com coluna ausente ou valor inválido | `Shape`, nomeando o defeito | `2` |
| tabela existente, forma válida, sem linhas | ledger vazio legítimo | `0` ou `4` conforme o plano |
| tabela existente, forma divergente em `apply`/`down` | `Shape` no `prepare`, antes de executar arquivo | `2` |

### 2.4 Quando a linha é gravada

A linha e o schema do arquivo commitam **na mesma transação**:

```text
BEGIN
  SET LOCAL search_path = ''
  <up do arquivo>
  INSERT INTO public.schema_migrations (version, slug, checksum)
COMMIT
```

Não existe "schema aplicado sem linha" nem "linha sem schema". Se qualquer instrução falhar, o
`ROLLBACK` desfaz o arquivo inteiro, a linha não é gravada e os arquivos seguintes não são tentados. A
mensagem leva a versão e a posição do erro; o ponto de parada continua verdadeiro.

## 3. Idempotência: o runner garante uma vez, o SQL não finge que já rodou

"Não repetir" aqui tem um significado operacional: **executar o mesmo comando duas vezes, com o mesmo
disco e o mesmo ledger, não altera o schema nem o ledger na segunda vez**. Não significa que todo
`CREATE TABLE` da série tenha `IF NOT EXISTS`; a série é de instalação e deve falhar alto se encontrar
um objeto que não criou.

| Camada | O que é idempotente | O que impede a repetição |
|---|---|---|
| `prepare` | criar o ledger ou validar a forma | segunda chamada não altera a tabela; divergência vira `Shape` |
| `apply` | aplicar somente as versões pendentes | `plan` compara `MigrationSet` com o ledger; ledger completo devolve `UpToDate` e nenhum `BEGIN` |
| arquivo individual | **não** é idempotente por SQL | o runner não o executa de novo se a versão já tem linha; `version` é PK e um segundo `INSERT` seria `23505` |
| falha no meio | o arquivo falho volta inteiro | `ROLLBACK` da transação; a próxima execução retoma da mesma versão |
| `down` | desfazer uma vez e apagar a linha | `forget(version)` na mesma transação; repetir `down --to` para um alvo já alcançado não desce abaixo dele |
| concorrência | um processo aplica por vez | advisory lock de sessão com chave própria e espera limitada; o segundo processo, ao entrar, relê o ledger e não reaplica |

Consequências que o implementador não pode "melhorar":

- **`apply` não reexecuta arquivo aplicado.** Se o ledger tem a versão e o checksum confere, ela é
  `skipped`; não há `DELETE` da linha para forçar uma reaplicação.
- **`apply` não usa `--force` para drift.** `--force` é trava do binário para destinos destrutivos de
  `down`, não uma forma de ignorar checksum; o contrato da biblioteca não tem esse campo.
- **O SQL da série não recebe `IF NOT EXISTS` para compensar duplicação.** Um objeto preexistente é
  erro de ambiente ou de série, e o `up` deve falhar; esconder isso tornaria o ledger uma ficção.
- **`status` e `dry-run` não criam ledger nem "preparam" o banco.** O primeiro `apply` é quem cria a
  tabela, sob lock.
- **Uma falha não deixa meia migration.** O arquivo e sua linha são atômicos; o retry é seguro a partir
  da versão que falhou, sem "limpar" o schema à mão.

O ciclo de desenvolvimento `up → down → up` é a prova de idempotência do reset: `down --to 0` deixa o
ledger vazio e os quatro schemas ausentes; o `apply` seguinte reaplica as 34 do zero. O reset não
substitui banco descartável em CI e não toca papéis do cluster.

## 4. Drift: o disco e o ledger não concordam

Drift é qualquer divergência entre o que o disco oferece e o que o ledger registra. O runner não
"corrige", não "reaplica por cima" e não escolhe o arquivo mais novo: ele **para** e lista todos os
achados.

### 4.1 As três causas

| `DriftKind` | O que aconteceu | Como o plano detecta |
|---|---|---|
| `Checksum` | o conteúdo do arquivo mudou depois de aplicado | a versão existe no ledger, mas o `checksum` do disco é diferente |
| `Slug` | o arquivo foi renomeado depois de aplicado | o checksum confere, mas o `slug` do caminho não é o do ledger |
| `MissingFile` | o ledger tem uma versão que não existe mais no disco | a versão aplicada não aparece no `MigrationSet` |

O plano percorre a série em ordem crescente, compara primeiro o checksum e depois o slug, e acrescenta
as versões do ledger que não têm arquivo. Devolve **todos** os drifts, ordenados por versão; parar no
primeiro esconderia o segundo e obrigaria a rodar o comando N vezes.

**Drift vence pendência.** Se houver um arquivo alterado e três pendentes, o resultado é
`Plan::Drift`, não `Plan::Apply`. Aplicar por cima de conteúdo divergente é exatamente o que o ledger
existe para impedir.

### 4.2 Checksum: arquivo inteiro, com EOL normalizado

O checksum é `sha256` do conteúdo **normalizado**, nesta ordem:

1. todo `CRLF` vira `LF`;
2. **um** `\n` final, se existir, é removido — `strip_suffix('\n')` uma vez, nunca em laço;
3. o hash cobre o arquivo inteiro: cabeçalho, `up` e `down`.

O repositório fixa `*.sql text eol=lf` no `.gitattributes`. Sem a normalização, um clone no Windows com
`autocrlf` produziria checksum diferente para o mesmo commit e o runner acusaria um drift que não
existe. Dois `\n` no fim são conteúdo e continuam cobertos; um só é a quebra que o editor acrescenta.

O valor gravado é o `Display` do `Checksum`: 64 hexadecimais minúsculos, o mesmo texto que a `CHECK`
aceita.

### 4.3 O que o drift bloqueia

| Comando | Efeito do drift | Código |
|---|---|---:|
| `status` | lista as versões com `drift`; não escreve | `0` |
| `status --check` | devolve `MigrateError::Drift` e falha | `4` |
| `apply` | falha **antes** de qualquer `BEGIN`; nenhum arquivo é aplicado | `4` |
| `apply --dry-run` | mostra o plano com os achados; não pega lock nem escreve; quem decide o código é o binário, como em `status` | — |
| `down` | calcula o plano e recusa antes de desfazer; um arquivo faltando é `MissingFile` | `4` |

`--force` não muda nenhuma linha desta tabela. Ele pertence ao `down` para destinos não descartáveis,
não é uma válvula para drift.

### 4.4 Como sair de um drift

| Achado | Correção |
|---|---|
| `Checksum` | restaurar **exatamente** o arquivo do commit que foi aplicado; qualquer alteração intencional é arquivo novo, nunca edição do aplicado |
| `Slug` | restaurar o nome original do arquivo; renomear de volta é a correção, não atualizar a linha do ledger |
| `MissingFile` | restaurar o `.sql` da versão que o ledger registra, com o mesmo conteúdo/checksum |
| ledger com forma errada | parar e corrigir o ambiente/ledger com operação explícita; `prepare` não "conserta" e `Shape` não é drift de conteúdo |
| suspeita de linha manual errada | não usar `UPDATE`/`DELETE` no ledger para "fazer passar"; o ledger é a fonte de verdade e a intervenção precisa ser registrada |

O caminho normal é **restaurar o arquivo**. Reescrever o passado para o checksum bater é a falha que o
desenho quer tornar impossível; se a mudança é desejada, ela entra em uma versão nova da série, nunca
num arquivo já aplicado.

## 5. Sequência de comando e pré-condições

O contrato completo está em `../src/migrate/spec.md` §7–§8. Para relacionar ordem, ledger e drift:

```text
source.load()
  → MigrationSet::build()          # ordem, contiguidade, duplicata, checksum
  → server_info()                  # papel atual e versão do servidor
  → preflight                      # current_user == wyd_migrator; PG 17
  → status / dry-run?              # sem lock, sem prepare; ledger ausente = []
  → advisory lock                  # apply/down
  → prepare()                      # cria ou valida o ledger
  → plan(set, applied)             # UpToDate | Apply(pendentes) | Drift(todos)
  → executa                        # apply crescente; down decrescente
```

| Comando | Lock | Escreve | Ledger ausente |
|---|---|---|---|
| `status` | não | não | lê como vazio; todas pendentes |
| `status --check` | não | não | falha só se houver drift/série inválida; ausência não é erro |
| `apply --dry-run` | não | não | lê como vazio; todas pendentes |
| `apply` | sim | sim | `prepare` cria o ledger e aplica as pendentes |
| `down --to <v>` / `down --to 0` | sim | sim | `prepare` valida/cria; depois desce em ordem inversa |

O preflight é estrito: papel diferente de `wyd_migrator` é `WrongRole` (código `2`); servidor fora da
major 17 é `WrongServer`; diretório ausente/ilegível é `Source` e diretório vazio é `SetInvalid`. O
opt-out de superusuário é lido pelo binário, nunca pelo módulo, e é logado como aviso.

O lock é de **sessão** (`pg_advisory_lock`), com chave própria `0x5759_4404` e espera limitada (padrão
10 s). Não é lock de transação: a sessão atravessa uma transação por arquivo. Estourou a espera →
`LockBusy` (código `3`), com a instrução de verificar `pg_locks`; não há espera infinita no boot.

## 6. Critérios de prova

O runner está implementado; esta nota só estará cumprida quando os testes de contrato e de sistema
demonstrarem, no mínimo:

1. `0001`, `0002`, `0004` com buraco é `SetInvalid`; versão duplicada e slug duplicado também.
2. Banco sem ledger: `status` lista as 34 pendentes, não cria `public.schema_migrations`.
3. Ledger completo: `apply` é `UpToDate`, não emite `BEGIN` e não altera o schema.
4. Ledger parcial: `apply` executa somente as pendentes, em ordem crescente.
5. Falha no segundo arquivo: o segundo não commita, o terceiro não é tentado e o ledger continua com o
   primeiro.
6. Um byte alterado no `up` ou no `down` de um arquivo aplicado produz `Drift(Checksum)`; renomear o
   arquivo produz `Drift(Slug)`; apagar o arquivo produz `Drift(MissingFile)`.
7. Drift + pendência produz `Drift`, e `apply` falha antes de qualquer `BEGIN`; `status --check` sai `4`.
8. `down --to 1` desfaz em ordem decrescente e deixa só a versão 1; `down --to 0` deixa os quatro
   schemas ausentes e o ledger vazio; o `apply` seguinte reaplica as 34.
9. Duas execuções concorrentes de `apply`: uma aplica; a outra espera o lock e termina sem reaplicar.
10. `prepare` duas vezes deixa uma tabela; ledger com coluna a mais ou `CHECK` removida é `Shape`.

Esses critérios são de `../src/migrate/spec.md` §13; esta página não antecipa a implementação nem
autoriza alterar o desenho para "facilitar" o teste.

## 7. O que esta página não decide

- **Não é a 35ª migration** e não entra em `migrations-checklist.md` §2, na contagem de 34 arquivos,
  nos 109 tabelas ou nos 141 atos.
- **Não é dona da ordem**: a lista canônica, a numeração e o escopo de cada arquivo são de
  `../plano-migrations.md` §3; aqui há uma leitura consolidada para o runner.
- **Não é dona do contrato do runner**: assinaturas, variantes de erro, forma do ledger, checksum,
  lock, transação e códigos de saída são de `../src/migrate/spec.md` e das páginas de `src/migrate/`.
- **Não decide DDL, seed, permissão ou dados de conteúdo**: cada página numerada e o `ddl-confirmado.md`
  continuam donos do texto; os seeds continuam bloqueados por curadoria.
- **Não decide a troca de série**: `SCHEMA_VERSION` continua `53` até a troca; o runner novo existe no
  código, mas a série 1.0.0 ainda não foi materializada e o boot ainda não o usa.
- **Estado da implementação (2026-11-25)**: `src/migrate/` existe com parser, `MigrationSet`, `plan`,
  portas, adapter PostgreSQL, `FsSource` e runner; os testes de unidade e de contrato passaram. Os
  testes de sistema contra PG17, o binário/CLI e a troca de `SCHEMA_VERSION` continuam pendentes. A
  série materializada em `crates/wyd-db/migrations/` ainda tem 28 arquivos, nomes com hífen e somente
  `up`; por isso o parser a recusa por `MissingDown` — completar os 34 arquivos com `down` é ato da
  série, não deste runner, e não altera a ordem.

## 8. Ato 3 — fechamento verificado (2026-11-25)

**Concluído no escopo de biblioteca e portas falsas.** O ato 3 entregou o runner de `src/migrate/`,
o parser, o ledger, o plano, os adapters e os testes de unidade/contrato. Ele **não** é a conclusão do
ato de sistema contra PostgreSQL 17: nenhum teste de `tests/sistema_pg.rs` foi executado nesta rodada,
porque `WYD_DB_TEST_CONTAINER` não está definido e não há container PG17 migrado disponível.

| Comando executado | Resultado observado |
|---|---|
| `cargo test --manifest-path crates/wyd-db/Cargo.toml --lib` | `test result: ok. 63 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s` (exit 0) |
| `cargo check --manifest-path crates/wyd-db/Cargo.toml --all-targets` | concluído sem erro (exit 0) |
| `cargo clippy --manifest-path crates/wyd-db/Cargo.toml --all-targets -- -D warnings` | concluído sem aviso (exit 0) |
| `cargo fmt --manifest-path crates/wyd-db/Cargo.toml -- --check` | concluído sem diferença (exit 0) |

**O que os 63 testes cobrem:** unidade de `Version`, `Slug`, `Checksum`, `Sql`, `File`, `MigrationSet`,
`Config` e `plan`; contrato com `FakeSource`/`FakeLedger`/`FakeTransaction` para status, apply, dry-run,
drift, down (até versão e até vazio), falha no meio, papel inválido e lock ocupado. Eles provam o
runner **contra portas falsas**, não o PostgreSQL, o binário, o boot nem a série real.

**Cobertura não executada:** `cargo test --manifest-path crates/wyd-db/Cargo.toml --test sistema_pg`
(exige `WYD_DB_TEST_CONTAINER` e PostgreSQL 17), o gate de documentação em container e qualquer teste
do binário/CLI. A cobertura desta nota é explicitamente **lib/fakes**, não de sistema com PostgreSQL.

**Série intocada:** `crates/wyd-db/migrations/` tem **28** arquivos `.sql`, **28** com `-- migrate:up` e
**0** com `-- migrate:down`; a lista ordenada de nomes tem SHA-256
`8afaa10dd845f7e1d2251950e6234a534ace2a69068152bb4385ec612ff5c103` e o diretório não aparece em
`git diff` nem em `git status --short -- crates/wyd-db/migrations`. Nenhum SQL foi escrito, apagado ou
renomeado nesta rodada.

**Comando da medição:** `find crates/wyd-db/migrations -maxdepth 1 -type f -name '*.sql' -printf '%f\n' | sort | sha256sum`; contagem por `grep -R -l '^-- migrate:up$'` e `'^-- migrate:down$'`.

## 9. Brief de decisão — a série sem `down`

O bloqueio medido é objetivo: o contrato de §1 e `../src/migrate/spec.md` §2 exige um `down` não vazio
por arquivo, mas os 28 `.sql` materializados têm só `up`. O parser os recusa com `MissingDown`; enquanto
isso não for decidido, o runner não aplica a série existente, não executa o ciclo `up → down → up` e
não prova `CAP-01`–`CAP-07`/`CAP-37` contra a série real. **Nenhuma das opções abaixo foi implementada.**

| Opção | O que destrava | O que arrisca em produção | Custo aproximado |
|---|---|---|---|
| **A — forward-only declarado** — aceitar `down` ausente apenas com um marcador explícito de migração irreversível (por arquivo ou declaração da série) | o runner aceita os 28 arquivos e a aplicação/`CAP-37` pode andar; não exige escrever 28 rollbacks | perde o rollback dessas versões: a correção passa a ser para frente ou restore de backup; muda `ADR-DB-0002`/spec §2 e remove a prova `down_completo`; risco de incidente maior | contrato + parser + testes + 28 marcações (ou uma declaração de série): **1–2 sessões** (estimativa `H`, não medida) |
| **B — escrever `down` real para os 28** — derivar cada reversão da página da migration, revisar e provar no PG17 | mantém o contrato e o reset; destrava a série, `down --to`, o ciclo de idempotência e os testes de sistema; nenhuma exceção permanente | um `down` errado pode destruir dado se for executado; exige revisão por arquivo e ambiente descartável; não há produção hoje, então o risco é de implementação, não de incidente imediato | 28 blocos SQL + 28 verificações + testes de sistema: **2–4 sessões** (estimativa `H`, não medida) |
| **C — baseline** — marcar os 28 como já aplicados no ledger/manifesto e exigir `down` só dos arquivos novos | evita reexecutar a série já materializada e separa o passado do próximo arquivo; preserva o contrato para o futuro | o baseline pode declarar “schema atual” sem provar a forma/checksum; esconde drift e não cria rollback para o passado; ainda exige uma exceção para os arquivos legados sem `down` | mecanismo de baseline + conferência dos 28 checksums/schema: **1 sessão + testes** (estimativa `H`), com exceção permanente |

**Recomendação única: B.** O projeto ainda não tem produção, as 28 páginas já documentam a reversão e a
ADR/`spec` tratam `down` como invariante do contrato. A opção A troca uma dívida de autoria por uma
exceção permanente e retira a prova de rollback; a C acrescenta um baseline que pode esconder schema
divergente e também não resolve sozinha o parser. B custa mais agora, mas mantém o desenho verificável
e é a única que destrava o ciclo completo sem reescrever a decisão estrutural.

**Decisão necessária — `Q-SERIE-1`:** escolher **A**, **B** ou **C**. Recomendação: **B**. Se a resposta
for B, a próxima fatia autorizada é escrever/revisar os 28 `down` e prová-los em PG17; se for A ou C, a
resposta precisa autorizar também a mudança de contrato e a revisão da `ADR-DB-0002`. Esta nota não
autoriza nenhuma das três.

### 9.1 Achado lateral — separador do nome

Os 28 arquivos usam `NNNN-slug.sql`; o contrato escreve `NNNN_slug.sql`. Isso não bloqueia o runner —
o parser aceita os dois separadores —, mas é uma divergência de forma. **Recomendação `Q-SEP-1 = A`:**
manter a compatibilidade de leitura para a série materializada e usar `_` nos arquivos novos, sem
renomear nada agora. A alternativa B (renomear os 28) preserva o contrato mas produz 28 renomeações e
novos checksums; a C (mudar o contrato para `-`) espalha uma forma que o restante da documentação não
usa. A decisão não muda a ordem da série.

## 10. Próxima frente recomendada

O quadro canônico de foco é `../../wyd-ops/docs/prioridade-do-projeto.md` §0.1 (D-06). A recomendação
única continua **A — D-01/`wyd-db`**: é a frente que fecha **64 células** e **141 atos** e autoriza
**40 issues**, sem depender de terceiro; B, C e D fecham **0 atos/issues** no curto prazo. Dentro dela,
a próxima fatia executável é o **ato 4** (`CAP-37`–`CAP-39`: **3 células `teste`**, **13 → 16 de 39**),
seguida do **ato 5** (**40 issues + 141 atos**). O ato 4 que usa a série real depende de `Q-SERIE-1`; a
decisão de B é o que libera `CAP-37`/o ciclo de sistema sem manter uma exceção de contrato.

**O que A libera de fato:** o DDL aceito deixa de ser proposta para `portal`/`admin`/`server`; as **40
issues** ganham autorização para virar roadmap e as **141 páginas** de `docs/` fecham o aceite; o
`wyd-protocol` tem o ato 2 absorvido e pode seguir com contratos/testes. **O que não libera:** as **6
lacunas abertas + 1 proposta** e os **6 seeds** continuam por terceiro/dado, e nenhuma lacuna aberta do
`wyd-db` é reaberta por esta nota.

**Perguntas abertas para o dono:**

1. **`Q-SERIE-1`** — qual política de `down`? **A** forward-only com marcador, **B** escrever os 28
   `down` reais (**recomendada**), **C** baseline dos 28. A resposta libera a próxima fatia de sistema
   sem alterar a ordem da série.
2. **`D-06`** — qual frente executa primeiro? **A** D-01/`wyd-db` (**recomendada**), **B** executar a
   migração D-05, **C** `wyd-portal`, **D** `wyd-protocol`. A recomendação A é a única que fecha
   **141 atos/64 células/40 issues** hoje; as demais fecham **0** no curto prazo.
3. **`Q-SEP-1`** — como resolver `-` × `_`? **A** compatibilidade de leitura + `_` nos novos
   (**recomendada**), **B** renomear os 28, **C** mudar o contrato para `-`. Não há renomeação nesta
   rodada.
