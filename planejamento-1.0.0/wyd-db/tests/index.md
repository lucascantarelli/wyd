> Pergunta: "como a suíte está montada, que alvo prova o quê e como eu rodo?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../wyd-db-1.0.0.md` §6.17; `../docs/planos/plano-de-testes.md` §2–§4; `../docs/guia-teste/piramide-e-alvos.md`
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-24) — página escrita; **4 dos 7** alvos implementados e **20 dos 39** capacidades
com teste executado (2026-11-27). `reversibilidade_pg`, `runner_pg` e `contrato_migrations` entraram
depois do aceite original.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre a suíte: `sistema_pg`, `reversibilidade_pg`, `runner_pg`,
  `contrato_migrations`, `contrato_api_migrate` e `contrato_api_provision` existem em
  `crates/wyd-db/Cargo.toml`; os três alvos de §2 sem arquivo são os planejados. O harness é `tests/comum/mod.rs` — não existe `common/` —
  e o dado de apoio está em `tests/resources/fixture/` e `tests/resources/fixture_falha/`.
- **`H`** — dúvida aberta, dita e localizada em §5, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o § ao lado.
- **sem marca** — regra de projeto (`../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../padrao-banco.md` §13).

Esta pasta é o **plano da suíte** do `crates/wyd-db`: sete alvos, o harness de `tests/comum/mod.rs`,
o dado de apoio de `tests/resources/` e as regras que valem para toda a suíte, em `specs/`.
A matriz de capacidades é [specs/capacidades.md](specs/capacidades.md) e é ela, não esta página, que diz
o que cada alvo prova.

## 1. Árvore de `crates/wyd-db/tests/` — o que existe e o que é planejado

```text
crates/wyd-db/tests/
├── comum/
│   └── mod.rs                # harness: DSN de teste e psql           (existe)
├── contrato/
│   ├── contrato_api_migrate.rs    # fachada pública de `migrate`     (existe)
│   ├── contrato_api_provision.rs  # fachada pública de `provision`   (existe)
│   ├── contrato_catalogo.rs       # catálogo × série                 (planejado)
│   └── contrato_migrations.rs     # forma da série                   (planejado)
├── resources/
│   ├── fixture/              # série contígua de 4 arquivos          (existe)
│   └── fixture_falha/        # falha no meio do arquivo              (existe)
└── sistema/
    ├── sistema_pg.rs         # comportamento no PG17 por psql        (existe)
    ├── reversibilidade_pg.rs # ciclo up → down → up por psql         (existe)
    ├── runner_pg.rs          # porta PgAdapter, ledger e lock        (existe)
    ├── sistema_sessoes.rs    # papel, fronteira e view               (planejado)
    └── sistema_stack_caps.rs # meta-alvo: a matriz e o anti-skip     (planejado)
```

`comum/` é o único lugar com helper compartilhado — hoje só `mod.rs`; `db.rs`, `factory.rs` e
`serie.rs` continuam planejados e estão inventariados em
[fixtures-e-factories.md](fixtures-e-factories.md). Arquivo dentro de `tests/` que não é alvo nem
recurso vive ali.

**Reorganização (2026-11-26):** os alvos de contrato ficam em `contrato/` e os de sistema em
`sistema/`; o harness atual continua em `comum/mod.rs` e o nome do alvo no `Cargo.toml` não mudou
(`--test sistema_pg`, `--test runner_pg`, `--test reversibilidade_pg`).

A movimentação não altera o comportamento: `cargo test --no-run` compilou os três alvos em 2026-11-26,
mas a execução contra PostgreSQL 17 **não foi repetida** localmente nessa data (sem
`WYD_DB_TEST_URL`/container); o job `pg` de `ci-db.yml` (matriz `schema`, `reversibilidade` e
`runner`) executou os três no PR #220 e passou.

