# `wyd-net` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Este é o **único** lugar onde uma lacuna do `wyd-net` tem
> dono, impacto e estado. `LNET-02` fechou em 2026-11-23 pela decisão do dono (`Q-NET-2` = A) e
> `LNET-03` na mesma data pela regra já fixada fora daqui; `LNET-04` fechou na mesma data por
> **regra vigente** (`X10`, `LAC-POR-06` e `AGENTS.md` §5, item 7), sem escolha do usuário. `Q-NET-1` = A
> fixou a **política** de limites em 2026-11-23, formalizada em `ADR-NET-0001` (aceito em 2026-11-25);
> `LNET-01` continua **bloqueada como medição** do valor final — não há conexão, latência nem recusa
> medidas —; a **autorização de execução foi dada em 2026-11-25** (opção A do bloco), registrada na
> [`pauta de decisões`](../wyd-ops/docs/pauta-de-decisoes-pendentes.md) (dono: QA + `wyd-ops`); falta executar. O briefing das
> respostas é `../briefing-de-decisoes-1.0.0.md` §2 e §3.

## 0. Como ler

| Estado | Significado |
|---|---|
| **fechada** | decidida ou produzida, com a evidência citada na coluna própria |
| **bloqueada** | falta decisão, execução ou evidência; a linha nomeia o desbloqueio (dono e documento) |

Dono: `arquitetura` propõe, o dono do sistema aprova. Fechar lacuna sem o dono é proibido
(`AGENTS.md` §5, item 8).

## 1. Situação em uma linha

| Grupo | Lacunas | Bloqueadas | Fechadas |
|---|---:|---:|---:|
| Conexão e limites | 2 | 1 | 1 |
| Sessão e identidade | 1 | 0 | 1 |
| Operação | 1 | 0 | 1 |
| **Total** | **4** | **1** | **3** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LNET-01` | Qual é o limite por conexão, o timeout ocioso e a política de rate-limit, e com que valor medido? | sem número, o limite vira opinião; abuso e conexão zumbi não têm defesa declarada | arquitetura + frente `wyd-ops` | **bloqueada como medição** (2026-11-25: política `Q-NET-1` = A formalizada em `ADR-NET-0001` e autorização de execução dada; a execução de prontidão encontrou **0** `Cargo.toml` em `crates/wyd-net`, **0** listeners e **0** cenário de carga, então não há valor para congelar; tarefa em `../wyd-ops/docs/planos/medicoes-lnet-01-lbot-03.md` §4.6; pergunta consolidada em `../wyd-ops/docs/lacunas.md` §4, item 7) |
| `LNET-02` | A versão de protocolo é negociada na sessão ou imposta pelo build do cliente? | `LPRT-09` fixou o build-alvo em 7662 + SHA-256 e esta lacuna decidiu a política: impor em vez de negociar evita tabela de compatibilidade sem prova e offset ambíguo | frente `wyd-client` + arquitetura | fechada (2026-11-23: decisão do dono, `Q-NET-2` opção **A** — impor o build-alvo 7662 no handshake, com erro tipado para build não suportado; registro em `docs/proposta-decisoes-net.md` §4 e §6; implementação pendente no transporte) |
| `LNET-03` | Qual é o modelo de sessão: quantas conexões por conta, como se identifica a sessão e o que acontece na segunda conexão? | a regra já está fixada fora daqui — uma sessão viva por conta e a segunda recusada; resta ao transporte implementá-la, não escolher o modelo | arquitetura + produto | fechada (2026-11-23: `R6`/`D9` em `wyd-db/rascunho-accounts.md` §2/§6 e o índice `ux_session_viva_por_conta`; implementação pendente no login, critério C3 do esqueleto do `wyd-server`) |
| `LNET-04` | O que o transporte expõe como métrica e log, e o que nunca entra em log? | `wyd-ops` opera por Prometheus/Loki; logar payload ou credencial viola `AGENTS.md` §5, item 7 | frente `wyd-ops` | fechada (2026-11-23: **derivada das regras vigentes, sem escolha do usuário** — `X10` fixa `/metrics` Prometheus coletado pelo `wyd-ops`, `LAC-POR-06` fixa a fonte externa e `AGENTS.md` §5, item 7 proíbe payload/PII em log; o transporte expõe conexões, erros por tipo, bytes e latência e registra só metadados; nomes de métrica são implementação; registro em `docs/proposta-decisoes-net.md` §4 e §6) |

## 3. O que não é lacuna deste sistema

- Reação a erro de codec, `UnknownType` e ordem por conexão: **decididas** em `LPRT-13`, `LPRT-08` e
  `LPRT-12` (`planejamento-1.0.0/wyd-protocol/lacunas.md`).
- Layout do fio: dono é `wyd-protocol`; aqui só se consome.
---

## 4. Bloco de decisão da lacuna bloqueada

A linha `LNET-01` em §2 não muda de estado até o artefato de fechamento existir; o que segue é a
proposta da arquitetura, **aceita pelo dono em 2026-11-25 na opção A** (`AGENTS.md` §5, item 8).

**LNET-01 — limites por conexão.**
- **Dono da decisão:** arquitetura + frente `wyd-ops`, com o dono do sistema aprovando o congelamento;
  o método está em `planejamento-1.0.0/wyd-ops/docs/planos/medicoes-lnet-01-lbot-03.md` §4.
- **Pergunta fechada:** com que valor medido ficam o balde por conexão, o teto por IP, o teto por
  shard, o timeout de handshake e o timeout ocioso, e qual recusa o teste aceita?
- **Opções:** (A) **valores absolutos congelados após a escada de carga** — tetos por conexão/IP/shard e
  timeouts fixos, com recusa contada por motivo; custo: re-medir quando o shard mudar; (B) **política
  adaptativa por taxa de recusa** — o limite abre e fecha conforme a recusa observada; custo: o
  comportamento deixa de ser previsível para o jogador e para o teste.
- **Recomendação:** A, porque `ADR-NET-0001` já fixou política estrita e o que falta é o número que o
  teste de recusa consegue provar.
- **Aceite do dono (2026-11-25):** opção **A** aprovada — valores absolutos congelados após a escada de carga, com recusa contada por motivo; a execução está autorizada e depende do ambiente de `OPS-03.1`.
- **Artefato que falta (o que fecha):** execução datada preenchendo `medicoes-lnet-01-lbot-03.md` §4.6, tetos aplicados no
  esqueleto do transporte e teste de recusa por motivo; a linha `LNET-01` passa a citar a execução.
