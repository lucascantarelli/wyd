> Pergunta: "o que o alvo `contrato_migrations` prova, com que entrada e como leio a falha?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../specs/capacidades.md` §2; `../specs/contrato.md`; `../../docs/guia-teste/testes-de-contrato.md`
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — alvo planejado; `Cargo.toml` e código não existem.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre o alvo: `contrato_migrations` não existe em `Cargo.toml` nem em
  `tests/`, e as fixtures de `resources/serie/` também não.
- **`H`** — dúvida aberta, dita e localizada em §6, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

**Alvo escrito:** `contrato_migrations` — arquivo `crates/wyd-db/tests/contrato_migrations.rs`, ainda
fora do `Cargo.toml`; o nome do alvo é o de [../index.md](../index.md) §2.

Prova a **forma** da série sem abrir conexão: sete invariantes, um cenário por invariante. Não prova
comportamento — `CHECK` que dispara ou privilégio que nega são de `sistema_pg` e `sistema_sessoes`.

## 1. Capacidades cobertas

Os **34** arquivos são os de `../../plano-migrations.md` §1 ("**34** arquivos e **109** tabelas"); a
contagem **será** conferida contra a série viva e contra a fixture `invalida/furo/`, quando as duas
existirem — hoje não há `.sql` nem fixture.

| `CAP` | Capacidade | Cenário |
|---|---|---|
| `CAP-01` | 34 arquivos, `0001`–`0034`, sem furo nem repetição | `serie_contigua` |
| `CAP-02` | `down` derruba o que o `up` criou | `down_completo` |
| `CAP-03` | nenhuma FK aponta para arquivo posterior | `fk_sem_futuro` |
| `CAP-04` | seed só depois da tabela | `seed_depois_da_tabela` |
| `CAP-05` | seed só em arquivo `_seed_` | `seed_em_arquivo_proprio` |
| `CAP-06` | toda constraint nomeada | `constraint_nomeada` |
| `CAP-07` | nenhum tipo proibido | `tipo_permitido` |

## 2. Cenários

| Cenário | Entrada | Resultado esperado | Falha esperada |
|---|---|---|---|
| `serie_contigua` | série viva **e** `serie/invalida/furo/` | a viva passa; a fixture falha | `série não contígua` com o número que falta |
| `down_completo` | série viva **e** `serie/invalida/down-incompleto/` | o `down` das 34 fecha em zero tabela | `tabela sobrevive ao down` com o nome da tabela |
| `fk_sem_futuro` | série viva **e** `serie/invalida/fk-futura/` | nenhuma FK para arquivo posterior | `FK para arquivo posterior` com origem e destino |
| `seed_depois_da_tabela` | `serie/invalida/seed-antes/` | `INSERT` só depois do `CREATE TABLE` | `seed antes da tabela` |
| `seed_em_arquivo_proprio` | `serie/invalida/seed-sem-marca/` | `INSERT` de catálogo só em arquivo `_seed_` | `seed em arquivo de DDL` |
| `constraint_nomeada` | série viva **e** `serie/invalida/constraint-anonima/` | 100% com `pk_`/`uq_`/`ck_`/`fk_` | `constraint anônima` com arquivo e linha |
| `tipo_permitido` | série viva **e** `serie/invalida/tipo-proibido/` | nenhum tipo da lista proibida | `tipo proibido` com coluna e arquivo |

Cada cenário roda duas vezes: contra a série real e contra a fixture que viola o invariante. Só a
segunda prova que o teste **reprova** — teste que só vê o caso bom não distingue "passou" de "não olhou".

## 3. Pré-requisito de ambiente

| Item | Regra |
|---|---|
| banco | **nenhum**; o alvo não abre conexão |
| diretório | o da série viva, ou a fixture por `--dir`/`SERIE` |
| `WYD_DB_URL_TEST` | **não** é lida por este alvo |
| arquivos | `resources/serie/valida/` e as sete `invalida/<defeito>/` existirem |

## 4. Como interpretar a falha

- **Erro com arquivo e linha:** defeito de série, para corrigir na migration, não no teste.
- **Fixture boa reprovando:** o invariante está mais estrito que a série real, ou a fixture deixou de ser
  válida — decidir qual dos dois antes de mexer em qualquer um.
- **Fixture ruim passando:** o teste não cobre o invariante; é falha do alvo, não da série.
- **Nenhum erro e zero arquivo lido:** caminho errado; o alvo falha dizendo o diretório.

## 5. Tempo esperado

**Não há tempo medido.** O alvo não roda, e esta página não inventa número: tempo esperado entra aqui
quando houver execução registrada, com a data.

## 6. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| o nome do parâmetro que aponta a série (`--dir`, env ou argumento) e o da série viva | não está decidido; `../resources/README.md` §3 cita `--dir` como exemplo | autor do alvo |
| a mensagem do cenário que falha precisa citar o número ausente; texto exato não fixado | a regra 6 de `../specs/contrato.md` exige arquivo, linha e invariante — o formato fica no PR | autor do alvo |

## 7. O que este alvo não prova

- **Comportamento:** `CHECK` que dispara, transação que desfaz, privilégio que nega — `sistema_pg` e
  `sistema_sessoes`.
- **Paridade com o catálogo:** é `contrato_catalogo` (`CAP-08`).
- **Série aplicada:** aqui os arquivos são texto; aplicar é do runner.
- **Cobertura de linha:** a regra é por capacidade (`../specs/capacidades.md` §7).
