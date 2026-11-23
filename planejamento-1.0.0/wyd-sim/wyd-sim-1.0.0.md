# wyd-sim 1.0.0 — abertura do planejamento

> Pergunta: "que cálculo o `wyd-sim` entrega, com que garantia de determinismo e o que falta para começar o código?"
> Público: Produto, arquitetura, QA e equipe do `wyd-sim`
> Dono: equipe do `wyd-sim`
> Base: `wyd-protocol-1.0.0.md` §1.2; `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7.2; `AGENTS.md` §6
> Revisado em: 2026-11-22

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
| M3 | versão do cálculo | sim declara | versão ou `content_version` na resposta | mudança de fórmula é rastreável | `wyd-sim` + `wyd-db` | **lacuna** `LSIM-02` |
| M4 | prova de determinismo | sim → QA | vetor de entrada e saída esperada | reprova quando diverge | QA + `wyd-sim` | **lacuna** `LSIM-03` |

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-sim -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-sim -type f` (só `.gitkeep`) |
| Fórmulas documentadas com exemplo | 0 | esta abertura não escreve fórmula |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 5. O que falta

`lacunas.md`: `LSIM-01` (quais fórmulas existem e com que exemplo numérico), `LSIM-02` (como a
versão do cálculo acompanha o `content_version`), `LSIM-03` (onde ficam os vetores de prova) e
`LSIM-04` (resolução de ponto flutuante e arredondamento).

## 6. Gate para sair do papel

1. Primeira fórmula escrita **com exemplo numérico** e origem do coeficiente citada.
2. `LSIM-04` decidida: ponto flutuante proibido ou com política explícita.
3. `RF-SIM` numerados com vetor de prova por requisito.
4. Árvore de `src/` e `tests/` planejada; nenhum teste com `sleep` ou relógio real.
5. Nenhum número de balanceamento inventado: valor sem fonte é `H` com lacuna.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/formulas/` (uma página por família de cálculo),
`docs/guia-teste/vetores-deterministicos.md` e `docs/roadmap/` com DoD.
