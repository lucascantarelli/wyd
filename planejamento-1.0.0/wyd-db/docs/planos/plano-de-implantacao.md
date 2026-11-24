> Pergunta: "como isto entra no ar?"
> Público: Operação e dono do produto
> Dono: equipe do wyd-db
> Base: `plano-migrations.md`, `src/provision/spec.md`, `docs/arquitetura/adr/adr-db-0001-provisionamento-de-papel-fora-da-serie.md`
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18)

Implantar a `1.0.0` é provisionar papel uma vez, aplicar a série uma vez e subir os serviços cada
um com a sua credencial. A ordem importa: cada etapa depende do objeto que a anterior criou, e
nenhuma delas é idempotente por acidente.

## Pré-requisitos

| Item | Exigência |
|---|---|
| PostgreSQL | 17, com um administrador de cluster disponível **só** para o provisionamento |
| binário | `wyd-db` (subcomandos de `provision` e o runner) na máquina que executa |
| série | `WYD_MIGRATIONS_DIR` apontando para os 34 arquivos, íntegros e na ordem de `plano-migrations.md` §3 |
| ambiente | arquivo com as seis DSNs preenchidas (nomes em `src/provision/spec.md` §3; valores fora do git) |
| banco | vazio, ou com backup verificado se houver dado a preservar |
| rede | porta do banco fora do loopback **só** com justificativa registrada; desenvolvimento é loopback |

`provision apply` usa `WYD_PROVISION_DSN` (admin do cluster **e** dono do banco — o bloco de `public`
em `src/provision/spec.md` §4 exige a posse); as migrations, `WYD_DB_URL_MIGRATOR`;
e cada serviço sobe com a DSN do seu papel — `WYD_DB_URL_GAME`, `WYD_DB_URL_PORTAL`,
`WYD_DB_URL_READ`. Nomes, leitor e o que acontece sem a variável estão em `src/provision/spec.md` §3;
aqui fica só qual etapa usa qual.

## Ordem

| # | Etapa | O que acontece | Como falha |
|---|---|---|---|
| 1 | provisionar papéis e `public` | `wyd-db provision apply` cria os seis papéis e aplica o bloco de `public` (dono: `src/provision/spec.md` §4); fora da série (`ADR-DB-0001`) | roda uma vez por **banco** — criar papel é no-op no cluster; reexecutar é no-op verificado |
| 2 | aplicar a série | o runner valida o papel, pega o advisory lock e aplica `0001`…`0034` em transação por arquivo — é `0001` que cria os quatro schemas | aborta no primeiro erro; o ledger diz até onde chegou |
| 3 | conteúdo | os seeds da própria série gravam o catálogo e fecham `content_version` | contagem divergente de `plano-migrations.md` §4 reprova o gate de conteúdo |
| 4 | serviços | jogo (`WYD_DB_URL_GAME`) e portal (`WYD_DB_URL_PORTAL`) sobem e rodam o preflight de conexão | preflight recusa papel errado em vez de degradar |
| 5 | verificação | §"Verificação pós-implantação" | falha aqui é rollback, não "ajustar depois" |

> [!WARNING] Dado legado é decisão aberta
> Se existir base antiga a preservar, esta ordem **não** se aplica como está: entra uma etapa de
> carga entre 2 e 3, com ETL e janela próprios. A `1.0.0` não decidiu isso ainda — está registrado
> como pendência, e assumir "banco novo" sem essa decisão é o erro mais caro desta página.

## Janela

A janela é a da etapa 2: enquanto a série aplica, os serviços não podem escrever. Para instalação
nova, o custo é o tempo de `0001`…`0029` num banco vazio; para base existente, some a carga. Anuncie
a janela com o número medido no ensaio, não com estimativa.

## Verificação pós-implantação

| O que conferir | Esperado |
|---|---|
| tabelas por schema (`content`, `game`, `portal`, `ops`) | soma **109** (`plano-migrations.md` §3), com fonte e data na página da série |
| views | 2 (`plano-migrations.md` §3) |
| versão de conteúdo | `content_version = 7` (`plano-migrations.md` §4) |
| papéis | os seis existem; `wyd_owner` é `NOLOGIN` (`seguranca.md`) |
| ledger | uma linha por arquivo aplicado, sem drift de checksum (`src/migrate/spec.md` §3–§4) |
| fronteira | `portal` recebe `permission denied` ao escrever em `game` (`padrao-banco.md` §2) |
| seis DSNs | nenhuma ausente, nenhuma com default (`src/provision/spec.md` §3) |

## Rollback

- **Schema:** o `down` de cada arquivo existe e é usado em desenvolvimento; em produção ele **não**
  é o plano A.
- **Dado:** não há rollback de dado. Banco anterior preservado (ou backup verificado) é o único
  caminho de volta — e ele perde o que foi escrito depois.
- **Proibido:** `docker compose down -v`, `DROP`/`TRUNCATE` e reset de volume sem pedido explícito;
  são perda de dado, não rollback — a regra é de repositório (`AGENTS.md` §7), não desta página.

## Comunicação

Antes: o que muda, quando, quem é afetado e o que ele deve fazer. Durante: um canal único e um
responsável por decidir abortar. Depois: o resultado da verificação com os números medidos, e o
registro em `planos/registro-de-execucao.md`. Se a janela falhar, a mensagem diz **estado atual**
(banco novo? série parcial? serviço antigo?) — nunca "estamos verificando".