## 2. Alvo → arquivo → tipo → precisa de banco
**Sem arquivo no repositório:** `contrato_catalogo.rs`, `sistema_sessoes.rs`, `sistema_stack_caps.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje. `sistema/sistema_pg.rs`, `sistema/reversibilidade_pg.rs`, `sistema/runner_pg.rs`, `contrato/contrato_migrations.rs` (desde 2026-11-27) e os contratos de API
`contrato/contrato_api_migrate.rs` e `contrato/contrato_api_provision.rs` estão versionados.

| Alvo (`Cargo.toml`) | Arquivo | Tipo | Precisa de banco | Capacidades |
|---|---|---|---|---|
| `contrato_migrations` | `contrato/contrato_migrations.rs` | contrato | **não** | `CAP-01`–`CAP-07` |
| `contrato_catalogo` | `contrato/contrato_catalogo.rs` | contrato | **não** | `CAP-08` |
| `sistema_pg` | `sistema/sistema_pg.rs` | sistema | **sim** | `CAP-09`–`CAP-19` e `CAP-24`–`CAP-36`, `CAP-38`–`CAP-39` |
| `reversibilidade_pg` | `sistema/reversibilidade_pg.rs` | sistema | **sim** | `CAP-02` (ciclo SQL da série) |
| `runner_pg` | `sistema/runner_pg.rs` | sistema | **sim** | `CAP-01`–`CAP-07` (metade runner) e `CAP-37` |
| `sistema_sessoes` | `sistema/sistema_sessoes.rs` | sistema | **sim** | `CAP-20`–`CAP-21` |
| `sistema_stack_caps` | `sistema/sistema_stack_caps.rs` | meta | **sim** | `CAP-22`–`CAP-23` |

Um alvo por arquivo, como manda `../wyd-db-1.0.0.md` §5.13. Tipo e fronteira de cada camada estão em
`../docs/guia-teste/piramide-e-alvos.md`; o que cada um **não** prova está na seção 4 de cada página de
`alvos/`.

### 2.1 Porta de entrada — as 13 páginas
**Sem arquivo no repositório:** `accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

O critério de pronto exige que toda página esteja linkada no índice da pasta (`../wyd-db-1.0.0.md` §10,
item 10). As doze abaixo são as demais páginas de `tests/`; esta porta fecha a décima terceira — são
**13** páginas, medido em 2026-11-25 (a página do `runner_pg` entrou depois do aceite). Cada linha
diz **a pergunta que a página responde**, **quando abri-la** e **para onde ir depois** — não é uma lista
de arquivos, é o caminho de leitura.

| Página | Pergunta que responde | Quando abrir | Depois vá para |
|---|---|---|---|
| [specs/capacidades.md](specs/capacidades.md) | que capacidade existe, qual alvo a prova e onde ela é provada | **primeiro**, quando quiser saber o que a suíte promete | a página do alvo que a linha nomeia |
| [alvos/contrato-migrations.md](alvos/contrato-migrations.md) | o que a **forma** da série prova (`CAP-01`–`CAP-07`) | ao mexer em migration ou revisar contrato | [specs/contrato.md](specs/contrato.md), [resources/README.md](resources/README.md) |
| [alvos/contrato-catalogo.md](alvos/contrato-catalogo.md) | a paridade catálogo × série (`CAP-08`) | ao tocar o dicionário; hoje está bloqueada | [specs/contrato.md](specs/contrato.md) |
| [alvos/sistema-pg.md](alvos/sistema-pg.md) | o comportamento do schema contra PG17 (`CAP-09`–`CAP-19`, `CAP-24`–`CAP-36`, `CAP-38`–`CAP-39`) | ao escrever teste que abre conexão por `psql` | [specs/sistema.md](specs/sistema.md), [fixtures-e-factories.md](fixtures-e-factories.md) |
| [alvos/runner-pg.md](alvos/runner-pg.md) | a porta `PgAdapter`: ledger, transação, drift e lock (`CAP-01`–`CAP-07` e `CAP-37`) | ao mexer no runner ou no lock | [../../src/migrate/spec.md](../../src/migrate/spec.md) |
| [alvos/sistema-sessoes.md](alvos/sistema-sessoes.md) | papel, fronteira e leitura por view (`CAP-20`–`CAP-21`) | ao mexer em `GRANT`, view ou auditoria | [specs/sistema.md](specs/sistema.md), [dados-de-teste.md](dados-de-teste.md) |
| [alvos/sistema-stack-caps.md](alvos/sistema-stack-caps.md) | a suíte sobre a suíte: DSN, anti-skip e matriz ligada (`CAP-22`–`CAP-23`) | ao mexer em ambiente de teste ou na matriz | [specs/capacidades.md](specs/capacidades.md) |
| [fixtures-e-factories.md](fixtures-e-factories.md) | o que existe em `common/` e como o dado é limpo | **antes** do primeiro teste de sistema | [dados-de-teste.md](dados-de-teste.md), [resources/README.md](resources/README.md) |
| [dados-de-teste.md](dados-de-teste.md) | que contas existem, o que cada uma prova e o que nunca entra | ao precisar de conta, personagem ou sessão | [resources/README.md](resources/README.md) |
| [resources/README.md](resources/README.md) | formato de `accounts.json` e das fixtures de série | ao criar fixture ou ficha de conta | [alvos/contrato-migrations.md](alvos/contrato-migrations.md) |
| [specs/contrato.md](specs/contrato.md) | as 8 regras da camada **sem** banco | ao escrever teste de contrato | [alvos/contrato-migrations.md](alvos/contrato-migrations.md) |
| [specs/sistema.md](specs/sistema.md) | as 18 regras da camada **com** banco | ao escrever teste de sistema | [alvos/sistema-pg.md](alvos/sistema-pg.md) |

