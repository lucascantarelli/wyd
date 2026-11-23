# wyd-sim 1.0.0 — abertura do planejamento

> Pergunta: "que cálculo o `wyd-sim` entrega, com que garantia de determinismo e o que falta para começar o código?"
> Público: Produto, arquitetura, QA e equipe do `wyd-sim`
> Dono: equipe do `wyd-sim`
> Base: `wyd-protocol-1.0.0.md` §1.2; `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7.2; `AGENTS.md` §6
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código**, nenhuma fórmula escrita e nenhum requisito numerado (`RF-SIM`). Este sistema
existe para que a regra de jogo não seja inventada por estimativa.

## 1. O que ele faz

Concentra o cálculo do jogo em funções puras e determinísticas: dano, chance, curva de nível,
experiência, drop e cooldown. Recebe valores tipados — do `wyd-core` em tempo de jogo, do catálogo
em `content` — e devolve resultado. Não lê banco, não abre socket e não guarda estado entre
chamadas: a mesma entrada produz a mesma saída.

## 2. O que ele **não** faz

| Fora | Dono |
|---|---|
| decidir **quando** a fórmula é aplicada | `wyd-core` |
| carregar e versionar o dado de conteúdo (item, skill, curva) | `wyd-db` (`content`) |
| arredondar para o cliente ou formatar para a tela | `wyd-protocol` / `wyd-portal` |
| equilibrar o jogo (valor final de dano, EXP) | produto |
| guardar histórico de cálculo | `wyd-db` (`ops.audit_log`, quando auditável) |

## 3. Contratos

| # | Contrato | Direção | Forma | Garantia | Dono | Estado |
|---|---|---|---|---|---|---|
| M1 | função de cálculo | sim ← `wyd-core` | função pura, sem I/O | determinística; mesma entrada, mesma saída | `wyd-sim` | **proposto** |
| M2 | origem dos coeficientes | sim ← `content` | valor tipado lido pelo chamador | nenhum coeficiente dentro do código sem fonte | `wyd-db` | **proposto** |
| M3 | versão do cálculo | sim declara | `formula_version` próprio + `content_version` usado, na resposta | mudança de fórmula é rastreável | `wyd-sim` + `wyd-db` | **decidido** `LSIM-02` (`Q-SIM-2` = A, 2026-11-23) |
| M4 | prova de determinismo | sim → QA | fixtures JSON versionadas com entrada, saída esperada e identificador da versão de fórmula | reprova quando diverge | QA + `wyd-sim` | **decidido** `LSIM-03` (`Q-SIM-3` = A, 2026-11-23); fixture não implementada |

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-sim -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-sim -type f` (só `.gitkeep`) |
| Fórmulas documentadas com exemplo | 0 | esta abertura não escreve fórmula |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 5. O que falta

`lacunas.md`: nenhuma pergunta segue aberta. `LSIM-02` fechou em 2026-11-23 por resposta explícita do
dono (`Q-SIM-2` = A): cada fórmula carrega `formula_version` próprio, registrado na spec e no cabeçalho
do fixture junto do número de `content_version` usado. `LSIM-03` fechou na mesma data (`Q-SIM-3` = A):
fixtures JSON versionadas em `tests/fixtures/sim/<formula>.json`, com entrada, saída esperada e
identificador da versão de fórmula (os arquivos ainda não existem). `LSIM-01` (lista de fórmulas) fechou em
2026-11-23: fatia vertical com vetor por fórmula — dano físico, chance de acerto, drop, uma fórmula de
economia e o deslocamento puro do passo 6. `LSIM-04` (aritmética) fechou na mesma data: aritmética
inteira na unidade do domínio, percentual em bps, sem escala global (a 1/1000 foi dispensada por falta
de consumidor no schema) e arredondamento meio-para-longe-do-zero uma vez por fórmula, `f64` só em
exibição. As decisões estão em `docs/proposta-decisoes-sim.md` §4.

## 6. Gate para sair do papel

1. Primeira fórmula escrita **com exemplo numérico** e origem do coeficiente citada — a lista está
decidida (`LSIM-01`), o formato/local do vetor está decidido (`LSIM-03`, `Q-SIM-3` = A) e a versão do
cabeçalho também (`LSIM-02`, `Q-SIM-2` = A); faltam escrever a fórmula, os valores do vetor e a fixture
versionada.
2. `LSIM-04` decidida em 2026-11-23: aritmética inteira, percentual em bps, sem escala global, e
   `f64` só em exibição (`docs/proposta-decisoes-sim.md` §4).
3. `RF-SIM` numerados com vetor de prova por requisito.
4. Árvore de `src/` e `tests/` planejada; nenhum teste com `sleep` ou relógio real.
5. Nenhum número de balanceamento inventado: valor sem fonte é `H` com lacuna.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/formulas/` (uma página por família de cálculo),
`docs/guia-teste/vetores-deterministicos.md` e `docs/roadmap/` com DoD.
