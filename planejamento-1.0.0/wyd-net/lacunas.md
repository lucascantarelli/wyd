# `wyd-net` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Este é o **único** lugar onde uma lacuna do `wyd-net` tem
> dono, impacto e estado. `LNET-02` fechou em 2026-11-23 pela decisão do dono (`Q-NET-2` = A) e
> `LNET-03` na mesma data pela regra já fixada fora daqui; `LNET-04` fechou na mesma data por
> **regra vigente** (`X10`, `LAC-POR-06` e `AGENTS.md` §5.7), sem escolha do usuário. `Q-NET-1` = A
> fixou a **política** de limites, mas `LNET-01` **segue aberta como medição**: não há conexão,
> latência nem recusa medidas. O briefing das respostas é `../briefing-de-decisoes-1.0.0.md` §2 e §3.

## 0. Como ler

| Estado | Significado |
|---|---|
| **aberta** | sem decisão; a página que depende dela não pode ser `aceita` |
| **proposta** | existe recomendação técnica escrita; falta o "sim" do dono |
| **fechada** | decidida e refletida no arquivo citado em "evidência" |

Dono: `arquitetura` propõe, o dono do sistema aprova. Fechar lacuna sem o dono é proibido
(`AGENTS.md` §5.8).

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Conexão e limites | 2 | 1 | 0 | 1 |
| Sessão e identidade | 1 | 0 | 0 | 1 |
| Operação | 1 | 0 | 0 | 1 |
| **Total** | **4** | **1** | **0** | **3** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LNET-01` | Qual é o limite por conexão, o timeout ocioso e a política de rate-limit, e com que valor medido? | sem número, o limite vira opinião; abuso e conexão zumbi não têm defesa declarada | arquitetura + frente `wyd-ops` | aberta (2026-11-23: decisão do dono, `Q-NET-1` opção **A** — **medir e congelar**: a forma do N6 fica fechada (balde por conexão, porte por IP, total por shard, timeout de handshake/ociosidade; recusa tipada, contador e fechamento), mas os valores só entram no gate após a primeira execução no CI efêmero, com data. O cenário de `Q-BOT-3` declara o perfil de teste para não medir o limite por IP. **A lacuna não fecha hoje**; 0 migration, 0 tabela, 0 coluna) |
| `LNET-02` | A versão de protocolo é negociada na sessão ou imposta pelo build do cliente? | `LPRT-09` fixou o build-alvo em 7662 + SHA-256 e esta lacuna decidiu a política: impor em vez de negociar evita tabela de compatibilidade sem prova e offset ambíguo | frente `wyd-client` + arquitetura | fechada (2026-11-23: decisão do dono, `Q-NET-2` opção **A** — impor o build-alvo 7662 no handshake, com erro tipado para build não suportado; registro em `docs/proposta-decisoes-net.md` §4 e §6; implementação pendente no transporte) |
| `LNET-03` | Qual é o modelo de sessão: quantas conexões por conta, como se identifica a sessão e o que acontece na segunda conexão? | a regra já está fixada fora daqui — uma sessão viva por conta e a segunda recusada; resta ao transporte implementá-la, não escolher o modelo | arquitetura + produto | fechada (2026-11-23: `R6`/`D9` em `wyd-db/rascunho-accounts.md` §2/§6 e o índice `ux_session_viva_por_conta`; implementação pendente no login, critério C3 do esqueleto do `wyd-server`) |
| `LNET-04` | O que o transporte expõe como métrica e log, e o que nunca entra em log? | `wyd-ops` opera por Prometheus/Loki; logar payload ou credencial viola `AGENTS.md` §5.7 | frente `wyd-ops` | fechada (2026-11-23: **derivada das regras vigentes, sem escolha do usuário** — `X10` fixa `/metrics` Prometheus coletado pelo `wyd-ops`, `LAC-POR-06` fixa a fonte externa e `AGENTS.md` §5.7 proíbe payload/PII em log; o transporte expõe conexões, erros por tipo, bytes e latência e registra só metadados; nomes de métrica são implementação; registro em `docs/proposta-decisoes-net.md` §4 e §6) |

## 3. O que não é lacuna deste sistema

- Reação a erro de codec, `UnknownType` e ordem por conexão: **decididas** em `LPRT-13`, `LPRT-08` e
  `LPRT-12` (`planejamento-1.0.0/wyd-protocol/lacunas.md`).
- Layout do fio: dono é `wyd-protocol`; aqui só se consome.
