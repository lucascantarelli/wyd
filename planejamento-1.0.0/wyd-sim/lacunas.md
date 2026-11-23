# `wyd-sim` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-22).** Único lugar onde uma lacuna do `wyd-sim` tem dono,
> impacto e estado. Nada aqui está decidido.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; produto aprova
o que é balanceamento.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Catálogo de fórmulas | 1 | 1 | 0 | 0 |
| Rastreabilidade | 1 | 1 | 0 | 0 |
| Prova | 1 | 1 | 0 | 0 |
| Aritmética | 1 | 1 | 0 | 0 |
| **Total** | **4** | **4** | **0** | **0** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LSIM-01` | Quais fórmulas existem na `1.0.0` e qual exemplo numérico prova cada uma? | sem lista, o core inventa cálculo no meio da regra e a cobertura não é medida | produto + arquitetura | aberta |
| `LSIM-02` | Como a versão da fórmula acompanha o `content_version` e o que muda quando o dado muda? | `game.character` já carrega `content_version`; cálculo sem versão quebra reprodutibilidade | `wyd-db` + arquitetura | aberta |
| `LSIM-03` | Onde ficam os vetores de entrada e saída que provam determinismo? | prova de fórmula sem vetor é opinião; o alvo de teste precisa de fixture | QA + `wyd-sim` | aberta |
| `LSIM-04` | Ponto flutuante é permitido, e com que arredondamento? | dano e economia com `f64` divergem entre plataformas; dinheiro é inteiro no `wyd-db` (`P-06`) | arquitetura | aberta |

## 3. O que não é lacuna deste sistema

- Quem aplica a fórmula e em que momento: dono é `wyd-core`.
- O valor final de balanceamento: decisão de produto, não de implementação.
