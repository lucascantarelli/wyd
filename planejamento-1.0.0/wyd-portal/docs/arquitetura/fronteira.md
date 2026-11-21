> Pergunta: "onde o portal termina e onde começam o `wyd-db` e o `wyd-admin`, e o que cada um pode exigir do outro?"
> Público: Desenvolvedor novo e arquitetura
> Dono: arquitetura
> Base: `M21` (`wyd-db/lacunas.md` §2.3); `wyd-db/ddl-confirmado.md` §1/§2; `AGENTS.md` §5/§6; `wyd-portal-prompt.md` §5/§6
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-20)

O portal é a única superfície onde pessoa e staff tocam o sistema fora do cliente 7662. Ele **não**
decide regra de jogo: lê estado e pede operação. Este texto fixa a fronteira, e o contrato de escrita
de estado de jogo foi decidido em 2026-11-20 (`LAC-POR-01`): **quem escreve em `game` é o
`wyd-admin`**, nunca o BFF do portal.

## 1. O que o portal faz

1. Autentica pessoa no portal (e-mail/senha, sessão revogável e tokens de cerimônia) e mantém a
   conta de jogo ligada à pessoa.
2. Serve o portal público: home, notícias, rankings, WYDpedia e atlas, com conteúdo do schema
   `content` e agregados do jogo.
3. Serve a área administrativa: NOC, jogadores e contas, economia, CMS, launcher, logs, segurança e as
   seis telas de monitoramento, além da configuração do servidor com trilha de auditoria (`M21`).
4. Expõe o BFF: rotas de servidor que validam sessão, permissão e entrada antes de tocar o banco.

## 2. O que o portal **não** faz

| Fora | Dono |
|---|---|
| tick, combate, drop, progressão, IA, física | `wyd-core` / `wyd-sim` |
| protocolo de fio CPSock, opcodes e offsets | `wyd-protocol` |
| sessão de jogo e topologia de canal | `wyd-server` (estado em `game.session`/`server_channel`) |
| DDL, migrations, adapters e pool | `wyd-db` |
| API interna de staff, jobs e ações privilegiadas | `wyd-admin` (ainda sem planejamento) |
| patch, manifesto e integridade de arquivo do cliente | `wyd-launcher` / `wyd-ops` |
| hospedagem, backup, monitoração de infraestrutura | `wyd-ops` |

## 3. Contratos (proposta)

| # | Contrato | Direção | Garantia | Erro esperado | Dono |
|---|---|---|---|---|---|
| C1 | SQL de leitura pelo adapter do `wyd-db` | portal → banco | leitura consistente; nenhuma escrita em tabela de jogo fora do BFF | `42501` (sem `USAGE`), `23503` | `wyd-db` |
| C2 | Escrita de estado de portal (`person`, `person_session`, `person_token`, `person_consent`) | portal → banco | transação por requisição; `updated_at` por trigger | `23505`, `23514` | portal |
| C3 | Escrita administrativa de configuração (`ops.config_revision`/`config_change`) | portal → banco | revisão + mudanças na mesma transação; autor em `actor_kind`/`actor_id` | `23514` (revisão inválida) | portal + `wyd-db` |
| C4 | Escrita em `game` e ação no processo (kick, broadcast, manutenção, ajuste de saldo) | portal → `wyd-admin` | HTTP/JSON interno com timeout, chave de idempotência e erro tipado | `4xx` de validação, `5xx` de execução; nenhum é traduzido em escrita local | arquitetura + `wyd-admin` |
| C5 | Conteúdo de catálogo (`content.*`) | portal → banco | leitura; escrita só via CMS, com revisão | — | portal + `wyd-db` |

**C4 foi decidido em 2026-11-20 (`LAC-POR-01`).** O `wyd-admin` é o **único executor** de escrita em
`game`; o BFF do portal chama a API interna e não recebe `USAGE` no schema. O contrato HTTP ainda não
tem forma publicada — o `wyd-admin` não tem planejamento —, então o portal depende dele para as ações,
mas não pode substituí-lo por escrita direta. `P-17` continua valendo: o portal lê `game` pela view de
`0029`, quando ela existir.

## 4. Consumo por superfície

| Superfície | Consome | Observação |
|---|---|---|
| Portal público | `content.*` e agregados de `game`/`portal` | cacheável; nada de escrita |
| Área administrativa | `game` (leitura por view), `portal`, `ops` e API de métricas do `wyd-ops` | escrita em `game` só pelo `wyd-admin`; telemetria é leitura externa, sem tabela no banco (`LAC-POR-06`); detalhe em `consumo-do-banco.md` |
| Monitoramento (rotas 38–43) | Prometheus, Loki e Grafana por HTTP, mais `game.v_live_session` | server-only: o navegador só recebe a URL pública do painel; nenhuma série no banco (`monitoramento.md`) |
| BFF | sessão de portal + permissão | toda rota de servidor valida antes de consultar |

## 5. Estado: o que persiste e o que é efêmero

| Persistente | Dono | Efêmero |
|---|---|---|
| `portal.person`, `person_profile`, `person_consent` | `0003` | — |
| `portal.person_session`, `person_token` | `0004` | cookie de sessão (o valor em claro) |
| `ops.config_revision`, `ops.config_change` | `0002` | rascunho de edição no navegador |
| cache de página e de consulta | portal | tudo (revalidado por `ISR`/`no-store`) |

## 6. O que quebra se o portal cair

| Afetado | Efeito |
|---|---|
| Jogador no cliente 7662 | **nada** muda: login e sessão de jogo não passam pelo portal (`0005`/`0007`) |
| Login e cadastro no site | indisponível enquanto durar a queda |
| Área administrativa | indisponível; configuração e moderação param |
| Loja de donate, recompensa diária e resgate | indisponível até voltar |
| Entrega de item comprado | depende da fila de `0012`, que ainda não tem DDL |

## 7. Fronteira de dados (não negociável)

- `wyd_portal` **não tem `USAGE`** no schema `game` (`P-17`): leitura de personagem e de sessão só
  pela view de `0029`, quando ela existir.
- `wyd_game` **não tem `USAGE`** em `portal`: o jogo não lê pessoa, e-mail ou consentimento.
- O portal **nunca** guarda segredo em claro: senha e PIN são Argon2id (`0003`/`0005`), token é hash
  de 64 hex (`0004`), e o valor em claro só existe no navegador do titular.

## 8. O que esta página não decide

| Assunto | Onde fecha |
|---|---|
| Forma do contrato HTTP do `wyd-admin` (rotas, payload, códigos) | planejamento do `wyd-admin`, ainda inexistente |
| Tabelas de CMS e de launcher no `wyd-db` | lote `0030` aprovado em 2026-11-20: 4 tabelas em `portal`; série 31 arquivos / 97 tabelas (`lacunas.md` §3) |
| Autenticação de staff com segundo fator | decidido em `LAC-POR-05` (TOTP em `portal.person_authenticator`), com DDL no lote `0031` (`ddl-confirmado.md` §5.20) |
| Telemetria de CPU/RAM/latência, logs e painéis | decidido em `LAC-POR-06` (reconfirmada em 2026-11-20): Prometheus, Loki e Grafana do `wyd-ops`; portal consulta a API — fonte e degradação em `monitoramento.md` |
| Forma do contrato de métricas (URL, credencial, intervalo, retenção) | `monitoramento.md` §4/§6 fixa nomes e fontes; o valor e a retenção definitiva são do `wyd-ops` |

*Página de planejamento. Nenhuma rota, componente ou consulta foi implementado.*
