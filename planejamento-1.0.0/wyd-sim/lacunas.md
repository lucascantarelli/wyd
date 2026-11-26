# `wyd-sim` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Único lugar onde uma lacuna do `wyd-sim` tem dono,
> impacto e estado. As quatro lacunas fecharam em 2026-11-23 pelas decisões do dono: `LSIM-01`
> (`Q-SIM-1` = A), `LSIM-04` (`Q-SIM-4` = A), `LSIM-03` (`Q-SIM-3` = A) e `LSIM-02` (`Q-SIM-2` = A).
> O formato dos vetores e a versão da fórmula estão decididos, mas nenhuma fórmula ou fixture foi
> implementada.

## 0. Como ler

Estados: **fechada** (com evidência citada) e **bloqueada** (falta decisão, execução ou evidência; a
linha nomeia o desbloqueio — dono e documento). `arquitetura` propõe; produto aprova
o que é balanceamento.

## 1. Situação em uma linha

| Grupo | Lacunas | Bloqueadas | Fechadas |
|---|---:|---:|---:|
| Catálogo de fórmulas | 1 | 0 | 1 |
| Rastreabilidade | 1 | 0 | 1 |
| Prova | 1 | 0 | 1 |
| Aritmética | 1 | 0 | 1 |
| **Total** | **4** | **0** | **4** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LSIM-01` | Quais fórmulas existem na `1.0.0` e qual exemplo numérico prova cada uma? | sem lista, o core inventa cálculo no meio da regra e a cobertura não é medida | produto + arquitetura | fechada (2026-11-23: decisão do dono, `Q-SIM-1` opção **A** — fatia vertical com vetor por fórmula: dano físico, chance de acerto, drop, uma fórmula de economia e o **deslocamento puro** do passo 6 do esqueleto; o coeficiente vem do conteúdo e o exemplo numérico nasce com o vetor em `LSIM-03`; registro em `docs/proposta-decisoes-sim.md` §4 e §6; implementação pendente) |
| `LSIM-02` | Como a versão da fórmula acompanha o `content_version` e o que muda quando o dado muda? | `game.character` já carrega `content_version`; cálculo sem versão quebra reprodutibilidade | `wyd-db` + arquitetura | fechada (2026-11-23: decisão do dono, `Q-SIM-2` opção **A** — cada fórmula carrega versão própria (`formula_version`, inteiro a partir de 1), declarada na spec da fórmula e no cabeçalho do fixture junto do **número** de `content_version` usado no cálculo (`content.content_version.version`, não o `id` da linha única); mudança de código que altere o resultado incrementa a versão da fórmula, mudança de conteúdo mantém a versão e é registrada pelo `content_version`; não cria tabela, coluna, fórmula nem fixture, e persistir o par versão/conteúdo no personagem, se houver consumidor, é decisão do `wyd-db`; registro em `docs/proposta-decisoes-sim.md` §4 e §6) |
| `LSIM-03` | Onde ficam os vetores de entrada e saída que provam determinismo? | prova de fórmula sem vetor é opinião; o alvo de teste precisa de fixture | QA + `wyd-sim` | fechada (2026-11-23: resposta explícita do dono, `Q-SIM-3` = **A** — fixtures JSON versionadas em `tests/fixtures/sim/<formula>.json`, com entrada, saída esperada e identificador da versão de fórmula; registro em `docs/proposta-decisoes-sim.md` §4 e §6. Fixture ainda não implementada.) |
| `LSIM-04` | Ponto flutuante é permitido, e com que arredondamento? | dano e economia com `f64` divergem entre plataformas; dinheiro é inteiro no `wyd-db` (`P-07`) | arquitetura | fechada (2026-11-23: decisão do dono, `Q-SIM-4` opção **A** — aritmética **inteira** em estado e economia, em inteiro de 64 bits na unidade do domínio e percentual em **bps**, **sem escala global** (a 1/1000 foi dispensada na mesma data porque nenhuma coluna do schema guarda fração: vida/AC/dano em `INTEGER`/`SMALLINT`, dano de arena `BIGINT`, dinheiro inteiro e taxa em bps), arredondamento **meio-para-longe-do-zero** uma única vez no fim de cada fórmula, divisão inteira pelo denominador da unidade e overflow verificado, dinheiro em unidades menores inteiras (`P-07`) e `f64` só na formatação de exibição; escala local só na spec da fórmula que precisar; registro em `docs/proposta-decisoes-sim.md` §4 e §6; implementação pendente) |

## 3. O que não é lacuna deste sistema

- Quem aplica a fórmula e em que momento: dono é `wyd-core`.
- O valor final de balanceamento: decisão de produto, não de implementação.
