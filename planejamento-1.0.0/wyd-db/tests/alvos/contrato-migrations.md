> Pergunta: "o que o alvo `contrato_migrations` prova, com que entrada e como leio a falha?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../specs/capacidades.md` §2; `../specs/contrato.md`; `../../docs/guia-teste/testes-de-contrato.md`
> Revisado em: 2026-11-27

**Estado:** aceito (2026-11-24), implementado em 2026-11-27 — alvo em `Cargo.toml` (`[[test]] name = "contrato_migrations"`),
código em `crates/wyd-db/tests/contrato/contrato_migrations.rs`, fixtures em
`crates/wyd-db/tests/resources/serie/`. Nove testes verdes, sem banco.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — o que está planejado e ainda não roda: nada nesta página; as sete capacidades rodam, e
  `CAP-01` roda **contra a série viva com resultado vermelho declarado** (§1 e §5).
- **`H`** — dúvida aberta, dita e localizada em §6, com o dono que a fecha. Hoje não há nenhuma.
- **`V`** — o que foi medido nesta árvore em 2026-11-27, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

Prova a **forma** da série sem abrir conexão: sete invariantes, um cenário por invariante. Não prova
comportamento — `CHECK` que dispara ou privilégio que nega são de `sistema_pg` e `sistema_sessoes`.

## 1. Capacidades cobertas

**Medido em 2026-11-27** com `cargo test --manifest-path crates/wyd-db/Cargo.toml --test contrato_migrations`
(9 testes, 0,09 s, sem `WYD_DB_TEST_URL`):

- a série viva (`crates/wyd-db/migrations/`, **28** arquivos) passa em `CAP-02`–`CAP-07` com **zero**
  defeito em cada um;
- a série viva **reprova** em `CAP-01`: faltam `0016`, `0017`, `0018`, `0020`, `0023` e `0027`.

A tabela abaixo diz onde cada invariante é provado. `CAP-01` prova a **regra** nas fixtures e fixa o
estado medido da série viva num teste próprio (`serie_viva_respeita_a_forma_com_contiguidade_pendente`);
não é capacidade coberta enquanto os seis arquivos faltarem (`../specs/capacidades.md` §7).

| `CAP` | Capacidade | Cenário | Série viva hoje |
|---|---|---|---|
| `CAP-01` | 34 arquivos, `0001`–`0034`, sem furo nem repetição | `serie_contigua` | **reprova**: 28 arquivos, seis furos |
| `CAP-02` | `down` derruba o que o `up` criou | `down_completo` | passa |
| `CAP-03` | nenhuma FK aponta para arquivo posterior | `fk_sem_futuro` | passa |
| `CAP-04` | seed só depois da tabela | `seed_depois_da_tabela` | passa |
| `CAP-05` | seed só em arquivo `_seed_` | `seed_em_arquivo_proprio` | passa |
| `CAP-06` | toda constraint nomeada | `constraint_nomeada` | passa |
| `CAP-07` | nenhum tipo proibido | `tipo_permitido` | passa |

## 2. Cenários

| Cenário | Entrada | Resultado esperado | Falha esperada |
|---|---|---|---|
| `serie_contigua` | `serie/valida/` e `serie/invalida/furo/`; a série viva só para medir | a boa passa; a `furo/` falha nomeando o número ausente | `série não contígua: falta o arquivo 0002` |
| `down_completo` | série viva **e** `serie/invalida/down-incompleto/` | toda tabela do `up` aparece no `down` do mesmo arquivo | `tabela sobrevive ao down: fixture.extra` com arquivo e linha |
| `fk_sem_futuro` | série viva **e** `serie/invalida/fk-futura/` | `REFERENCES` só para tabela de arquivo igual ou anterior | `FK para arquivo posterior: fixture.b nasce em 0002` |
| `seed_depois_da_tabela` | `serie/invalida/seed-antes/` | `INSERT` só depois do `CREATE TABLE` | `seed antes da tabela` com arquivo e linha |
| `seed_em_arquivo_proprio` | `serie/invalida/seed-sem-marca/` | `INSERT` só em arquivo com `_seed_` no nome | `seed em arquivo de DDL` |
| `constraint_nomeada` | série viva **e** `serie/invalida/constraint-anonima/` | 100% com `CONSTRAINT nome` | `constraint anônima` com arquivo e linha |
| `tipo_permitido` | série viva **e** `serie/invalida/tipo-proibido/` | nenhum tipo da lista proibida | `tipo proibido: varchar` com arquivo e linha |

