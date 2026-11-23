# `wyd-net` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-22).** Este é o **único** lugar onde uma lacuna do `wyd-net` tem
> dono, impacto e estado. Nenhuma linha aqui está decidida: a abertura de 2026-11-22 nomeia as
> perguntas, não as responde.

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
| Conexão e limites | 2 | 2 | 0 | 0 |
| Sessão e identidade | 1 | 1 | 0 | 0 |
| Operação | 1 | 1 | 0 | 0 |
| **Total** | **4** | **4** | **0** | **0** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LNET-01` | Qual é o limite por conexão, o timeout ocioso e a política de rate-limit, e com que valor medido? | sem número, o limite vira opinião; abuso e conexão zumbi não têm defesa declarada | arquitetura + frente `wyd-ops` | aberta |
| `LNET-02` | A versão de protocolo é negociada na sessão ou imposta pelo build do cliente? | depende de `LPRT-09`, que segue aberta por falta de captura datada; afeta handshake e compatibilidade | frente `wyd-client` + arquitetura | aberta |
| `LNET-03` | Qual é o modelo de sessão: quantas conexões por conta, como se identifica a sessão e o que acontece na segunda conexão? | multibox e limite por IP já são regra de negócio em `rascunho-accounts.md` §3 do `wyd-db`; falta o dono do transporte | arquitetura + produto | aberta |
| `LNET-04` | O que o transporte expõe como métrica e log, e o que nunca entra em log? | `wyd-ops` opera por Prometheus/Loki; logar payload ou credencial viola `AGENTS.md` §5.7 | frente `wyd-ops` | aberta |

## 3. O que não é lacuna deste sistema

- Reação a erro de codec, `UnknownType` e ordem por conexão: **decididas** em `LPRT-13`, `LPRT-08` e
  `LPRT-12` (`planejamento-1.0.0/wyd-protocol/lacunas.md`).
- Layout do fio: dono é `wyd-protocol`; aqui só se consome.
