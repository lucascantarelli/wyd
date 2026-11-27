> Pergunta: "o que o alvo `runner_pg` prova pela porta `PgAdapter` real, com que fixture e como se lê a falha?"
> Público: QA / testador e revisor de CI
> Dono: equipe do `wyd-db`
> Base: `../specs/capacidades.md` §5; `../../src/migrate/spec.md` §13; `../../proposta-alvo-pgadapter.md`; `.github/workflows/code.yml` job `runner`
> Revisado em: 2026-11-27

**Estado:** aceito (2026-11-25) — alvo escrito e executado contra PostgreSQL 17: **7/7** casos verdes
(V, 2026-11-26: `crates/wyd-db/tests/sistema/runner_pg.rs`, log `/tmp/wyd-runner-pg-wyddb030.log`,
33,43 s; os 5 primeiros já tinham passado em 2026-11-25). Fecha `CAP-37` e a metade runner de
`CAP-01`–`CAP-07`.

**Reexecução de 2026-11-27:** mesma fixture, `WYD_DB_TEST_URL` com `wyd_migrator` em `127.0.0.1`,
**7 passed; 0 failed; 0 ignored** em 31,24 s — é a medição vigente do alvo (a de 2026-11-25 tinha 5
casos).

**Provas do DoD da `WYDDB-030` acrescentadas em 2026-11-26:** drift de **um byte** no arquivo (não no
ledger), ciclo `down` até vazio seguido de `apply` e concorrência entre **dois processos** de verdade —
o segundo reexecuta o binário de teste com `--exact` em vez de abrir outra conexão no mesmo processo.

**Reorganização (2026-11-26):** o arquivo passou a `crates/wyd-db/tests/sistema/runner_pg.rs`;
`cargo test --no-run` recompilou o alvo, mas a execução não foi repetida localmente (sem PG17 e
`WYD_DB_URL_TEST`); o job `pg` (`runner`) de `ci-db.yml` executou o alvo no PR #220 e passou.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — o alvo está em `crates/wyd-db/Cargo.toml` (`runner_pg`) e a fixture em
  `crates/wyd-db/tests/resources/`.
- **`V`** — medido contra PG17 em `127.0.0.1`: 5 casos em 2026-11-25 e **7 casos, 0 falhas** em
  2026-11-27.
- **`H`** — dúvida aberta, em §6; hoje só a série real, que depende de contiguidade.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL.

Este é o alvo que faltava entre as duas provas existentes: `sistema_pg` prova o **schema** por `psql` e
`reversibilidade_pg` prova o **SQL** do ciclo `up → down → up`; nenhum dos dois exercita `PgAdapter`,
ledger, transação por arquivo, `search_path = ''` ou `pg_advisory_lock`. O `runner_pg` executa o mesmo
comando que o binário vai executar, com as portas reais.

## 1. Capacidades cobertas

| `CAP` | Capacidade | Cenário |
|---|---|---|
| `CAP-37` | a segunda execução espera no advisory lock no máximo **10 s** e falha nomeando a espera (`RNF-DB-002`) | `cap37_segunda_execucao_espera_no_advisory_lock` |
| `CAP-01`–`CAP-07` (metade runner) | aplicar, reaplicar sem efeito, transação por arquivo, drift de checksum, `down` e ledger | `apply_grava_ledger_real_e_e_idempotente`, `drift_de_checksum_reprova_status_e_apply`, `down_ate_vazio_esquece_o_ledger` |

`CAP-38` (p95 da leitura de personagem) e `CAP-39` (boot) **não** estão aqui: são caminho de consulta e
binário, não porta de migration.

## 2. Cenários

