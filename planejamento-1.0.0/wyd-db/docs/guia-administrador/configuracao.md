> Pergunta: "quais parâmetros existem, o que cada um faz e onde ele é lido?"
> Público: Operação e quem instala o `wyd-db`
> Dono: equipe do `wyd-db` com a operação
> Base: `src/provision/spec.md` §3 e §7; `src/migrate/spec.md` §7 e §9; `docs/arquitetura/seguranca.md`; `lacunas.md` `M24` e `M27`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O contrato de ambiente do `wyd-db` tem uma regra que vale mais que a lista: **cada processo lê só a
DSN do seu papel**, e DSN ausente é erro que **nomeia a variável** — nunca um `localhost` silencioso.
Não existe `DATABASE_URL` na `1.0.0`: um nome único para todos os papéis é como um superusuário entra
em runtime sem ninguém notar (`src/provision/spec.md` §3). Esta página lista os nomes e o efeito; os
**valores** são do ambiente e nunca aparecem aqui.

## 1. DSNs por papel

| Parâmetro | Efeito | Default | Obrigatório | Onde é lido |
|---|---|---|---|---|
| `WYD_PROVISION_DSN` | DSN do admin do cluster **e** dono do banco; só o provisionamento usa | nenhum | sim, para `provision` | binário `wyd-db`, subcomandos de `provision` |
| `WYD_DB_URL_MIGRATOR` | DSN do papel `wyd_migrator`; o runner recusa outro papel | nenhum | sim, para o runner | runner de migrations |
| `WYD_DB_URL_GAME` | DSN do papel `wyd_game` | nenhum | sim, para o servidor de jogo | servidor de jogo |
| `WYD_DB_URL_PORTAL` | DSN do papel `wyd_portal` | nenhum | sim, para o portal/BFF | portal |
| `WYD_DB_URL_ADMIN` | DSN do papel `wyd_admin` (decisão `M27`) | nenhum | sim, quando o `wyd-admin` roda | serviço `wyd-admin` |
| `WYD_DB_URL_READ` | DSN do papel `wyd_read` | nenhum | sim, para leitura operacional | operação/BI |
| `WYD_DB_URL_TEST` | DSN do `wyd_migrator` em banco descartável | nenhum | sim, para a suíte de sistema | suíte de teste |

> [!WARNING] A DSN do `wyd_admin` ainda não está na tabela do contrato
> A decisão `M27` (2026-11-21) criou o papel `wyd_admin` com DSN própria, `WYD_DB_URL_ADMIN`,
> nomeada em [`seguranca.md`](../arquitetura/seguranca.md) e em `lacunas.md` `M27`. A tabela de
> `src/provision/spec.md` §3, de 2026-11-18, tem seis linhas e **ainda não inclui essa sétima
> variável**. Esta página lista a linha com a marca de decisão, mas não corrige o contrato: a
> correção é de `src/provision/spec.md` §3, na próxima revisão dele.

## 2. Variáveis de instalação e do runner

| Parâmetro | Efeito | Default | Obrigatório | Onde é lido |
|---|---|---|---|---|
| `WYD_MIGRATIONS_DIR` | diretório da série de migrations | `/migrations` | não | binário que monta o runner |
| `WYD_ENV_FILE` | caminho do arquivo de ambiente | não declarado | não | binário `wyd-db`; nunca `spec.rs` nem a biblioteca |
| `WYD_MIGRATE_ALLOW_SUPERUSER` | opt-out do preflight de papel, **só desenvolvimento**, sempre logado | ausente = desligado | não | binário, antes de abrir a conexão |
| `lock_timeout` | espera máxima pelo advisory lock da série | `10 s` (faixa `1 s`–`10 min`) | não | runner, campo de `MigrateConfig` |

`WYD_MIGRATIONS_DIR` é o **único com default** (`/migrations`, o caminho que as imagens copiam) e é
dado de instalação, não segredo. `WYD_ENV_FILE` também não é segredo; ele só é lido pelo binário.
`lock_timeout` não é variável de ambiente: é campo do `MigrateConfig`, com faixa validada — fora dela
o construtor devolve erro, não corrige em silêncio (`src/migrate/spec.md` §9).

Papel esperado (`wyd_migrator`) e versão do servidor (17) **não são configuração**: são constantes do
runner. Uma checagem de segurança que se desliga por arquivo de ambiente não é checagem; mudar
qualquer uma das duas é commit com `ADR-DB-0002` revista, não variável nova.

## 3. DSN e segredo por nome

| O que | Onde vive | Regra |
|---|---|---|
| nome e atributo do papel | `src/provision/spec.md` §2, versionado | vira `db/roles.sql` por `render` |
| senha do papel `LOGIN` | ambiente ou cofre | nunca no git, no SQL gerado ou em log |
| DSN (`WYD_DB_URL_*`, `WYD_PROVISION_DSN`) | ambiente ou cofre | **segredo**: a senha viaja dentro; `Debug` do `Dsn` é redigido |
| autenticação permitida | `pg_hba.conf` do ambiente | fora do escopo desta página |
| rotação | operação | `ALTER ROLE … PASSWORD` fora da série; `render` não a toca |

A DSN é secret **pelo valor**, não pelo nome: o nome da variável pode aparecer em documentação e em
mensagem de erro, o valor não. Nenhum caminho de `Display`/`Debug` do crate imprime a senha
(`src/dsn.md`; `src/provision/spec.md` §7). O `SESSION_SECRET` citado em `AGENTS.md` §2.5 **não** é
lido pelo `wyd-db` e não faz parte deste contrato.

## 4. O que muda sem reiniciar

**Nada do contrato recarrega a quente.** No runner, o binário monta o `MigrateConfig` no boot e a
partir daí só usa o valor recebido; trocar a DSN ou o diretório exige **novo processo**
(`src/migrate/spec.md` §9–§10.1). Para os serviços de jogo, portal e staff, **não há desenho de
recarga documentado**; reiniciar o processo é o caminho conservador até que exista um.

O que muda sem reiniciar é do **ambiente**, não do contrato: `pg_hba.conf` pode ser recarregado pelo
PostgreSQL, e essa operação é do operador — não é parâmetro do `wyd-db`. Papel e versão do servidor
não mudam por ambiente.

## 5. O que esta página não cobre

- **Valores e rotação:** são ato do operador; a página só diz onde o valor vive.
- **Pool, timeout de query e tamanho de conexão:** não há desenho documentado para jogo e portal.
- **A linha `WYD_DB_URL_ADMIN` no contrato:** está na decisão `M27`, não na tabela de §3; a
  divergência está declarada acima e não é corrigida por esta página.
- **Instalação e ordem:** são de [`plano-de-implantacao.md`](../planos/plano-de-implantacao.md).
- **Prova de conexão:** `WYD_DB_URL_TEST` e o roteiro de teste são de
  [`testes-de-sistema.md`](../guia-teste/testes-de-sistema.md).
