# WYDDB-030 — testar sistema PostgreSQL e ciclo

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 6 — testes de lacuna e harness


## Contexto

Só o banco real prova `CHECK`, FK, `down` e concorrência; o contrato de texto não substitui o efeito transacional.

## Entregável

Criar `sistema_pg` com aplicação da série, reaplicação sem mudança, drift de um byte, ciclo `down --to 0` e dois processos concorrentes.

## DoD

- [x] Teste aplica a **série que o runner aceita** — a fixture contígua de quatro arquivos, porque a
      série materializada tem 28 dos 34 e o `MigrationSet` recusa furo (`Q-ALVO-2` = B1, 2026-11-25) — e
      confere o ledger com o número real de arquivos da série aplicada: 4 linhas, checksum de 64
      hexadecimais e slug correto.
- [x] Reaplicar não muda nada; drift de **um byte** no arquivo falha com código 4.
- [x] `down --to 0` deixa zero tabela/view e ledger vazio; o `apply` seguinte reaplica.
- [x] Dois processos: um aplica, o outro espera e termina sem aplicar.

O critério 1 foi reescrito em 2026-11-26 com aprovação do dono para refletir `Q-ALVO-2` = B1: a prova
contra a série materializada (34 arquivos e checksums reais) fica pendente da curadoria dos seis seeds
ausentes, dona da `WYDDB-031`, e não é pré-requisito desta fatia.

## Evidência de fechamento (2026-11-26)

`V` **(execução no CI, prova que fecha a fatia):** workflow `ci-db`, execução nº 29 (`45911198818`),
https://github.com/lucascantarelli/wyd/actions/runs/45911198818, commit
`e671cb78d558fa21fe78c973239616c2932db3dd` (PR #222). Os três modos da matriz `pg` passaram —
`pg / schema`, `pg / reversibility` e `pg / runner` — e o alvo `runner_pg` terminou **7 passed;
0 failed; 0 ignored** em 21,86 s contra o service container `postgres:17` (job `140219739708`).
`gate-db`, `gate-code` e `gate-docs` fecharam verdes no mesmo commit.

`V` **(execução local, mesmo dia):** mesmo comando com `WYD_DB_TEST_URL` do papel `wyd_migrator` em
`postgres:17` descartável, TCP em `127.0.0.1`: **7 passed; 0 failed; 0 ignored** (33,43 s na execução
local; a prova que fica registrada é o job `140219739708` do CI). O local e o CI usam a mesma DSN e o
mesmo papel; o CI é a prova registrada na issue.

| Critério | Cenário que prova | Resultado medido |
|---|---|---|
| 1 | `apply_grava_ledger_real_e_e_idempotente` | 4 versões aplicadas; ledger com 4 linhas, checksum de 64 hex e slug `fixture_base`; segunda execução não escreve |
| 2 | `apply_grava_ledger_real_e_e_idempotente` + `drift_de_um_byte_no_arquivo_reprova_com_codigo_4` | reaplicar devolve 4 puladas e ledger intacto; um byte trocado em `0003_fixture_view.sql` produz `Drift(Checksum)` na versão 3 com código 4 e o `apply` recusa sem escrever |
| 3 | `down_ate_vazio_esquece_o_ledger` | tabela, view e índice ausentes, ledger com 0 linha; o `apply` seguinte reaplica as 4, o ledger volta a 4 e o seed roda uma vez |
| 4 | `dois_processos_o_segundo_espera_no_lock_e_termina_sem_aplicar` | segundo processo (reexecução do binário de teste, espera de 2 s) sai com código 3 sem preparar o ledger; solto o lock, o primeiro aplica as 4 e o segundo volta, sai com 0 e não reaplica |

Os outros três casos do alvo (`arquivo_que_falha_nao_commita_parcial`, `drift_de_checksum_reprova_status_e_apply`
e `cap37_segunda_execucao_espera_no_advisory_lock`) continuam verdes e fecham `CAP-37`.

O detalhe dos cenários, o pré-requisito de ambiente e a leitura de falha estão em
`planejamento-1.0.0/wyd-db/tests/alvos/runner-pg.md`.

## Fora de escopo

Não testar carga, latência ou volume; não usar banco compartilhado de produção.

## Risco

O ciclo completo pode ser lento e flaky; isolamento e timeout são parte do harness, não um `sleep`.

## Depende de

WYDDB-013, WYDDB-028.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do alvo com `WYD_DB_TEST_URL` — o nome que o alvo lê — e banco descartável.
