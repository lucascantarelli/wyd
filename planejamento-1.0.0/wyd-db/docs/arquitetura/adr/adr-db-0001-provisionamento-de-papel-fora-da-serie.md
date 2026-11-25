> Pergunta: "os cinco papéis de cluster nascem dentro da série versionada ou numa fase própria de provisionamento?"
> Público: Arquiteto de schema e desenvolvedor do `wyd-db` que for implementar o provisionamento
> Dono: equipe do `wyd-db`
> Base: `padrao-banco.md` §8, `src/provision/spec.md`
> Revisado em: 2026-11-17

**Estado:** aceito (2026-11-17)

# ADR-DB-0001 — Provisionamento de papel é fase própria, fora da série versionada

Quatro níveis de afirmação convivem aqui:

- **`P`** — o que a 1.0.0 projeta e ainda não existe no banco: os cinco papéis, os `GRANT`, o
  `db/roles.sql` gerado e a fase de provisionamento.
- **`H`** — leitura de intenção, dita com o que a fecha: a ordem do DDL que exigiria os papéis foi
  deduzida do arquivo, não executada.
- **`V`** — o que foi medido no repositório herdado, com caminho e data: `crates/wyd-db/src/migrate.rs`,
  `grep`/`ls` de 2026-11-17.
- **sem marca** — comportamento do PostgreSQL: `CREATE ROLE` escreve em `pg_authid`, papel vale para o
  cluster e não viaja em `pg_dump`.

## Contexto

A série 1.0.0 nasce com quatro schemas de domínio e cinco papéis — `wyd_owner`, `wyd_migrator`,
`wyd_game`, `wyd_portal` e `wyd_read` —, definidos em `padrao-banco.md` §8 (`P`: nada disto existe
no banco). O `0001_bootstrap` cria schemas, `GRANT` e default privileges e **pressupõe** os papéis
existentes: sem eles falha no primeiro `AUTHORIZATION` com `role "wyd_owner" does not exist`
(`H`, pela ordem do DDL; não executado).

O runner atual é de **schema por banco** (`V`, `crates/wyd-db/src/migrate.rs`, 2026-11-17), e são
estas as propriedades que importam aqui:

- série lida de uma variável de ambiente — nome **herdado** `MIGRATIONS_DIR`; a 1.0.0 usa
  `WYD_MIGRATIONS_DIR` (`M24`) —, ordenada por nome;
- ledger `schema_migrations(version, checksum, applied_at)` **no banco**;
- advisory lock de sessão, uma transação por arquivo;
- baseline por `public.accounts`;
- gate final exige `public.item_catalog`;
- `--reset` faz `DROP SCHEMA IF EXISTS public CASCADE`;
- chamado no boot do canal e do gateway, e pela operação (`--migrate`/`--reset`).

As linhas de `migrate.rs` e dos dois binários que provam cada item estão em `Rastreabilidade`.

Papel é outra coisa: `CREATE ROLE` escreve em `pg_authid`, vale para o **cluster** inteiro e não
viaja em `pg_dump`. Hoje a série herdada não tem uma única linha de papel ou de `GRANT` (`V`,
`grep -rn "CREATE ROLE\|ALTER ROLE\|GRANT" crates/wyd-db/migrations/` sem resultado, 2026-11-17) e
o `db/roles.sql` que `padrao-banco.md` §8 menciona **não existe** (`V`, `ls db/` vazio, 2026-11-17).
A questão só aparece agora, porque é a 1.0.0 que introduz a fronteira por papel.

O problema concreto: **o ledger é por banco, o papel é por cluster**, e todo processo que migra —
inclusive o servidor de jogo — passaria a precisar de `CREATEROLE` para que o `0001` rodasse.

