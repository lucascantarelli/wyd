> Pergunta: "o que o `wyd-admin` faz, o que ele não faz e o que depende dele?"
> Público: Arquiteto, desenvolvedor novo e operação
> Dono: arquitetura
> Base: `wyd-portal/docs/arquitetura/fronteira.md` §3/§4; `LAC-POR-01` (2026-11-20); `AGENTS.md` §6
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21).

## 1. O que este sistema faz

1. Expõe uma **API HTTP/JSON interna**, server-to-server, para o BFF do portal executar ações de staff.
2. É o **único componente com credencial de escrita em `game`** (`LAC-POR-01`): moderação de conta,
   carteira, canal, preço, imposto e **custódia de item do mercado** (desde 2026-11-22, C4 §4.9–§4.12).
3. Executa **uma ação por requisição**, em transação única, com `ops.audit_log` na mesma transação.
4. Garante **idempotência** por chave enviada pelo chamador: repetir a chamada devolve o mesmo
   resultado, não a segunda escrita.
5. Encaminha as ações de **processo** (kick, broadcast) ao `wyd-server`, quando esse contrato existir —
   proposta em 2026-11-22 e aguardando aprovação ([autenticação e controle](autenticacao-e-controle.md) §3).

## 2. O que ele não faz

| Fora | Dono |
|---|---|
| Tela, HTML, sessão de navegador e segundo fator do staff | `wyd-portal` |
| Decidir regra de jogo (dano, drop, progressão, economia) | `wyd-sim` / `wyd-core` |
| Tick, combate, IA e estado de mundo em memória | `wyd-core` / `wyd-server` |
| DDL, migrations, papel e `GRANT` | `wyd-db` |
| Autenticar pessoa, emitir sessão e guardar consentimento | `wyd-portal` (`0003`/`0004`) |
| Configuração versionada (`ops.config_revision`/`config_change`) | `wyd-portal` (contrato C3) |
| Conteúdo editorial (`portal.news` e afins), CMS | `wyd-portal` |
| Métricas, logs, backup e infraestrutura | `wyd-ops` |

O `wyd-admin` **não** é um segundo BFF e **não** atende navegador: quem chama é o servidor do portal.

## 3. Quem consome

| Consumidor | O que consome | Garantia exigida |
|---|---|---|
| BFF do `wyd-portal` | as rotas de ação de staff e as rotas de custódia de item do [contrato C4](contrato-c4.md) | resposta tipada em até 5 s; repetição segura por `Idempotency-Key`; nas rotas de custódia o ator é a conta do jogador, não o staff |
| `wyd-server` (futuro) | recebe kick/broadcast/manutenção | `LAC-ADM-03`: proposta em 2026-11-22 (HTTP interno síncrono com ack), aguardando o dono |
| `wyd-ops` | `GET /health` e `GET /ready` | 200/503 sem detalhe interno |

## 4. O que ele consome

| Fornecedor | Contrato | Uso |
|---|---|---|
| `wyd-db` | SQL transacional sob papel próprio | escrita em `game`, `ops.audit_log` e leitura de conferência |
| `wyd-server` | canal de controle (`LAC-ADM-03`, proposta de 2026-11-22) | kick, broadcast e manutenção de processo |
| `wyd-portal` | identidade do staff já autenticado (`X-Staff-Person-Id`) ou a conta do jogador (`X-Actor-Account-Id`), no cabeçalho | autoria da ação |

O portal é a autoridade de **quem** é o staff; o admin é a autoridade de **o que** pode ser escrito.

## 5. Estado: o que persiste e o que é efêmero

| Persistente | Onde | Dono |
|---|---|---|
| Efeito da ação (bloqueio, lançamento, capacidade, preço, custódia de item) | tabelas de `game` (`market_escrow` no lote `0034`, `ddl-confirmado.md` §5.34) | `wyd-db` |
| Trilha da ação | `ops.audit_log` | `wyd-db` |
| Registro de idempotência | `ops.idempotency_record` (`0033`, `ddl-confirmado.md` §5.33), decidido em 2026-11-22 (`M30`) | `wyd-db` |

Efêmero: pool de conexões, cache de configuração lida, `trace_id` e o corpo da requisição em memória.
O admin **não** mantém fila durável própria na `1.0.0`; ação que não concluiu não é “depois”.

## 6. O que quebra se ele cair

| Afetado | Efeito |
|---|---|
| Portal público e leitura da área administrativa | **nada**: o BFF lê `game` por view e `portal` direto |
| Ações de staff (bloquear, ajustar saldo, kick, preço, canal) | indisponíveis; a tela mostra erro tipado, não sucesso |
| Jogador no cliente 7662 | **nada** muda: login e sessão de jogo não passam pelo admin |
| Configuração (`ops.config_*`) | continua funcionando, porque é escrita do portal (C3) |
| Auditoria | sem ação, sem linha: não existe auditoria “preparada” fora da transação |

## 7. Fronteira de dados (não negociável)

- Escrita em `game` só pelo admin; o portal não tem `USAGE` em `game` (`P-17`).
- O admin **não** escreve em `portal`: moderação de pessoa, sessão de portal e CMS são do BFF.
- Toda escrita carrega autor (`person_id` do staff) e `trace_id`; `ops.audit_log` é gravado na mesma
  transação da mudança.
- Nenhum payload, log ou resposta contém `password_hash`, `pin_hash`, `token_hash`, segredo TOTP ou
  PII além do identificador da conta/personagem.

## 8. O que esta página não decide

| Assunto | Onde fecha |
|---|---|
| Rotas, payload, códigos e prazos | [contrato C4](contrato-c4.md) (proposta de 2026-11-21) |
| Papel e `GRANT` de escrita | `LAC-ADM-01` — `wyd-db` |
| Armazenamento e retenção da idempotência | **fechados em 2026-11-22** (`M30`): tabela em `ops`, 7 dias após `created_at`; ver `lacunas.md` |
| Canal com o `wyd-server` | `LAC-ADM-03` — **proposta em 2026-11-22**, aguardando o dono ([autenticação e controle](autenticacao-e-controle.md) §3) |
| Autenticação portal → admin | `LAC-ADM-04` — **proposta em 2026-11-22**, aguardando o dono (§2 da mesma página) |
| Emissão e rotação do segredo de cada elo | `wyd-ops`; a página de proposta só nomeia as variáveis |
| Dono da escrita de `account_moderation` | **decidido em 2026-11-21** (`LAC-ADM-05`): é do `wyd-admin` (contrato C4 §4.1); o doc do portal foi corrigido e o portal só lê |
