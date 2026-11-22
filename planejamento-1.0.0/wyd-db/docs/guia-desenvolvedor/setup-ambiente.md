> Pergunta: "o que preciso ter na máquina para desenvolver o `wyd-db` e o que ainda não foi decidido?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §3; `src/migrate/spec.md` §9; `plano-migrations.md` §7; `guia-administrador/configuracao.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O ambiente tem duas partes: ferramentas que ainda não têm versão aprovada e **variáveis por papel**,
que já têm nome e dono. Esta página não inventa versão de Rust, imagem de container, porta local ou
comando de instalação; ela diz o que existe e o que falta para o setup ser executável.

## 1. Ferramentas

| Ferramenta | Necessidade | Estado |
|---|---|---|
| Rust + Cargo | compilar o crate e rodar os alvos | versão mínima não decidida em documento |
| PostgreSQL 17 | alvo de sistema e ciclo de migrations | versão decidida; instalação local não desenhada |
| `sqlx` | acesso ao banco, confinado ao adapter | decisão de dependência do crate; versão não fixada aqui |
| cliente 7662 | corte vertical e captura de protocolo | fonte local, fora do setup do crate |

Não há `docker-compose.yml`, `Makefile` ou script de bootstrap aprovado para o `wyd-db`; a página não
cria um. O banco descartável pode ser fornecido pelo ambiente, mas a imagem, a porta e o volume são
decisão de implantação, não deste guia.

## 2. Variáveis de ambiente

Nenhuma tem default; ausência deve falhar **nomeando a variável**. A fonte é
`src/provision/spec.md` §3; esta tabela só repete o nome e o leitor, não o valor.

| Variável | Papel / uso | Quem lê |
|---|---|---|
| `WYD_PROVISION_DSN` | admin do cluster e dono do banco, só no provisionamento | binário `wyd-db` |
| `WYD_DB_URL_MIGRATOR` | `wyd_migrator`, DDL e ledger | runner de migrations |
| `WYD_DB_URL_GAME` | `wyd_game`, estado quente | servidor de jogo |
| `WYD_DB_URL_PORTAL` | `wyd_portal`, identidade e PII | portal/BFF |
| `WYD_DB_URL_READ` | `wyd_read`, leitura de operação | BI/operação |
| `WYD_DB_URL_TEST` | `wyd_migrator` em banco descartável | suíte de sistema |
| `WYD_MIGRATIONS_DIR` | diretório da série; único com default `/migrations` | binário e runner |
| `WYD_MIGRATE_ALLOW_SUPERUSER` | opt-out do preflight, só desenvolvimento e logado | binário |

**Não existe `DATABASE_URL` na `1.0.0`.** Um nome único para todos os papéis permitiria um superusuário
em runtime; o preflight recusa conexão que não seja `wyd_migrator`. O arquivo de ambiente é lido pelo
binário, nunca pela biblioteca; não há valor de DSN nesta página.

## 3. Banco local

O mínimo é um PostgreSQL 17 descartável com quatro schemas e os papéis provisionados. A ordem é a de
[`../planos/plano-de-implantacao.md`](../planos/plano-de-implantacao.md): provisionar papéis, aplicar a
série, semear e só então apontar o serviço. O banco de desenvolvimento pode usar a DSN administrativa
do container; isso satisfaz `CREATEROLE` e posse do banco, mas **não** é o papel de runtime.

## 4. Como rodar a suíte

Os comandos ainda não existem porque o crate e os alvos não existem. O desenho é:

```text
cargo test -p wyd-db                    # alvos unitários e de contrato
WYD_DB_URL_TEST=... cargo test -p wyd-db --test sistema_pg
```

A segunda linha exige um banco descartável e falha nomeando `WYD_DB_URL_TEST` quando ela falta; não
há `SKIP` por ambiente ausente. O comando único da suíte é `P` e está registrado em
[`../planos/plano-de-testes.md`](../planos/plano-de-testes.md) §2.

## 5. Erros comuns de setup

| Sintoma | Causa provável | Ação |
|---|---|---|
| erro nomeando `WYD_DB_URL_MIGRATOR` | variável ausente | definir a DSN do papel de migração; não usar `DATABASE_URL` |
| `permission denied` ao criar schema | conexão com papel de runtime | usar a DSN de provisionamento/admin para o setup |
| migration falha no preflight de servidor | PostgreSQL fora da versão 17 | usar o PG17 do contrato; não desligar a checagem |
| teste de sistema não roda | `WYD_DB_URL_TEST` ausente | prover banco descartável; não marcar `#[ignore]` |
| `sqlx` não compila offline | cache de consulta não preparado | seguir o setup do adapter quando ele existir; não há receita aprovada hoje |

## 6. O que falta para esta página fechar

- versão mínima de Rust e política de toolchain;
- imagem, porta e volume do PostgreSQL local;
- comando de bootstrap e cache de `sqlx`;
- comando único da suíte com saída esperada.