| Cenário | Entrada | Resultado esperado |
|---|---|---|
| `apply_grava_ledger_real_e_e_idempotente` | fixture contígua de 4 arquivos; `apply`; depois `apply` de novo | 4 versões aplicadas; ledger com 4 linhas, checksum de 64 hex e slug correto; segunda execução não escreve |
| `arquivo_que_falha_nao_commita_parcial` | fixture em que o arquivo 2 cria tabela e divide por zero | erro `Sql` na versão 2, código de saída 5; ledger com 1 linha; a tabela do arquivo 2 não existe |
| `drift_de_checksum_reprova_status_e_apply` | ledger com checksum adulterado na versão 2 | `Status { check: true }` devolve `Drift(Checksum)` na versão 2, código 4; `apply` recusa sem escrever |
| `down_ate_vazio_esquece_o_ledger` | `Down { to: Empty }` depois do apply, seguido de `apply` | 4 versões desfeitas; ledger vazio; tabela, view e índice ausentes; o `apply` seguinte reaplica as 4, o ledger volta a 4 linhas e o seed roda uma vez |
| `drift_de_um_byte_no_arquivo_reprova_com_codigo_4` | cópia descartável da fixture; `apply`; **um byte** trocado em `0003_fixture_view.sql` | `Drift(Checksum)` na versão 3, código de saída 4; `apply` recusa sem escrever e o ledger segue com 4 |
| `cap37_segunda_execucao_espera_no_advisory_lock` | processo A com o advisory lock tomado; processo B executa `apply` | B devolve `LockBusy { waited: 10 s }`, código de saída 3, sem preparar o ledger; liberado o lock, B aplica as 4 versões |
| `dois_processos_o_segundo_espera_no_lock_e_termina_sem_aplicar` | processo 1 com o lock; processo 2 reexecuta o binário de teste (`--exact`, espera de 2 s); depois o processo 1 aplica e o 2 volta | o processo 2 sai com código 3 e não prepara o ledger; o 1 aplica as 4; o 2 volta, sai com 0 e **não** reaplica |

**Por que a fixture é contígua e sintética (`Q-ALVO-2` = B1):** `MigrationSet::build` recusa série com
furo, e a série real ainda tem seis versões ausentes (`0016`, `0017`, `0018`, `0020`, `0023`, `0027`). O
lock e o ledger não dependem de qual série roda; inventar arquivo para tapar o furo seria falsificar a
curadoria de seed.

**Por que o papel é real (`Q-ALVO-3`):** `preflight` exige `current_user = wyd_migrator`. O teste conecta
com esse papel, criado com `LOGIN` só no container descartável; `allow_superuser(true)` enfraqueceria
justamente o preflight que o alvo deve provar.

## 3. Pré-requisito de ambiente

| Item | Regra |
|---|---|
| PostgreSQL | **17**, em container descartável |
| acesso | `WYD_DB_TEST_URL` com TCP em **`127.0.0.1`** (`Q-ALVO-1` = A1); nunca `0.0.0.0` |
| papel | `wyd_migrator` com `LOGIN` e `CREATE` no banco e no schema `public` |
| ausência | sem a variável o alvo **falha**, não se ignora |
| paralelismo | uma base por execução; o alvo serializa os casos por mutex e o CI usa `--test-threads=1` |
| limpeza | cada caso derruba `fixture` e `public.schema_migrations` antes de começar |

O job `runner` de `.github/workflows/code.yml` publica a porta só em loopback, cria o papel de teste e
roda o alvo como bloqueante.

## 4. Como interpretar a falha

- **`WrongRole`:** a DSN não é do `wyd_migrator` — o defeito é de ambiente, não do runner.
- **`LockBusy` que não vem:** o lock não está sendo tomado por sessão distinta, ou o `lock_timeout` não
  chegou ao servidor.
- **`Drift` que não vem:** o checksum gravado não é recalculado do arquivo, ou o `record` não persiste.
- **Tabela que sobrevive ao arquivo que falhou:** `begin`/`commit` não estão por arquivo.
- **`relation does not exist` em SQL qualificado:** alguma instrução ficou sem schema e o
  `search_path = ''` a recusou — o comportamento é o esperado em produção.

## 5. O que este alvo não prova

- **A série materializada e os checksums reais:** continua fora deste alvo, porque
  `MigrationSet::build` recusa série com furo e a série tem seis versões ausentes (V, 2026-11-26:
  `crates/wyd-db/migrations/` numera 28 arquivos e falta `0016`, `0017`, `0018`, `0020`, `0023` e
  `0027`). Fechar o furo é curadoria de seed, não teste.
- **A contiguidade da série real:** depende das seis versões de seed.
- **`CAP-38` e `CAP-39`:** consulta de personagem e boot têm alvos próprios, ainda sem código.

## 6. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| quando o alvo passa a aplicar a série real? | fechar as seis versões ausentes e trocar a fixture por `migrations/` (`Q-ALVO-2` = B2) | curadoria de seed + `wyd-db` |

**Medição de 2026-11-26:** enquanto o furo existir, rodar o runner sobre `migrations/` devolve
`SetInvalid(NotContiguous { expected: 16, found: 19 })` — a série para em `0015` e a próxima existente é
`0019`. Isso é comportamento provado do `MigrationSet`, não falha do alvo (`V`:
`crates/wyd-db/src/migrate/migration/set.rs`, teste `rejects_gap`).
