# `wyd-server` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-22).** Único lugar onde uma lacuna do `wyd-server` tem dono,
> impacto e estado. Nada aqui está decidido.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Topologia | 2 | 2 | 0 | 0 |
| Operação | 1 | 1 | 0 | 0 |
| Ciclo de vida | 1 | 1 | 0 | 0 |
| **Total** | **4** | **4** | **0** | **0** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LSRV-01` | Como o processo é composto (net + core + acesso a dados) e onde termina o `main`? | sem composição definida, cada crate vira um binário e a ordem de inicialização fica implícita | arquitetura | aberta |
| `LSRV-02` | Quantos canais a `1.0.0` abre e como o jogador escolhe? | a lista de canais aparece no launcher e no login; sem número, o teste de dois canais não existe | produto + `wyd-ops` | aberta |
| `LSRV-03` | O que `ready` significa: socket aberto, banco alcançável ou tick rodando? | `wyd-ops` usa prontidão para liberar tráfego; prontidão errada derruba jogador em deploy | `wyd-ops` + arquitetura | aberta |
| `LSRV-04` | O que acontece no encerramento: drena conexão, salva e sai, ou mata o processo? | o save em lote é de `wyd-db` (`WYDDB-017`, `WYDDB-021`); sem ordem, há perda de estado | `wyd-db` + arquitetura | aberta |

## 3. O que não é lacuna deste sistema

- Forma do comando de controle e autenticação do elo: **decidido** (`LAC-ADM-03`, 2026-11-22).
- Métricas expostas e coleta: **decidido** o caminho (`LAC-POR-06`); o catálogo de métricas é de
  `wyd-ops`.
