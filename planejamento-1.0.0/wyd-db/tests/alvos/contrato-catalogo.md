> Pergunta: "o que o alvo `contrato_catalogo` prova e por que ele ainda não pode rodar?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../specs/capacidades.md` §3; `../specs/contrato.md`; `../../plano-migrations.md` §7; `../../docs/guia-teste/testes-de-contrato.md` §1
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — alvo planejado, **bloqueado por pré-requisito**; `Cargo.toml`,
código e documento de catálogo não existem.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre o alvo: `contrato_catalogo` não existe, e o documento
  `docs/dicionario-dados/banco-catalogo.md` também não.
- **`H`** — dúvida aberta, dita e localizada em §6, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

**Alvo escrito:** `contrato_catalogo` — arquivo `crates/wyd-db/tests/contrato_catalogo.rs`, ainda fora do
`Cargo.toml`; o nome do alvo é o de [../index.md](../index.md) §2.

Prova que o **documento de catálogo** e a **série** descrevem o mesmo banco: as mesmas 109 tabelas e as
mesmas 2 views, todas com schema. É o único alvo de contrato que compara dois artefatos, em vez de ler um.

## 1. Capacidade coberta

| `CAP` | Capacidade | Cenário |
|---|---|---|
| `CAP-08` | catálogo e série descrevem as mesmas 109 tabelas e 2 views, com schema | `catalogo_igual_a_serie` |

## 2. Cenário

| Passo | Entrada | Resultado esperado |
|---|---|---|
| 1 | documento de catálogo (`docs/dicionario-dados/banco-catalogo.md`) | lido como texto, sem conexão |
| 2 | série viva, por `serie.rs` | lista de `CREATE TABLE`/`CREATE VIEW` por arquivo |
| 3 | comparação | mesmos nomes, mesmo schema, mesma quantidade: **109** tabelas e **2** views |
| 4 | diferença | falha nomeando o que só existe de um lado, com arquivo ou seção |

O cenário não compara coluna: coluna, tipo e `CHECK` são do dicionário por tabela e da página da
migration (`CAP-06`/`CAP-07` cobrem a forma). Comparar coluna aqui seria um segundo dono da forma.

## 3. Pré-requisito — e por que ele não está cumprido

| Pré-requisito | Estado em 2026-11-24 | Efeito |
|---|---|---|
| documento de catálogo | **não existe**: `docs/dicionario-dados/banco-catalogo.md` depende do PR de ferramentas (`../../docs/guia-desenvolvedor/como-adicionar-migration.md` §"catálogo") | sem lado esquerdo, não há comparação |
| leitor de catálogo | o `banco-parse` em uso ancora `CREATE TABLE` em nome simples e **não enxerga** `CREATE TABLE game.account (` | o lado da série também não é lido hoje |
| 109 tabelas e 2 views | fechadas no DDL (`../../plano-migrations.md` §7, gate `catálogo`) | contagem de referência existe; a prova, não |

**Consequência declarada:** `CAP-08` é **0 coberta**, e a paridade catálogo × série hoje é conferência
manual. O alvo é desenho até o PR de ferramentas — e a página não cita verde que não existe.

**Destino de `CAP-08` na `1.0.0`, decidido em 2026-11-24:** `Q-BLK-20` = **A**
(`../../pendencias-de-fechamento.md` §6.6.31). O dicionário fica **pós-`1.0.0`**, `CAP-08` é
**declarada não executável na `1.0.0`** e o gate `catálogo` é conferência manual até o PR de ferramentas;
nenhuma migration `.sql` fica bloqueada por ela.

## 4. Como interpretar a falha

- **Tabela só na série:** migration criou tabela que o dicionário não conhece — corrigir o dicionário na
  mesma entrega, ou remover a tabela.
- **Tabela só no catálogo:** documento gerado de uma série antiga, ou tabela removida sem atualizar o
  catálogo.
- **Contagem 108 ou 110:** a série divergiu do DDL fechado; comparar com `../../ddl-confirmado.md` §1.
- **View faltando:** `0029` não foi lida ou o leitor ignorou `CREATE VIEW`.

## 5. Tempo esperado

**Não há tempo medido**, e o alvo nem roda: tempo entra quando houver execução, com data. Enquanto isso,
o campo fica declarado como pendente, não estimado.

## 6. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| o documento de catálogo entra na `1.0.0` ou fica pós-`1.0.0`? | **decidida em 2026-11-24 (`A`)**: pós-`1.0.0`, com `CAP-08` declarada não executável (`../../pendencias-de-fechamento.md` §6.6.31) | dono do `wyd-db` |
| quando o PR de ferramentas for autorizado, o leitor novo enxerga schema qualificado e view? | é o critério de pronto do PR; hoje o `banco-parse` não enxerga `CREATE TABLE esquema.tabela` | dono do PR de ferramentas |

## 7. O que este alvo não prova

- **Coluna, tipo, nulidade, `CHECK`:** dicionário por tabela e `CAP-06`/`CAP-07`.
- **Comportamento:** `sistema_pg`.
- **Privilégio:** `sistema_sessoes`.
- **Cobertura:** a matriz é por capacidade; este alvo cobre uma.
