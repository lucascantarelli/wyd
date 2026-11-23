# `wyd-core` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-22).** Único lugar onde uma lacuna do `wyd-core` tem dono,
> impacto e estado. Nada aqui está decidido.

## 0. Como ler

Estados: **aberta** (sem decisão), **proposta** (recomendação escrita, falta o "sim"),
**fechada** (decidida, com evidência). Dono propõe `arquitetura`; o dono do sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Escopo e catálogo | 1 | 1 | 0 | 0 |
| Modelo de estado | 1 | 1 | 0 | 0 |
| Fronteira com `wyd-sim` | 1 | 1 | 0 | 0 |
| Persistência | 1 | 1 | 0 | 0 |
| **Total** | **4** | **4** | **0** | **0** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LCORE-01` | Que funcionalidades o `wyd-core` entrega na `1.0.0`, já que o catálogo funcional não existe? | sem entrada de catálogo não há meta de cobertura nem tamanho de sistema; hoje a `1.0.0` é "jogo completo" sem lista (`wyd-1.0.0.md` §1.5) | produto + arquitetura | aberta |
| `LCORE-02` | O que é estado em memória e o que é durável, e com que frequência o durável é escrito? | define save em lote, perda aceitável e o contrato com `wyd-db` (`WYDDB-017`) | arquitetura + `wyd-db` | aberta |
| `LCORE-03` | Onde termina a regra do core e começa a fórmula do `wyd-sim`? | a divisão foi decidida em tese (2026-11-20); sem critério de fronteira, a mesma regra nasce duas vezes | arquitetura | aberta |
| `LCORE-04` | Como o core lê e grava sem SQL próprio, e quais operações são transacionais entre domínios? | `P-17` e a separação de papéis proíbem SQL espalhado; a porta precisa de contrato | `wyd-db` + arquitetura | aberta |

## 3. O que não é lacuna deste sistema

- Payload tipado na fronteira: **decidido** (`LPRT-18`).
- Ordem por conexão: **decidida** (`LPRT-12`), dono `wyd-net`.
