> Pergunta: "como testo este crate, quais alvos existem e o que cada um prova?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.17; `plano-de-testes.md`; `tests/` planejado
> Revisado em: 2026-11-27

**Estado:** proposto (2026-11-22)

O guia de teste lista os alvos do crate: quatro dos sete da tabela já estão declarados no `Cargo.toml`
(`sistema_pg`, `reversibilidade_pg`, `runner_pg` e `contrato_migrations`), e os dois contratos de API também
(`contrato_api_migrate` e `contrato_api_provision`); os três restantes seguem planejados. O comando
único da suíte é `cargo test -p wyd-db` (§3), e a coluna “Estado” de §1 diz o que já rodou, com data;
o guia proíbe chamar de verde o que não rodou.

## 1. Os alvos

| Alvo | Prova | Banco | Estado |
|---|---|---|---|
| `contrato_catalogo` | documento de catálogo e série descrevem as mesmas tabelas | não | planejado |
| `contrato_migrations` | forma da série: contiguidade, `down`, FK, seed, constraint e tipo | não | 9 testes em 2026-11-27: `CAP-02`–`CAP-07` verdes na série viva; `CAP-01` vermelho (seis furos) |
| `sistema_pg` | aplicar schema e cenários `CAP-09`–`CAP-19`, `CAP-24`–`CAP-36` e `CAP-38`–`CAP-39` por `psql` | sim | 13/13 dos cenários de `CAP-24`–`CAP-36` em PG17 (2026-11-25) |
| `runner_pg` | porta `PgAdapter`: ledger, transação por arquivo, drift e advisory lock em duas sessões | sim | executado 7/7 em PG17 (2026-11-27); fecha `CAP-37` |
| `reversibilidade_pg` | ciclo SQL `up → down → up` por `psql`; não passa pelo runner | sim | 3/3 em 2026-11-25 (1 ciclo em PG17 + 2 conferências textuais) |
| `sistema_sessoes` | privilégio por papel e leitura cruzada por view | sim | planejado |
| `sistema_stack_caps` | matriz de capacidades `CAP-nn` | sim | planejado; a cobertura é medida por `tests/specs/capacidades.md` |

O detalhe de cada alvo está em `tests/alvos/` e a matriz em `tests/specs/capacidades.md` — fora do site,
porque `tests/` não é publicado. A página
[`piramide-e-alvos.md`](piramide-e-alvos.md) diz como escolher o nível; a página
[`gates-e-anti-skip.md`](gates-e-anti-skip.md) diz como impedir teste que não roda.

## 2. Mapa dos guias

| Guia | Pergunta | Estado |
|---|---|---|
| [`piramide-e-alvos.md`](piramide-e-alvos.md) | que tipo de teste escrevo? | recorte escrito |
| [`fixtures-e-factories.md`](fixtures-e-factories.md) | como crio dado? | escrito (2026-11-24) |
| [`dados-de-teste.md`](dados-de-teste.md) | que dado existe? | escrito (2026-11-24) |
| [`testes-de-contrato.md`](testes-de-contrato.md) | como provo a forma sem banco? | escrito |
| [`testes-de-sistema.md`](testes-de-sistema.md) | como provo contra PG real? | escrito |
| [`gates-e-anti-skip.md`](gates-e-anti-skip.md) | como impeço `SKIP`? | recorte escrito |

## 3. Comando único

O comando único da suíte é `cargo test -p wyd-db`: os contratos de API rodam sem banco e os alvos de
sistema entram por `--test`, como em `tests/index.md` §3. `sistema_pg` (13/13) e `reversibilidade_pg`
(3/3) rodaram em 2026-11-25, e o `runner_pg` (7/7) em 2026-11-27. A DSN de teste é lida do ambiente
pelo harness e pelo alvo; os dois nomes estão em `src/provision/spec.md` §3.

## 4. Regras que valem para todos os alvos

- Nenhum teste sem capacidade associada; nenhuma capacidade sem teste.
- `SKIP`, `#[ignore]` novo ou filtro silencioso reprovam o aceite.
- Teste de contrato não abre conexão; teste de sistema não usa falso para provar o banco.
- Dado de teste é fictício, isolado por chave e limpo pelo próprio teste.
- Falha nomeia arquivo, versão ou variável — não uma mensagem genérica.

## 5. A fronteira dos dois clientes

O crate toca o PostgreSQL por dois clientes, e isso é decisão, não duplicação (2026-11-25):

| Cliente | Onde vive | O que prova | O que não prova |
|---|---|---|---|
| `psql` por `docker exec` | harness dos alvos com banco (`crates/wyd-db/tests/comum/mod.rs`) | o contrato do **schema**: constraint, índice, privilégio, view e o ciclo `up → down → up` | a porta do runner: transação por arquivo, `search_path = ''`, `record`/`forget` e a espera no advisory lock |
| `sqlx` | `migrate/adapter/pg.rs`, e só lá (`../guia-desenvolvedor/mapa-do-crate.md` §2) | a **porta**: `Ledger`, `MigrationExecutor`, `AdvisoryLock` e a transação do runner | nada de schema que não passe pelo runner |

Duas consequências que valem para quem escreve cenário:

- `psql -f` roda em autocommit por instrução e o runner roda um arquivo por transação; cenário que
  dependa de rollback de arquivo inteiro só vale pelo runner.
- o container da CI publica porta TCP **só em loopback** para `runner_pg` (`127.0.0.1`), porque a porta precisa de uma conexão `sqlx` real; o alvo de schema continua por `docker exec`.
- a série real ainda não é contígua: `runner_pg` aplica uma fixture em `tests/resources/` até as seis versões ausentes fecharem.

## 6. O que esta página não cobre

- **Cenários detalhados:** são de `tests/alvos/`, com seis páginas de alvo desde 2026-11-27.
- **Cobertura percentual e tempo:** não medidos; a regra é por capacidade.
- **CI:** `.github/workflows/code.yml` declara os jobs de schema e reversibilidade em PG17; a execução local deles está registrada em `migrations/serie-e-regras.md` §9.2. Isso não substitui uma execução remota do GitHub Actions.
