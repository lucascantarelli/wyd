> Pergunta: "o que os tipos do `wyd-db` garantem antes de o SQL ser executado?"
> Público: Desenvolvedor do crate e revisor de contrato
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2–§4; `src/dsn.md`; `padrao-banco.md` §4; `proposta-paginas.md` §5
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Os tipos de identidade e de migration existem para que o valor inválido não chegue ao banco. Esta
página cobre os tipos com invariante publicado: `GameLogin`, `GamePassword`, `NumericPin`, `Dsn`,
`Version`, `Slug`, `Sql` e `Checksum`. Os modelos de personagem, item e economia ainda não têm
contrato de tipo; não são inventados aqui.

## 1. Tipos de identidade

| Tipo | Forma publicada | Invariante | Quem reforça |
|---|---|---|---|
| `GameLogin` | 16 bytes | login do cliente 7662 no formato do protocolo; não é e-mail | construtor do tipo; `game.account.login_fold` para busca |
| `GamePassword` | 12 bytes | senha do jogo no formato do cliente; o banco guarda hash, não o valor | construtor; `password_hash` no DDL |
| `NumericPin` | 6 bytes | PIN numérico de 6 posições; o banco guarda hash | construtor; `pin_hash` no DDL |

As três formas vêm do contrato do cliente e de `padrao-banco.md` §4. O tipo não decide hash, salt ou
algoritmo; isso é da camada que grava. O `Debug` não pode imprimir o valor — a regra de segredo é a
mesma de `Dsn`.

## 2. Tipos do runner

| Tipo | Invariante | Onde é provado |
|---|---|---|
| `Dsn` | parse único do crate; `Debug` redige a senha; não existe `DATABASE_URL` | página `src/dsn.md`; teste de redação |
| `Version` | `NonZeroU32`; zero não é versão, `DownTo::Empty` é o reset | teste de construção e `down --to 0` |
| `Slug` | `snake_case`, sem acento, coerente com o nome do arquivo | parser da série; `SetInvalid` |
| `Sql` | não vazio, sem `-- migrate:up/down` interno, sem `BEGIN`/`COMMIT`/`ROLLBACK` | construtor de `Sql`; teste de marcador e controle de transação |
| `Checksum` | `sha256` dos bytes normalizados; `CRLF` vira `LF` | `checksum.rs`; teste de um byte de diferença |
| `MigrateConfig` | DSN, diretório e `lock_timeout`; timeout fora de 1 s..=10 min é erro | construtor com campos privados; `with_lock_timeout` |

`Version`, `Slug`, `Sql` e `Checksum` são tipos puros: não leem disco, ambiente ou relógio. O
`MigrationSet` é quem ordena e recusa buraco/duplicata; o `plan` só compara disco e ledger.

## 3. Constantes e origem

| Constante | Valor | Origem |
|---|---:|---|
| tamanho de `GameLogin` | 16 B | contrato do cliente 7662; `padrao-banco.md` §4 |
| tamanho de `GamePassword` | 12 B | contrato do cliente 7662; `padrao-banco.md` §4 |
| tamanho de `NumericPin` | 6 B | contrato do cliente 7662; `padrao-banco.md` §4 |
| `MIGRATION_LOCK_KEY` | `0x5759_4404` | `src/migrate/spec.md` §5; chave estável do lock |
| `SERVER_MAJOR` | 17 | `src/migrate/spec.md` §9; não é configurável |
| `lock_timeout` padrão | 10 s | `src/migrate/spec.md` §5; faixa 1 s..=10 min |
| `SCHEMA_VERSION` | 34 | `plano-migrations.md` §2; valor da troca, não da série herdada |

Nenhum desses números é uma decisão desta página: cada linha cita o dono. Um valor novo exige
alteração na fonte e, quando for contrato, revisão de ADR.

## 4. O que o banco reforça

| Invariante de tipo | Reforço no banco |
|---|---|
| login único por `login_fold` | índice único e `CHECK` de formato quando o DDL fechar |
| PIN e senha nunca em claro | coluna de hash; o tipo não é persistido |
| versão de migration positiva | `version` inteira no ledger; zero não é inserível pelo runner |
| conteúdo de uma versão | FK/versão em `content.*`; mistura é recusada no boot |
| lock único | chave constante de advisory lock; não é uma tabela |

O que o banco **não** reforça: a semântica de um campo `H` (hipótese) e a regra de jogo. Essas ficam
marcadas na página do domínio e não viram `CHECK` por conveniência.

## 5. O que esta página não cobre

- **Modelos de personagem, item, clã e economia:** não têm contrato de tipo publicado.
- **Hash, salt e rotação:** são da camada de identidade e do ambiente; não há algoritmo novo aqui.
- **Formato completo do protocolo:** depende da captura do cliente 7662 (`LPRT-*`), ainda aberta.
