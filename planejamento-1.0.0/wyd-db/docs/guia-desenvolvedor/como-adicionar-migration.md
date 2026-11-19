> Pergunta: "como crio a próxima migration?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-db
> Base: `plano-migrations.md`, `padrao-banco.md`
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18)

A série da `1.0.0` é escrita do zero e o arquivo novo é sempre o **próximo número livre**. Nada
entra no schema por fora dela — nem `ALTER` manual, nem script de operação, nem "ajuste rápido" no
banco de desenvolvimento (regras da série em `plano-migrations.md` §2; a invariante "nada por fora da
série" é de [`migracoes.md`](../arquitetura/migracoes.md) §4). Um arquivo, um número, uma tabela ou um
grupo coeso de tabelas.

## Regra da série

| Regra | Por quê |
|---|---|
| ordem e nome de cada arquivo são de `plano-migrations.md` §3 | é o dono único da numeração; duas listas divergem por construção |
| `up` e `down` no mesmo arquivo (`plano-migrations.md` §2) | o `down` prova que o `up` é desfazível; separá-los deixa o segundo apodrecer |
| DDL consolidado: o arquivo cria a tabela inteira, sem `ALTER` | série de instalação, não de evolução; `plano-migrations.md` §2 |
| toda instrução qualificada (`schema.tabela`), 100% das constraints nomeadas | o schema é lido por quatro papéis; nome implícito é nome que ninguém acha (`padrao-banco.md` §1 `P-01`, §3 `P-13`) |
| tipos de `padrao-banco.md` §4 (`bigint`, `timestamptz`, `text`, `bytea`; `numeric` só para grandeza exata não monetária, `P-07`) | `serial`/`timestamp`/`money`/`varchar`/`enum` são reprovados pelo gate |
| seed no mesmo arquivo, com `down` que restaura o estado anterior | `plano-migrations.md` §4; seed órfão não tem rollback |
| dependência só para arquivo **anterior** | FK para tabela futura não aplica e não tem `down` (`plano-migrations.md` §2) |

## Passo a passo

1. **Número.** Pegue o último da série + 1 e confirme em `plano-migrations.md` §3 qual assunto cabe
   nele. Nome fora da tabela é de `plano-migrations.md` §3, não seu.
2. **`up`.** Escreva o `CREATE TABLE` completo, com PK, FK, `CHECK` e índices no mesmo arquivo, na
   ordem que aplica sem erro. Coluna sem decisão fechada não entra: abre lacuna em `lacunas.md`.
3. **`down`.** Espelhe na ordem inversa — índices, depois dados, depois a tabela. Confira que
   rodar `up` → `down` → `up` deixa o banco idêntico.
4. **Gate local.** Rode o gate de `plano-migrations.md` §7 (estrutura, tipos, constraints,
   catálogo) e `make check` quando o `Makefile` da série existir.
5. **Regenerar.** `make banco-catalogo` e o gerador do dicionário; nunca edite `gerado/` à mão —
   gerado × escrito é de `wyd-db-1.0.0.md` §3, item 3, e a divergência é defeito registrado (§8.2).
6. **Página.** Abra a página da migration no molde de `wyd-db-1.0.0.md` §7.2 e registre linha em
   `migrations-checklist.md`.

O runner conecta com `WYD_DB_URL_MIGRATOR` e lê o diretório da série em `WYD_MIGRATIONS_DIR`; nomes,
papéis e o efeito de variável ausente são de `src/provision/spec.md` §3. O contrato de configuração do
runner — variável ausente, faixa do `lock_timeout`, papel e versão de servidor esperados — está em
`src/migrate/config.md`, e as variantes de erro (inclusive diretório da série ausente, que não é falha
de conexão) em `src/migrate/error.md` §2–§3.

## O que nunca fazer

- `ALTER TABLE` no `up` para "completar" um arquivo anterior — o anterior está errado, e o conserto
  é antes de a série ser aplicada.
- Seed de conteúdo antes do DDL que a recebe, ou seed sem contagem conferida contra `plano-migrations.md` §4.
- FK para tabela que ainda não existe, ou `CASCADE` em dinheiro, crédito e auditoria
  (`plano-migrations.md` §2; `padrao-banco.md` §5, item 7).
- Editar arquivo já aplicado: o checksum em `schema_migrations` acusa drift e o runner para
  (`src/migrate/spec.md` §4).
- Criar objeto com nome implícito (`person_pkey` sem dono declarado) ou `CHECK` anônimo — a convenção
  de nome é de `padrao-banco.md` §3 (`P-13`), e esta página não a redigita.
- Tabela no plural, coluna `varchar`/`char`, ou `text` guardando número (`padrao-banco.md` §1 `P-02`,
  §4 e §11).

## Como regenerar catálogo e dicionário

O catálogo (`docs/dicionario-dados/banco-catalogo.md`) e as páginas `gerado/<tabela>.md` vêm do
banco aplicado, não da sua memória do DDL. Depois de aplicar a série: gere, confira o diff e
commite o resultado junto com o arquivo. Divergência entre `gerado/` e a migration é defeito de
entrega, não detalhe de formatação.

## Gate

O aceite é o de `plano-migrations.md` §7 — nenhum arquivo é "pronto" por aplicar sem erro. Enquanto
o linter da série não existir, os critérios de tipos e de nome de constraint são conferidos à mão e
o resultado fica na página da migration.