**Ordem de leitura, para quem chega agora:** [specs/capacidades.md](specs/capacidades.md) → a página do
alvo da capacidade que interessa → a regra da camada ([specs/contrato.md](specs/contrato.md) ou
[specs/sistema.md](specs/sistema.md)) → o recurso de que o cenário depende
([fixtures-e-factories.md](fixtures-e-factories.md), [dados-de-teste.md](dados-de-teste.md),
[resources/README.md](resources/README.md)).

## 3. Como rodar

| O que | Comando | Observação |
|---|---|---|
| tudo do crate | `cargo test -p wyd-db` | os contratos rodam sem banco; os três alvos de sistema exigem `WYD_DB_TEST_URL` |
| um alvo | `cargo test -p wyd-db --test <alvo>` | `<alvo>` é o nome da coluna 1 |
| um cenário | `cargo test -p wyd-db --test <alvo> <cenário>` | o nome do cenário é o da matriz |
| só os contratos | `cargo test -p wyd-db --test contrato_migrations --test contrato_api_migrate --test contrato_api_provision` | não abre conexão |

**Ambiente:** os alvos que exigem banco leem a DSN de teste do ambiente — `WYD_DB_TEST_URL`, ou
`WYD_DB_TEST_CONTAINER` para o `psql` dentro do container (`../src/provision/spec.md` §3) — e falham
**nomeando a variável** quando ela falta, nunca pulam (`../docs/guia-teste/testes-de-sistema.md`); `CAP-22`. Nenhum
binário lê `DATABASE_URL`.

**Pré-requisito dos alvos de sistema:** banco vazio → provisionamento → série. Banco "já quente" não é
ambiente de teste: o banco nasce e morre com a execução.

## 4. O que a suíte não faz

- **Não mede cobertura de linha:** a regra é por capacidade (`CAP-nn`), não por percentual.
- **Não testa interface, E2E, carga ou regra de jogo:** os dois primeiros são dos sistemas consumidores;
  os dois últimos não têm volume nem limiar aprovado (`../docs/planos/plano-de-testes.md` §6).
- **Não substitui o contrato de catálogo do repositório:** o leitor em uso ainda não enxerga
  `CREATE TABLE esquema.tabela` (`../docs/guia-teste/testes-de-contrato.md` §1).
- **Não declara verde:** só o que rodou conta; a contagem de capacidades cobertas é de
  [specs/capacidades.md](specs/capacidades.md) §7 — hoje **20 de 39**, 13 em `sistema_pg`, `CAP-37` em
  `runner_pg` e `CAP-02`–`CAP-07` em `contrato_migrations` (V, 2026-11-27: 9 testes em 0,09 s, sem
  banco; `CAP-01` segue fora porque a série viva tem seis furos).

## 5. Dúvidas do dono e o que elas decidiram

As duas perguntas do dono que decidiam o que esta pasta autoriza estão respondidas:

| Dúvida | O que a resolve | Dono |
|---|---|---|
| o aceite das 12 páginas libera o código dos cinco alvos, ou o código espera a issue no GitHub? | `Q-BLK-18` — **A** em 2026-11-24: o aceite libera e a issue registra o vínculo no lote da onda (`../pendencias-de-fechamento.md` §6.6.27) | dono do `wyd-db` |
| a leva de aceite começa por `tests/` ou por `migrations/`? | `Q-BLK-17` — **fechada em A pela execução** (2026-11-24): a leva 1 começou por `tests/`, por razão itens/ato de 7,3 contra 1,1 | dono do `wyd-db` |

Com as duas respondidas, o código foi liberado e **4 dos 5 alvos planejados** já estão em
`crates/wyd-db/Cargo.toml` — `sistema_pg`, `reversibilidade_pg`, `runner_pg` e, desde 2026-11-27,
`contrato_migrations` —, ao lado dos contratos de API `contrato_api_migrate` e `contrato_api_provision`;
`contrato_catalogo`, `sistema_sessoes` e `sistema_stack_caps` seguem sem arquivo escrito (§2).

## 6. O que esta página não decide

- **Não define capacidade:** isso é de [specs/capacidades.md](specs/capacidades.md), dono único da matriz.
- **Não define cenário:** cada página de `alvos/` descreve os seus, com pré-requisito e leitura de falha.
- **Não decide a estratégia de teste:** ela é de `../docs/planos/plano-de-testes.md` e
  `../docs/guia-teste/`.
- **Não cria comando novo de CI:** não existe página de CI em `docs/planos/` do `wyd-db`; o plano do
  projeto é `../../wyd-ops/docs/devops/index.md` (`aceito`, 2026-11-26), e
  `docs/planos/plano-de-testes.md` §81 anuncia a página local para depois do primeiro código. Aqui só o
  comando local por alvo.
