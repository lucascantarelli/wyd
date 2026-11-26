> Pergunta: "como o erro do `migrate`/`provision` aparece, qual código sai e o que nunca vai para o log?"
> Público: Desenvolvedor, operador e revisor de segurança
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §11–§12; `src/provision/spec.md` §6; `wyd-db-1.0.0.md` §6.11
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-22)

A taxonomia é fechada: código novo é decisão registrada, não `catch-all`. O erro carrega contexto
(`Version`, nome de variável, papel), mas **não** carrega DSN, senha, token, PIN ou o texto do SQL. Esta
página cobre `migrate` e `provision`; erros de runtime de jogo e portal não têm desenho publicado.

## 1. Códigos do runner

| Código | Situação | O que o operador vê |
|---:|---|---|
| 0 | aplicado, já atualizado ou `status` sem problema | resumo com `applied`, `skipped`, `pending` |
| 2 | configuração, conexão, preflight (papel, servidor, diretório) ou ledger com forma divergente | a variável, o papel esperado ou o arquivo, nunca a DSN |
| 3 | lock ocupado | “outro processo está migrando; verifique `pg_locks`” |
| 4 | drift, arquivo faltando, série vazia ou inválida | versão/arquivo e o tipo de achado |
| 5 | erro de SQL durante a aplicação | `Version` e a mensagem da porta, sem o SQL inteiro |

A tabela é a de `src/migrate/spec.md` §11. `status --check` usa 4 para drift; `apply --dry-run` não
escreve e não pega lock. O boot trata `Drift` e `Sql` como “não servir”: subir com schema divergente é
pior que ficar fora.

## 2. Códigos do provisionamento
**Sem arquivo no repositório:** `db/roles.sql`, `domain/spec.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Código | Situação | Dono |
|---:|---|---|
| 0 | `render` sem `--check`, ou `render --check` sem divergência | subcomando |
| 2 | configuração ou conexão; DSN ausente nomeia a variável | binário |
| 3 | `render --check` com divergência entre `db/roles.sql` e o renderizado | subcomando |
| 4 | `plan`/`check` com drift entre `pg_roles` e `ROLES` | `ADR-DB-0001` |

`domain/spec.rs` não devolve erro: `render()` é infalível por construção. Nenhum `Result` atravessa essa
fronteira. O módulo não formata a saída; quem formata é o binário.

## 3. O que entra no log

| Entra | Não entra |
|---|---|
| `version`, `slug`, `elapsed_ms` e resultado do arquivo | DSN, usuário com senha ou URL de conexão |
| resumo `applied`, `skipped`, `pending`, `findings` | texto do `up`/`down` (pode conter seed sensível) |
| nome da variável ausente | valor da variável |
| papel encontrado × esperado | hash de senha, PIN ou token |
| código de saída e tipo de drift | PII de jogador ou conteúdo de `portal` |

O `Debug` de `Dsn` redige a senha; o `Display` de erro não imprime o SQL. Se uma mensagem de conexão
do driver trouxer a URL, ela é substituída por uma mensagem do módulo, não logada crua.

## 4. Correlação

- Um span por arquivo: `migration.apply`, campos `version`, `slug`, `elapsed_ms`.
- Um evento de resumo no fim, com os mesmos campos de `Report`.
- O binário é quem escreve a linha final; a biblioteca devolve dado, não string.
- O operador correlaciona pelo `version`/`slug`, não pelo texto do SQL.
- Um erro de lock não vira `panic`; vira `LockBusy` com o tempo esperado.

## 5. O que esta página não cobre

- **Erros de `repo`, `pool`, `transfer` e runtime do jogo:** não têm taxonomia publicada.
- **Formato de log estruturado, retenção e destino:** dependem do ambiente e do `wyd-ops`. O que já
  está decidido desde 2026-11-23 (`M32`, `Q-DP-6`): log não carrega e-mail, login ou PIN em claro —
  use `person_id`, `account_id` ou hash; e retenção de log continua `pendente`, com dono `wyd-ops`.
- **Mensagem exata de cada driver:** o módulo traduz para o seu erro; não há promessa de texto do
  `sqlx`.
- **Diagnóstico de disco, pool ou conexões:** é de [`../guia-administrador/diagnostico.md`](../guia-administrador/diagnostico.md);
  sem medição, a página só cobre lock e ledger.
