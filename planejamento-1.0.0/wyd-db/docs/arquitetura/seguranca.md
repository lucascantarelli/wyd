> Pergunta: "quem pode o quê e o que é auditado?"
> Público: Arquiteto de schema e operação
> Dono: equipe do wyd-db
> Base: `padrao-banco.md` §2, §8, `ddl-confirmado.md`, `docs/arquitetura/adr/adr-db-0001-provisionamento-de-papel-fora-da-serie.md`
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18)

A segurança do `wyd-db` é feita de três camadas que não se substituem: **privilégio** (quem pode
tocar no quê), **hash** (o que não é reversível) e **auditoria** (o que fica registrado mesmo quando
o resto dá errado). Papel mal escolhido anula as outras duas.

## Papéis do banco

| Papel | Para que existe | Pode | Não pode |
|---|---|---|---|
| `wyd_owner` | dono dos schemas e dos objetos | nada em runtime: é `NOLOGIN` | conectar |
| `wyd_migrator` | aplicar a série | DDL no schema, escrita no ledger | servir requisição de jogo ou portal |
| `wyd_game` | servidor de jogo | escrever o estado vivo do jogo | ler dado de pessoa do portal, fazer DDL |
| `wyd_portal` | BFF do portal | ler identidade/sessão e escrever em `ops` | escrever estado vivo; conceder item direto (só via `delivery_queue`) |
| `wyd_read` | operação e BI | `SELECT` nas views | escrever qualquer coisa |

Os papéis nascem **fora da série**, no provisionamento (`ADR-DB-0001`), porque criar papel exige
privilégio de cluster e a série roda como `wyd_migrator`. O runner recusa conexão que não seja
`wyd_migrator`; a única saída é `WYD_MIGRATE_ALLOW_SUPERUSER=1`, restrita a desenvolvimento e
sempre logada.

As DSNs são **segredo** — a senha viaja dentro da string, e nunca são logadas em claro. Nome, leitor de
cada DSN e a classificação segredo × dado de instalação estão em `src/provision/spec.md` §3; a regra de
credencial, em §7.

**Fora do contrato:** outros segredos do repositório — o `SESSION_SECRET` citado em `AGENTS.md` §2.5 é
um deles — **não** são lidos pelo `wyd-db` e não entram no contrato de ambiente
(`src/provision/spec.md` §3 e §7). Com cookie opaco e só o `sha256` no banco (`A16`), a 1.0.0 pode não
precisar de segredo de assinatura nenhum; se algum sobreviver, é do portal, e esta página não é o dono
do nome.

## O que é hash e com que algoritmo

| Dado | Forma | Regra |
|---|---|---|
| senha do portal (`portal.person`) | `argon2id`, com sal por linha | nunca reversível, nunca comparada em claro |
| PIN de conta de jogo | `argon2id`, com sal por linha | PIN curto não muda o algoritmo: o custo vem do parâmetro, não do tamanho |
| token de sessão do portal | hash do token | o valor em claro existe só no cookie do cliente |
| idempotência de crédito | chave UUID única gerada pelo BFF, janela de 24 h | impede crédito duplicado por reenvio (`M17`, `0011`/`0012`) |

Parâmetro de custo é decisão de instalação, documentada; baixar o custo para "ficar rápido" é
regressão de segurança, não ajuste de performance.

## Política de credencial (decidida em 2026-11-19)

- **PIN de conta de jogo:** 4–6 dígitos, 5 falhas consecutivas bloqueiam por 10 minutos, desbloqueio
  só por tempo; `game.account.pin_failure_count` e `pin_locked_until` são os contadores.
- **Login do portal:** o mesmo limiar — 5 falhas/10 min, desbloqueio por tempo, sem intervenção de
  staff; os contadores de `portal.person` passam a ser movidos no login.
- **Senha do portal:** 15–128 caracteres, sem regra de composição, sem expiração periódica, com
  blocklist e `argon2id` (`M9`).

## O que nunca é gravado

- Senha, PIN ou token em claro — nem em coluna "temporária", nem em log de erro.
- DSN, chave de assinatura de cookie ou token em log, em `Debug`/`Display` de struct, em artefato gerado ou no git.
- IP público do jogador: a coluna de IP existe para operação e limite de conexão, não para perfil.
- Destinatário de reset de PIN dentro de `game` (`M2`): o dado de contato fica no portal.
- IP privado em seed (`192.168.x.x`): dado de instalação não entra na série (`AGENTS` §7).

## O que é auditado e onde

| Evento | Onde fica | Quem escreve |
|---|---|---|
| ação administrativa (ban, grant, ajuste) | `ops.audit_log`, append-only | serviço que executou, com ator derivado da sessão |
| pedido de concessão vindo da web | `ops.delivery_queue`, com autor na linha | portal |
| mudança de configuração | `ops.config_revision` / `config_change` | quem editou, com valor anterior |
| sessão encerrada | `game.session.released_at` | reaper do jogo |

Auditoria não tem `CASCADE` e não é apagada por exclusão de conta: o registro sobrevive ao objeto
que ele descreve (`padrao-banco.md` §8; `M18-A`, com o DDL em `ddl-confirmado.md` §5.3).

## Ameaça considerada × controle

| Ameaça | Controle |
|---|---|
| superusuário em runtime "resolve" um bug | preflight de papel no runner + DSN por papel: runtime não tem a credencial que faz DDL |
| portal escreve estado vivo direto | fronteira testada (`permission denied`) e `delivery_queue` como ponte única (`padrao-banco.md` §2 e §8) |
| reenvio de grant dobra item ou crédito | chave de idempotência com unique (`M17`) |
| dump de `game` vaza e-mail | PII fora de `game`; teste que varre o dump (`M2`) |
| segredo em log de erro de conexão | DSN descrita por nome de variável, nunca impressa |
| conta fechada com personagem vivo | `RESTRICT` na FK; fechamento é transição explícita, não `DELETE` (`P-15`, `P-20`; `padrao-banco.md` §5, item 1) |

Parâmetros de configuração, defaults e onde cada um é lido ficam em
`guia-administrador/configuracao.md`; esta página só diz o que é segredo e por quê.