## Decisão
**Sem arquivo no repositório:** `crates/wyd-db/src/provision/domain/spec.rs`, `db/roles.sql`, `domain/spec.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Adotamos **provisionamento de papel como fase anterior e separada da série versionada**, executada
uma vez por cluster com um DSN administrativo próprio, porque o ledger é por banco e exigir
`CREATEROLE` em todo processo que migra anula a separação de papéis de `padrao-banco.md` §8 (`P-18`).

Regras que a decisão implica (`P`, nada implementado):

1. **Papel não entra na série.** Nenhum arquivo de `crates/wyd-db/migrations/` contém
   `CREATE ROLE`, `ALTER ROLE`, `DROP ROLE` ou `GRANT <papel> TO`. A série continua DDL de banco.
2. **Uma fonte para os papéis.** A lista mora em código, em `crates/wyd-db/src/provision/domain/spec.rs`
   (`const ROLES: &[RoleSpec]`, cinco entradas na ordem de emissão), e `db/roles.sql` passa a ser
   **artefato gerado** dela — como `padrao-banco.md` é texto e não SQL solto. Edição manual do `.sql`
   fica proibida. O texto gerado, o golden e os invariantes têm dono único: `spec.md` §2, §4 e §8.
3. **DDL idempotente.** `CREATE ROLE` não tem `IF NOT EXISTS`, e `IN ROLE` só vale na criação: num
   cluster onde o papel já existe, a associação nunca seria aplicada. Portanto, `DO $$ … IF NOT
   EXISTS (SELECT 1 FROM pg_roles …) THEN CREATE ROLE … $$` por papel e **`GRANT <dono> TO <membro>`
   separado** para a associação.
4. **DSN administrativo separado.** `WYD_PROVISION_DSN` (com `CREATEROLE` e posse do banco) é lido só pelo passo de
   provisionamento. A DSN de migrations é de `wyd_migrator`; as de runtime são `wyd_game` e
   `wyd_portal`. Nenhuma delas cria papel.
5. **CLI do `wyd-db`, sem flag no `wyd-admin`.** O provisionamento é subcomando de `wyd-db`:
   `provision render` gera `db/roles.sql` de `domain/spec.rs` (`--check`: sai `3` na divergência),
   `provision plan` compara o estado desejado com `pg_roles`/`pg_auth_members`, `provision apply`
   envia o script pela porta `Script` — sessão, lock e transação são contrato de
   `src/provision/port/script.md` §3, e não se repetem aqui —, e `provision check` só lê, saindo
   `!= 0` em drift. `migrate` fica na mesma CLI. A execução mora no adapter do provisionamento, cujo
   construtor e assinatura moram só em `src/provision/adapter/pg.md` §2, com nome distinto do
   `PgAdapter` do runner; decisão de 2026-11-18 registrada em `lacunas.md` §1. O `wyd-admin` continua apenas
   chamando `migrate::run` no boot. As assinaturas e as fronteiras entre os subcomandos são de
   `spec.md` §5; os códigos de saída, de
   `spec.md` §6; o advisory lock do **runner de migrations**, de `src/migrate/spec.md` §5.
6. **Preflight de papel no runner.** `migrate::run` valida antes do `load(dir)` que
   `current_user = 'wyd_migrator'` — comparação estrita, porque `pg_has_role` responde `t` para
   superusuário — e devolve erro próprio com a instrução de correção. Opt-out explícito
   `WYD_MIGRATE_ALLOW_SUPERUSER=1`, só em desenvolvimento e logado.
7. **Boot não provisiona.** Gateway e canais continuam chamando apenas `migrate::run`; só o passo de
   operação tem privilégio de cluster.
8. **O estado de `public` é do provisionamento, não do `0001`** (revisto em 2026-11-18; a versão
   anterior deste item mantinha as duas instruções no `0001`). `provision render` emite, em
   `db/roles.sql`, a revogação do `CREATE` de `PUBLIC` e a concessão de `USAGE` e `CREATE` a
   `wyd_migrator`; o texto exato é de `src/provision/spec.md` §4. Três fatos decidem: `public` existe
   com o banco, enquanto os quatro schemas da série são criados pelo `0001`; o runner cria o ledger em
   `public` antes do primeiro arquivo, então o privilégio tem de ser anterior a ele; e, no PG15+,
   `public` pertence a `pg_database_owner`, de modo que rodando como `wyd_migrator` a revogação de
   `PUBLIC` seria aviso sem efeito e a concessão a `wyd_migrator` falharia com `42501` por falta de
   grant option — dentro da série as duas linhas não produziam o estado que anunciavam. Para isso a DSN
   de provisionamento precisa ser a do dono do banco (item 4).

## Consequências

- **Positivas:** privilégio de cluster existe só no passo de provisionamento; `CREATE ROLE` deixa de
  disputar semântica com ledger, baseline e `--reset`; reexecutar é no-op inclusive em cluster com
  papel já criado; boot com papel ausente falha com mensagem acionável em vez de `42501` vindo do
  `0001`; drift de papel é diagnosticável sem acesso manual ao banco de produção; e o `down` do
  `0001` não toca em `public`, então o reset (`down --to 0`) não perde o `USAGE` que o `forget` do
  ledger precisa no último arquivo.
- **Negativas:** o ambiente ganha uma segunda DSN, usada só pela operação; provisionar vira passo
  obrigatório antes da primeira subida, e playbook que o esqueça falha no boot; `db/roles.sql` deixa
  de ser editável à mão; `provision apply` precisa de `CREATEROLE` e da posse do banco, então não roda
  em container de app.
- **Invariantes afetados:** (a) a série não contém DDL de cluster; (b) processo de jogo ou portal
  nunca conecta com DSN administrativo; (c) `db/roles.sql` é gerado, não escrito; (d) todo caminho de
  migration passa pelo preflight de papel; (e) `--reset` — hoje `down` até vazio (`ADR-DB-0002`,
  `src/migrate/spec.md` §8.1) — deixa os quatro schemas ausentes e o ledger vazio, **nunca** toca papel
  nem o estado de `public`.
- **O que continua pendente:** o ajuste de baseline (`public.accounts` → `content.content_version`),
  do gate final (`public.item_catalog` → `content.item_catalog`) e do `--reset` — que a série nova
  resolve **sem** `DROP` de schema, pelo `down` até vazio (`ADR-DB-0002`, `src/migrate/spec.md` §8.1) —
  é consequência de qualquer série nova e **não** faz parte desta decisão.

## Alternativas consideradas
**Sem arquivo no repositório:** `db/roles.sql` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Alternativa | Por que foi descartada |
|---|---|
| **`CREATE ROLE` dentro do `0001`** (com `DO … IF NOT EXISTS`) | O ledger é por **banco** e o papel é por **cluster**: no segundo banco do mesmo cluster o `0001` encontraria o papel já criado, e o `DO` esconderia a associação em vez de aplicá-la. Pior: `CREATE ROLE` exige `CREATEROLE`/superusuário em **toda** conexão que migra, incluindo o boot do servidor de jogo e dos canais (ADR-0024 do repositório), o que anula `P-18` e mantém privilégio de cluster no processo mais exposto. Some-se que `CREATE ROLE` não é idempotente, então `--reset` (que derruba schema e preserva papel) e baseline perdem semântica clara: "algo que roda uma vez por cluster" dentro de um ledger por banco é o acoplamento que a série 1.0.0 existe para eliminar. |
| `db/roles.sql` escrito à mão e aplicado via `psql` (o caminho da série herdada; `padrao-banco.md` §8 hoje exige `provision render` e chama a edição manual de divergência) | Funciona uma vez e nunca mais é verificado: sem detecção de drift, sem prova em CI e cinco nomes digitados à mão por ambiente. Não responde como o boot sabe que pode migrar. |
| Só `docker-entrypoint-initdb.d` | O entrypoint oficial só roda com `PGDATA` vazio — já é a razão pela qual a ADR-0024 do repositório rejeitou esse caminho para schema, e vale igual para papel. Volume existente com papel novo não recebe nada. Fica como conveniência do compose de dev, não como provisionamento. |
| Binário ou crate próprio (`wyd-provision`) | `crates/wyd-db` já é dono do assunto e publica a própria CLI (`wyd-db`), que já tem `migrate` e é quem conhece `ROLES` e o preflight do runner; um crate ou binário a mais paga `Cargo.toml`, dependência e árvore nova por um subcomando de `provision`, sem separar nada que hoje precise ser separado. |
| Provisionar em todo boot do gateway como superusuário | Superfície permanente: o gateway precisaria de `CREATEROLE` para sempre por um passo que muda uma vez por ambiente. |
| Parsear `db/roles.sql` para descobrir os papéis | Texto livre não é contrato; comentário ou quebra de linha mudaria o resultado. `ROLES` em Rust é verificável pelo compilador. |
| Colocar os `GRANT` de schema da série no provisionamento também | `GRANT` nos quatro schemas que a série **cria** é objeto do banco e precisa ser versionado com ela (`P-18`); provisionar privilégio junto com papel perderia o histórico do schema. `public` é a exceção (item 8), porque já existe antes do `0001`. |

## Rastreabilidade

- Evidência histórica (`V`, 2026-11-17): `crates/wyd-db/src/migrate.rs` — `:114` (`dir_from_env`),
  `:163` (`load`), `:29`/`:248` (ledger), `:230`/`:294` (lock e transação), `:260` (baseline),
  `:316` (gate final), `:239` (`--reset`) —, `crates/wyd-server/src/boot/mod.rs:119`,
  `crates/wyd-admin/src/main.rs:1086` e `:1429`, `grep` sem papel/`GRANT` na série herdada e `ls db/`
  vazio. **Essas referências expiram:** o runner herdado dá lugar ao módulo `migrate/` na implementação
  da 1.0.0, e a série herdada só deixa de rodar na troca (`plano-migrations.md` §6) — até lá é ela que
  o produto executa. Depois da troca, leia como registro do que era, não como caminho vivo.
- Planejamento que esta ADR consome (marca **planejamento**, fora de `docs/`):
  `planejamento-1.0.0/wyd-db/padrao-banco.md` §8,
  `planejamento-1.0.0/wyd-db/src/provision/spec.md`,
  `planejamento-1.0.0/wyd-db/migrations/0001-bootstrap.md`.
- Controle de andamento que esta ADR afeta: `planejamento-1.0.0/wyd-db/migrations-checklist.md` (na raiz
  da pasta de planejamento) — a fase de provisionamento não entra nas 29 linhas nem na soma de tabelas.
- Páginas desta árvore a atualizar na implementação: `docs/guia-administrador/index.md` (passo de
  provisionamento no ambiente), `docs/guia-desenvolvedor/index.md` (como aplicar antes de migrar).
- Evidência que falta para o aceite (`P`: nada disto rodou): (a) `provision apply` rodado duas
  vezes sem alterar `pg_roles` nem `pg_auth_members`; (b) `provision check` saindo `!= 0` com um papel
  removido; (c) boot falhando com erro próprio quando a DSN do papel aponta para superusuário
  (`WYD_DB_URL_*`, nunca um
  `DATABASE_URL` único — decisão `M24`, 2026-11-18).
- Issues: nenhuma aberta para esta frente em 2026-11-17 (`V`, `roadmap-rust/issues/` não cita
  `roles.sql` nem provisionamento de papel); a numeração `WYDDB-nnn` entra quando a série for aberta.

## Revisão
**Sem arquivo no repositório:** `db/roles.sql` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

**Revisão de estado — 2026-11-21 (`M27`/`0032`).** O total vigente passou de cinco para seis papéis
com a inclusão de `wyd_admin` (`src/provision/spec.md` §2; `lacunas.md` §2.3). A série passou de 31
para 32 arquivos com `0032` (`plano-migrations.md` §3; `migrations/0032-admin-grants.md`). Esta nota
atualiza os números de estado, sem reescrever a decisão original: as menções a cinco papéis acima
registram o desenho considerado no aceite de 2026-11-17.

**Revisão de estado — 2026-11-22 (`M30`/`M31`).** O `0032` foi emendado com os `GRANT` da custódia
(`game.character_item`, `game.character`) e a série fechou em **34 arquivos e 109 tabelas** com os lotes
`0033` e `0034` (`plano-migrations.md` §3; `migrations/0032-admin-grants.md`, `0033-ops-idempotency.md`
e `0034-portal-market.md`). O papel e a forma da decisão não mudaram: só o escopo de privilégio de um
arquivo e o total da série.

- Revisar quando: houver mais de um banco por cluster com séries diferentes; entrar réplica de leitura
  ou RLS (`P-18`, multi-servidor); ou o provisionamento passar a gerenciar senha e rotação.
- Sinais de que envelheceu: papel novo pedido em produção sem rodar `provision apply`; `42501` recorrente
  no boot; `db/roles.sql` editado sem regeneração; `--check` nunca usado no CI.