Cada cenário roda duas vezes: contra a fixture que passa e contra a que viola **só** aquele invariante.
Um defeito por fixture, e o teste exige que a fixture ruim produza **exatamente uma** mensagem — fixture
que viola dois invariantes prova que o alvo reprova, não qual regra ele confere.

O `up`/`down` de cada arquivo é separado pelo **parser do runner**
(`wyd_db::migrate::migration::file::parse`), que também valida o nome `<NNNN>_<slug>` e os marcadores
(regra 2 de `../specs/contrato.md`). Os analisadores do alvo só localizam o texto de cada invariante, e
ignoram comentário: prosa que cita `references` ou nome de coluna com `time` não vira defeito.

## 3. Pré-requisito de ambiente

| Item | Regra |
|---|---|
| banco | **nenhum**; o alvo não abre conexão |
| diretório | fixo no código: `migrations/` para a série viva, `tests/resources/serie/` para as fixtures |
| `WYD_DB_TEST_URL` | **não** é lida por este alvo |
| arquivos | `resources/serie/valida/` (4) e as sete `invalida/<defeito>/` (8) existirem |

**O parâmetro de diretório não existe** (`--dir`, `SERIE` ou argumento): a série viva e as fixtures são
caminhos constantes, para o alvo ser determinístico sem depender de `env` (regra 7). Apontar outra série
é editar a constante, não passar argumento.

## 4. Como interpretar a falha

- **Erro com arquivo e linha:** defeito de série, para corrigir na migration, não no teste.
- **Fixture boa reprovando:** o invariante está mais estrito que a série real, ou a fixture deixou de ser
  válida — decidir qual dos dois antes de mexer em qualquer um.
- **Fixture ruim passando:** o teste não cobre o invariante; é falha do alvo, não da série.
- **`serie_viva_respeita_a_forma_com_contiguidade_pendente` reprovando:** a série viva mudou de forma, ou
  os seis furos de `CAP-01` foram fechados — atualizar §1 e `../specs/capacidades.md` §7 na mesma rodada.
- **Nenhum erro e zero arquivo lido:** caminho errado; o alvo falha dizendo o diretório.

## 5. Tempo esperado

**Medido em 2026-11-27:** `cargo test --test contrato_migrations` roda os 9 testes em **0,09 s** com a
árvore já compilada (compilar é o custo, não rodar), sem banco. É o alvo mais barato da suíte: só lê
`.sql`.

## 6. Dúvidas abertas

Nenhuma. As duas de 2026-11-24 foram fechadas pelo autor do alvo nesta implementação:

| Dúvida | Decisão | Onde aparece |
|---|---|---|
| nome do parâmetro que aponta a série | **não há parâmetro**: caminho constante (§3) | `SERIE_VIVA` e `FIXTURES` no alvo |
| texto exato da mensagem | `<arquivo>:<linha>: <invariante>: <detalhe>`, por exemplo `0001_fixture_base.sql:22: tabela sobrevive ao down: fixture.extra` | `impl Display for Defeito` |

## 7. O que este alvo não prova

- **Comportamento:** `CHECK` que dispara, transação que desfaz, privilégio que nega — `sistema_pg` e
  `sistema_sessoes`.
- **Paridade com o catálogo:** é `contrato_catalogo` (`CAP-08`).
- **Série aplicada:** aqui os arquivos são texto; aplicar é do runner.
- **Contiguidade da série viva:** medida e declarada vermelha (§1); o alvo não conserta a série.
- **Cobertura de linha:** a regra é por capacidade (`../specs/capacidades.md` §7).
