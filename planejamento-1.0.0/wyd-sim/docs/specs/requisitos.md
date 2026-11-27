> Pergunta: "quais requisitos o `wyd-sim` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-sim-1.0.0.md` §1, §3, §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados da abertura; `LSIM-01`–`LSIM-04` fechadas, implementação pendente.

Requisito sem critério de aceitação é intenção. Numeração `RF-SIM-nnn` (funcional) e `RNF-SIM-nnn` (não funcional).

## Requisitos funcionais

### RF-SIM-001 — Calcular dano físico/mágico
- **Entrada:** valores tipados.
- **Saída:** dano resultante.
- **Aceitação:** função pura; aritmética inteira; `formula_version` declarada.
- **Contrato:** `LSIM-01`, `LSIM-04`.

### RF-SIM-002 — Calcular chance de acerto
- **Entrada:** valores tipados.
- **Saída:** probabilidade em bps.
- **Aceitação:** pura; percentual em bps; sem RNG no sim.
- **Contrato:** `LSIM-01`, `LSIM-04`.

### RF-SIM-003 — Calcular drop
- **Entrada:** valores tipados.
- **Saída:** limiar/probabilidade.
- **Aceitação:** pura; sorteio fica no core.
- **Contrato:** `LSIM-01`.

### RF-SIM-004 — Calcular economia
- **Entrada:** valores tipados.
- **Saída:** quantia em unidades menores.
- **Aceitação:** dinheiro inteiro; arredondamento meio-para-longe-do-zero.
- **Contrato:** `LSIM-01`, `LSIM-04`, `P-07`.

### RF-SIM-005 — Calcular deslocamento puro
- **Entrada:** valores tipados.
- **Saída:** delta de posição.
- **Aceitação:** pura; sem colisão.
- **Contrato:** `LSIM-01`, passo 6.

### RF-SIM-006 — Declarar versão da fórmula
- **Entrada:** fórmula.
- **Saída:** `formula_version` + `content_version` usado.
- **Aceitação:** mudança de código incrementa versão; mudança de conteúdo mantém.
- **Contrato:** `LSIM-02`, `Q-SIM-2` = A.

### RF-SIM-007 — Provar determinismo por fixture
- **Entrada:** fixture JSON versionada.
- **Saída:** pass/fail.
- **Aceitação:** entrada, saída esperada, versão de fórmula; divergência reprova.
- **Contrato:** `LSIM-03`, `Q-SIM-3` = A.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-SIM-001 | Determinismo em todas as plataformas | mesmo vetor → mesma saída |
| RNF-SIM-002 | Aritmética inteira | sem f64 em estado/economia |
| RNF-SIM-003 | Overflow verificado | teste de overflow |
| RNF-SIM-004 | Sem I/O/estado/RNG/relógio | grep limpo |
| RNF-SIM-005 | Cobertura mínima de 85% por capacidade | gate de cobertura |
| RNF-SIM-006 | Sem número inventado | valor sem fonte é `H` |

*Página de planejamento. Nenhum requisito foi implementado.*
