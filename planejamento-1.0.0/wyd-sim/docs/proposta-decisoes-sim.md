# Decisões do `wyd-sim` — fórmulas, versão, vetores e aritmética

> Pergunta: "quais fórmulas existem, como a versão acompanha o conteúdo, onde ficam os vetores de
> prova e ponto flutuante é permitido?"
> Público: produto, arquitetura, `wyd-sim`, `wyd-core`, `wyd-db` e QA
> Dono: equipe do `wyd-sim`
> Base: `lacunas.md` (`LSIM-01`–`LSIM-04`); `docs/index.md`; `wyd-sim-1.0.0.md` §1–§6;
> `wyd-1.0.0.md` §7.1, §7.3 (X4) e §7.6; `wyd-db/padrao-banco.md` `P-07`; `AGENTS.md` §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Quatro perguntas, uma por lacuna aberta. Nenhuma está respondida e
nada foi aplicado. Não há fórmula, vetor nem execução; nenhum coeficiente é medido.

## 1. Fronteira

**Fecha:** lista de fórmulas, versionamento contra `content_version`, vetores de determinismo e
política aritmética — `LSIM-01` a `LSIM-04`.

**Não decide:** quando a fórmula é aplicada (`wyd-core`), valor de balanceamento (produto), carga do
dado (`wyd-db`), formatação (`wyd-portal`/`wyd-protocol`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **4 abertas, 0 propostas, 0 fechadas** | `lacunas.md` §1 |
| fórmulas escritas | **0** | `wyd-sim-1.0.0.md` §4 |
| vetores de prova | **0** | `wyd-1.0.0.md` §7.6 |
| código | **0** (só `.gitkeep`) | `wyd-sim-1.0.0.md` §4 |
| desempenho de cálculo | **não medido** | `docs/index.md` |
| contrato X4 | fórmula determinística: **proposto** | `wyd-1.0.0.md` §7.3 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| dinheiro inteiro | `P-07`: dinheiro é inteiro no banco | `wyd-db/padrao-banco.md` `P-07` | a aritmética de economia não usa `f64` |
| cálculo puro | `wyd-sim` é cálculo determinístico; `wyd-core` aplica | `wyd-1.0.0.md` §7.1 | a fronteira é a pergunta, não o princípio |
| cobertura e anti-skip | 85% por sistema, sem `SKIP` silencioso | `AGENTS.md` §8 | vetor de prova não pode ser pulado |

## 4. Decisões

### Q-SIM-1 — Quais fórmulas existem na 1.0.0?

**Medido.** Não há catálogo funcional nem fórmula escrita; `LSIM-01` está aberta e a cobertura da
1.0.0 "não medido" (`wyd-1.0.0.md` §1.5). Nada foi executado.

**O que está em jogo.** "Todas as fórmulas do legado" não tem lista; esperar o catálogo bloqueia;
uma fatia vertical prova o contrato com o core e cresce.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Fatia vertical: dano físico, chance de acerto, drop e uma fórmula de economia, cada uma com vetor | prova o contrato X4 e a aritmética; cobertura parcial declarada |
| B | Lista completa do legado | cobre mais; sem catálogo, o tamanho e a prova são indefinidos |
| C | Esperar o catálogo | nenhum retrabalho; o sim fica sem fórmula |

**Recomendação: A.** É o menor conjunto que prova determinismo e a fronteira com o core; o catálogo
depois acrescenta. **Custo:** cobertura parcial. **Fontes:** `lacunas.md` `LSIM-01`;
`wyd-1.0.0.md` §1.5 e X4; `wyd-sim-1.0.0.md` §6. **Recusa B** por lista inexistente; **C** por
bloquear.

### Q-SIM-2 — Como a versão da fórmula acompanha o `content_version`?

**Medido.** `game.character` já carrega `content_version`; não há versão de fórmula nem regra de
mudança. `LSIM-02` está aberta.

**O que está em jogo.** Sem versão, mudar coeficiente quebra a reprodutibilidade de um cálculo antigo;
amarrar tudo ao `content_version` mistura dado e código; uma versão própria separa os dois.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Versão de fórmula própria, registrada com o `content_version` usado | reproduz cálculo antigo; exige campo/registro e disciplina de mudança |
| B | Amarrar a fórmula ao `content_version` | uma chave só; mudar código sem mudar conteúdo fica invisível |
| C | Sem versão | simples; cálculo antigo não é reproduzível |

**Recomendação: A.** Separa o que muda por código do que muda por dado, sem perder a combinação.
**Custo:** um identificador de versão. **Fontes:** `lacunas.md` `LSIM-02`;
`wyd-1.0.0.md` X4; `wyd-db/migrations/0006-game-character.md` §6.1 (`content_version`). **Recusa B** por
misturar; **C** por quebrar reprodutibilidade.

### Q-SIM-3 — Onde ficam os vetores de entrada e saída?

**Medido.** Não há vetor nem fixture; `LSIM-03` aberta e `wyd-1.0.0.md` §7.6 diz "0 vetores".
O alvo de teste precisa de fixture.

**O que está em jogo.** Vetor no código é fácil de perder; fixture versionada é reproduzível; vetor
externo não é auditável no repositório.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | `tests/fixtures/sim/<formula>.json` com entrada, saída e versão, versionado | prova reproduzível e revisável; exige disciplina de atualização |
| B | Vetores inline no teste | simples; dificulta reuso e revisão por fórmula |
| C | Vetores fora do repositório | não polui; não é auditável nem versionado com o código |

**Recomendação: A.** É a forma reproduzível que o alvo de teste aceita e permite rodar a mesma prova
em Rust; o caminho é proposta, não um path já decidido. **Custo:** fixtures por fórmula. **Fontes:**
`lacunas.md` `LSIM-03`; `wyd-1.0.0.md` §7.6; `AGENTS.md` §8. **Recusa B** por baixo reuso; **C** por
não ser auditável.

### Q-SIM-4 — Ponto flutuante é permitido e com que arredondamento?

**Medido.** Não há política aritmética; `LSIM-04` aberta; `P-07` já fixa dinheiro inteiro no banco.
A linha de `LSIM-04` cita `P-06`, mas a regra de dinheiro inteiro é `P-07`; a página segue a fonte. Nenhum
cálculo foi executado.

**O que está em jogo.** `f64` diverge entre plataformas e quebra determinismo; inteiro/fixed-point é
reproduzível, mas exige escala; misturar os dois cria borda.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Inteiro/fixed-point para tudo que afeta estado e economia; `f64` só para exibição, se necessário | determinismo e dinheiro inteiro; exige escala e arredondamento declarado |
| B | `f64` em toda fórmula | simples; divergência entre plataformas e quebra do determinismo |
| C | Tudo inteiro, inclusive dano e curva | determinístico; qualquer fração vira escala inteira e não há representação fracionária |

**Recomendação: A.** `P-07` já decide dinheiro inteiro; a política estende o mesmo critério ao estado.
**Custo:** uma escala e uma regra de arredondamento. **Fontes:** `lacunas.md` `LSIM-04`;
`wyd-db/padrao-banco.md` `P-07`; `wyd-sim-1.0.0.md` §2. **Recusa B** por determinismo; **C** por não representar fração.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LSIM-01` | `Q-SIM-1` | aberta |
| `LSIM-02` | `Q-SIM-2` | aberta |
| `LSIM-03` | `Q-SIM-3` | aberta |
| `LSIM-04` | `Q-SIM-4` | aberta |

## 6. Registro da decisão

**Aguardando resposta do dono.** As quatro lacunas continuam `aberta`; nada foi aplicado.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-SIM-1` | `LSIM-01` | — |
| `Q-SIM-2` | `LSIM-02` | — |
| `Q-SIM-3` | `LSIM-03` | — |
| `Q-SIM-4` | `LSIM-04` | — |
