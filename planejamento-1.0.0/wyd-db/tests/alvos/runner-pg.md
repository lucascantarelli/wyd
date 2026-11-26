> Pergunta: "o que o alvo `runner_pg` prova pela porta `PgAdapter` real, com que fixture e como se lê a falha?"
> Público: QA / testador e revisor de CI
> Dono: equipe do `wyd-db`
> Base: `../specs/capacidades.md` §5; `../../src/migrate/spec.md` §13; `../../proposta-alvo-pgadapter.md`; `.github/workflows/code.yml` job `runner`
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25) — alvo escrito e executado contra PostgreSQL 17: **5/5** casos verdes
(log local `/tmp/wyd-runner-pg.log`; 23,81 s). Fecha `CAP-37` e a metade runner de `CAP-01`–`CAP-07`.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — o alvo está em `crates/wyd-db/Cargo.toml` (`runner_pg`) e a fixture em
  `crates/wyd-db/tests/resources/`.
- **`V`** — o que foi medido em 2026-11-25: 5 casos, 0 falhas, contra PG17 em `127.0.0.1`.
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
| `down_ate_vazio_esquece_o_ledger` | `Down { to: Empty }` depois do apply | 4 versões desfeitas; ledger vazio; tabela e view ausentes |
| `cap37_segunda_execucao_espera_no_advisory_lock` | processo A com o advisory lock tomado; processo B executa `apply` | B devolve `LockBusy { waited: 10 s }`, código de saída 3, sem preparar o ledger; liberado o lock, B aplica as 4 versões |

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

- **A série materializada e os checksums reais:** isso é `reversibilidade_pg` e o alvo de contrato.
- **A contiguidade da série real:** depende das seis versões de seed.
- **`CAP-38` e `CAP-39`:** consulta de personagem e boot têm alvos próprios, ainda sem código.

## 6. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| quando o alvo passa a aplicar a série real? | fechar as seis versões ausentes e trocar a fixture por `migrations/` (`Q-ALVO-2` = B2) | curadoria de seed + `wyd-db` |
