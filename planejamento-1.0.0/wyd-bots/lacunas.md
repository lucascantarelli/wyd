# `wyd-bots` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-22).** Único lugar onde uma lacuna do `wyd-bots` tem dono,
> impacto e estado. Nada aqui está decidido.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Ambiente | 1 | 1 | 0 | 0 |
| Dado | 1 | 1 | 0 | 0 |
| Medição | 1 | 1 | 0 | 0 |
| Relatório | 1 | 1 | 0 | 0 |
| **Total** | **4** | **4** | **0** | **0** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LBOT-01` | Em que ambiente o bot roda: efêmero do CI, homologação ou máquina local? | bot em produção simula jogador real e distorce métrica; bot em CI efêmero precisa de servidor completo, que ainda não existe | `wyd-ops` + arquitetura | aberta |
| `LBOT-02` | Como nasce a conta e o personagem sintético de cada cenário? | conta real ou banco compartilhado produz teste não reprodutível e risco de dado pessoal | `wyd-db` + QA | aberta |
| `LBOT-03` | Qual é o limiar de carga (conexões, duração, latência aceitável) e o que reprova? | limiar sem medição vira promessa; `AGENTS.md` §8 exige gate medido | QA + `wyd-ops` | aberta |
| `LBOT-04` | O que uma execução falha entrega: log, captura, trace ou nada? | sem artefato a falha não é reproduzível e o custo do teste não se paga | QA | aberta |

## 3. O que não é lacuna deste sistema

- Codec em modo cliente: **decidido** (`wyd-protocol` §1.4).
- Provisionamento de ambiente: dono é `wyd-ops`.
