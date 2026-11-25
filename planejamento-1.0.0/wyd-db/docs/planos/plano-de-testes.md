> Pergunta: "como provamos o `wyd-db` e quando a `1.0.0` pode sair?"
> Público: QA, desenvolvedor, arquiteto e operação
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.17 e §6.9; `plano-migrations.md` §7; `docs/guia-teste/`; `plano-de-qualidade-e-gates.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-22)

O plano de testes define **o que precisa ser provado**, em qual alvo e com qual dado. Ele não inventa
capacidade: a matriz `CAP-nn` está escrita fora do site, em `tests/specs/capacidades.md` — **39**
capacidades em **5** alvos, **0** cobertas —, e esta página aponta para ela em vez de repetir. Os cinco
alvos estão planejados e a suíte não foi implementada; a matriz existe como planejamento, o que muda a
natureza da lacuna: o critério 6 deixou de ser "matriz ausente" e passou a ser "matriz preenchida, teste
não escrito". O critério de saída só vale quando o alvo existir e produzir evidência; "não rodou" não é
verde.

## 1. Escopo

| Entra | Não entra |
|---|---|
| forma da série e do catálogo, sem banco | teste de balanceamento ou regra de jogo |
| comportamento contra PostgreSQL 17 real | teste de interface do portal ou do cliente |
| isolamento por papel e por sessão | carga, stress e capacidade sem volume medido |
| ciclo `up` → `down` → `up` | ETL da base legada |
| contrato do runner com portas falsas | CI real, enquanto o workflow não existir |

## 2. Alvos planejados

| Alvo | O que prova | Precisa de banco | Fonte |
|---|---|---|---|
| `contrato_catalogo` | o documento de catálogo e a série descrevem as mesmas tabelas | não | `guia-teste/testes-de-contrato.md` |
| `contrato_migrations` | série contígua, `down` completo, FK para o futuro, seed fora de ordem, constraint anônima e tipo proibido | não | `guia-teste/testes-de-contrato.md` |
| `sistema_pg` | aplicação, reaplicação, drift, ciclo de `down`, lock e concorrência | sim | `guia-teste/testes-de-sistema.md` |
| `sistema_sessoes` | `permission denied` por papel e leitura cruzada só por view | sim | `guia-teste/testes-de-sistema.md` |
| `sistema_stack_caps` | a matriz de capacidades e o anti-skip | sim | `tests/specs/capacidades.md` |

Os cinco nomes são os de `wyd-db-1.0.0.md` §6.17; o `Cargo.toml` existe desde 2026-11-24 e ainda não declara esses alvos. A coluna
“precisa de banco” separa o que pode rodar sem PostgreSQL do que exige um banco descartável.

## 3. Matriz capacidade × alvo

A matriz `CAP-nn → alvo → cenário → requisito` existe em `tests/specs/capacidades.md`, fora do site por
não ser publicada: **39** capacidades, **5** alvos, **0** cobertas por teste executado. Ela não é copiada
nem substituída por uma tabela paralela aqui — esta página é dona da **estratégia**, a matriz é dona da
**capacidade**. Cada capacidade nova entra na matriz e ganha alvo antes de o teste ser aceito. A regra de
cobertura continua a de [`../guia-teste/gates-e-anti-skip.md`](../guia-teste/gates-e-anti-skip.md):
nenhum teste sem capacidade, nenhuma capacidade sem teste.

## 4. Dado e ambiente

| Item | Regra |
|---|---|
| banco sem PostgreSQL | contrato lê a série como texto e não abre conexão |
| banco real | PostgreSQL 17 descartável; DSN em `WYD_DB_URL_TEST` |
| isolamento | cada teste usa chave própria e limpa o que criou; nada de estado global |
| dado de conteúdo | fixture explícita; seed de desenvolvimento não é dado de produção |
| segredo | nenhum valor real em fixture, log ou snapshot |
| id | nada de id mágico; a factory devolve o identificador criado |

O DSN de teste é o único ambiente que o alvo de sistema lê; a lista completa de variáveis por papel
está em `src/provision/spec.md` §3, e o guia não a duplica.

## 5. Critério de saída da `1.0.0`

1. `contrato_migrations` e `contrato_catalogo` passam na série viva.
2. `sistema_pg` prova aplicar, reaplicar, detectar drift e desfazer até zero.
3. `sistema_sessoes` prova a fronteira de escrita e a leitura por view.
4. Nenhum `SKIP`, `#[ignore]` novo ou filtro silencioso no caminho de aceite.
5. O corte vertical de `plano-migrations.md` §3.2 passa com o cliente 7662 (evidência manual datada).
6. A matriz `CAP-nn` está preenchida e cada capacidade aponta alvo e cenário — ou a lacuna está
   declarada no relatório de saída.

Os itens 1–3 e 5 não podem ser marcados sem a suíte e a série. O item 6 tem a primeira metade cumprida: a
matriz está preenchida com **39** capacidades e **0** cobertas, e o que falta é o alvo existir. O que o
plano **não** mede é cobertura por linha, tempo de execução ou custo por alvo.

## 6. O que este plano não cobre

- **Teste de carga, stress e capacidade:** não há volume nem limiar aprovado.
- **Teste de interface e E2E:** pertencem aos sistemas consumidores, não ao crate.
- **CI real:** é de `plano-ci-real.md` e será aplicado depois do primeiro código; o gate local de
  docs não substitui teste do crate.
- **Cobertura percentual:** a regra é por capacidade, não por linha; não há número de cobertura.

## CI, cobertura e monitoramento

- **CI:** o job `code` de `.github/workflows/code.yml` roda `cargo fmt --all -- --check`, `cargo clippy -p wyd-db --all-targets -- -D warnings`, `cargo test -p wyd-db --all-targets` e `cargo deny check`; o PR para `develop` e `main` só passa com os quatro verdes.
- **Cobertura:** `cargo llvm-cov -p wyd-db --fail-under-lines 85` é o piso; a cobertura de linha não substitui os comportamentos listados acima e `#[ignore]` exige issue dona e prazo.
- **Monitoramento:** Pool em uso, latência por consulta, conexões esperando, locks, tamanho e crescimento por tabela, além do ledger de migrations. Log sem DSN, SQL de jogador ou payload; alerta de pool esgotado e migration divergente.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.
