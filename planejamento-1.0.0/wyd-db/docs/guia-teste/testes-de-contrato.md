> Pergunta: "como provo que o schema e o catálogo batem, sem abrir conexão?"
> Público: Desenvolvedor do `wyd-db` e revisor de migration
> Dono: equipe do `wyd-db`
> Base: `padrao-banco.md` §10; `plano-migrations.md` §1.1, §1.3 e §7; `migrations-checklist.md` §5; `src/migrate/spec.md` §2
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Teste de contrato é o que prova a **forma** da série sem PostgreSQL: os arquivos são contíguos, cada
`down` derruba o que o seu `up` criou, nenhuma FK aponta para arquivo posterior, seed só aparece
depois da tabela que ele preenche, toda constraint tem nome e nenhum tipo proibido entrou. Ele não
prova comportamento — `CHECK` que dispara, privilégio que nega, transação que desfaz —; isso é do
teste de sistema. A divisão das camadas é de `padrao-banco.md` §10.

## 1. O que o contrato lê

O alvo percorre o diretório da série **como texto**, sem abrir conexão: nome de quatro dígitos,
`-- migrate:up` e `-- migrate:down` sozinhos na linha, `CREATE TABLE schema.tabela (` na linha e `);`
sozinho no fim. A leitura é a mesma que o runner faz; o teste não inventa um segundo parser
(`src/migrate/spec.md` §2). O alvo lê dois caminhos **constantes**: `migrations/` (série viva) e
`tests/resources/serie/` (fixtures), sem `SERIE` nem `--dir` — parâmetro por ambiente tornaria o
resultado dependente de quem executa (`tests/specs/contrato.md` regra 7).

**Limite atual, declarado:** o leitor de catálogo em uso (`banco-parse`) ancora `CREATE TABLE` em
nome simples e **não enxerga** `CREATE TABLE game.account (`; por isso o contrato da série nova ainda
não roda nesse leitor e é conferência manual até o PR de ferramentas
(`plano-migrations.md` §1.1 e §1.3) — esse limite é do leitor de catálogo, que é `contrato_catalogo`.
O alvo `contrato_migrations` roda desde 2026-11-27 e não usa esse leitor: nove testes em 0,09 s, sem
banco (`tests/alvos/contrato-migrations.md` §1).

## 2. O que ele reprova

| Invariante | Como o defeito aparece | Erro esperado | Correção |
|---|---|---|---|
| série contígua | falta um número entre `0001` e `0034`, ou há dois com o mesmo número | `série não contígua` | renumerar o arquivo e a página; a numeração é de `plano-migrations.md` §3 |
| `down` completo | uma tabela criada no `up` sobrevive ao `down` do próprio arquivo | `tabela sobrevive ao down` | acrescentar o `DROP` na ordem inversa, no mesmo arquivo |
| FK para o futuro | `REFERENCES` a tabela criada em arquivo posterior | `FK para arquivo posterior` | mover a FK para o arquivo que já tem a tabela, ou reordenar a série |
| seed fora de ordem | `INSERT` em arquivo antes do `CREATE TABLE` da tabela | `seed antes da tabela` | mover o seed para depois do DDL do domínio |
| seed sem marca | `INSERT` em arquivo cujo nome não tem `_seed_` | `seed em arquivo de DDL` | separar o seed em arquivo próprio |
| constraint anônima | `PRIMARY KEY`, `UNIQUE`, `CHECK` ou `FOREIGN KEY` sem `CONSTRAINT nome` | `constraint anônima` | nomear com `pk_`, `uq_`, `ck_` ou `fk_` |
| tipo proibido | `serial`, `timestamp` sem fuso, `time`, `money`, `varchar` ou `enum` em coluna | `tipo proibido` | trocar pelo tipo do padrão (`BIGINT IDENTITY`, `TIMESTAMPTZ`, `BIGINT`, `TEXT`) |

As sete linhas saem de `padrao-banco.md` §10 e de `plano-migrations.md` §7; o detalhe de cada tipo e
de cada nome está em [`padroes-postgres.md`](../arquitetura/padroes-postgres.md) e em
[`integridade-e-constraints.md`](../arquitetura/integridade-e-constraints.md).

## 3. Como corrigir quando falha

1. **Leia o arquivo e a linha**, não a mensagem isolada: o contrato aponta o arquivo, e a correção é
   no DDL ou no nome, não no teste.
2. **Corrija a origem**, não o sintoma: renumerar exige atualizar a página, o checklist e as citações
   de contagem na mesma entrega; nomear constraint exige a página de migration e o dicionário.
3. **Rode de novo o alvo** e o de sistema quando a correção tocar privilégio, `CHECK` ou `down`.
4. **Não silencie**: `#[ignore]`, filtro de arquivo ou exceção no leitor é reprovação, não correção
   (`SKIP` proibido, `wyd-db-1.0.0.md` §5.9).

## 4. O que o contrato não prova

| Fora do contrato | Onde é provado |
|---|---|
| `CHECK` que dispara e devolve `23514` | `sistema_pg` |
| `UNIQUE`/FK que devolvem `23505`/`23503` | `sistema_pg` |
| `permission denied` por papel | `sistema_sessoes` e a fronteira de `plano-migrations.md` §7 |
| `down` contra PostgreSQL real, com 0 tabela ao fim | `sistema_pg` (`migrate-cycle`) |
| concorrência, lock e isolamento | `sistema_pg` e o teste de sessão |
| documento de catálogo ↔ série | `contrato_catalogo` (o par do contrato) |

O contrato é a primeira barreira, não a prova final: um schema pode passar nele e falhar no banco.

## 5. Estado e o que não cobre

- **Alvo:** `contrato_migrations` está declarado no `Cargo.toml` desde 2026-11-27 e roda em
  `tests/contrato/contrato_migrations.rs`; `CAP-02`–`CAP-07` passam na série viva, `CAP-01` não —
  faltam `0016`, `0017`, `0018`, `0020`, `0023` e `0027` (`tests/alvos/contrato-migrations.md` §1).
- **Leitor:** o parser atual não entende schema qualificado, view nem `INET`; até o PR de
  ferramentas, a conferência é manual (`plano-migrations.md` §1.3).
- **Fixture:** `tests/resources/serie/valida/` tem **4** arquivos e cada uma das **7** pastas
  `invalida/<defeito>/` tem a série que viola **um** invariante (`tests/resources/README.md` §3).
- **Cobertura:** este teste não mede cobertura de capacidade; a matriz está em
  `tests/specs/capacidades.md`, e os contratos cobrem `CAP-01`–`CAP-08`.
