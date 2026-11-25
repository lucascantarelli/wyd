> Pergunta: "quais requisitos o `wyd-core` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-core-1.0.0.md` §1, §3, §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados da abertura; catálogo completo na `1.0.0`, primeiro domínio: entrar na cena.

Requisito sem critério de aceitação é intenção. Numeração `RF-CORE-nnn` (funcional) e `RNF-CORE-nnn` (não funcional).

## Requisitos funcionais

### RF-CORE-001 — Avançar o tick determinístico
- **Entrada:** semente e passo de tempo.
- **Saída:** estado atualizado.
- **Aceitação:** mesma semente → mesma sequência; sem `sleep`.
- **Contrato:** `LCORE-01`.

### RF-CORE-002 — Manter entidades e componentes em memória
- **Entrada:** spawn/despawn/query.
- **Saída:** estado da entidade.
- **Aceitação:** CRUD e query por filtro; estado reconstruível.
- **Contrato:** `LCORE-02`.

### RF-CORE-003 — Resolver cena, mapa e colisão
- **Entrada:** posição e mapa.
- **Saída:** posição resultante.
- **Aceitação:** colisão respeitada; deslocamento vem do `wyd-sim`.
- **Contrato:** `LCORE-03`, `LSIM-01`.

### RF-CORE-004 — Aplicar combate sem duplicar fórmula
- **Entrada:** ataque com alvo.
- **Saída:** efeito aplicado + evento.
- **Aceitação:** fórmula do `sim`; sorteio no core; duplicação é defeito.
- **Contrato:** `LCORE-03`.

### RF-CORE-005 — Decidir comportamento de IA
- **Entrada:** estado da entidade.
- **Saída:** decisão de alvo/momento.
- **Aceitação:** mudança de estado muda decisão.
- **Contrato:** `LCORE-03`.

### RF-CORE-006 — Persistir via porta sem SQL
- **Entrada:** `load_session`, `load_character`, `commit(UoW)`.
- **Saída:** snapshot ou receipt.
- **Aceitação:** sem `sqlx`/SQL no core; ausência é `None`; UoW atômica.
- **Contrato:** `LCORE-04`, C4.

### RF-CORE-007 — Classificar estado durável/memória
- **Entrada:** estado novo.
- **Saída:** classe (crítico/eventual/memória).
- **Aceitação:** dado novo entra na tabela antes do código; dúvida → durável crítico.
- **Contrato:** `LCORE-02`, C6.

### RF-CORE-008 — Publicar eventos internos
- **Entrada:** evento.
- **Saída:** entrega ordenada.
- **Aceitação:** ordem por conexão preservada.
- **Contrato:** `LPRT-12`.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-CORE-001 | Determinismo do tick | mesma semente → mesma sequência |
| RNF-CORE-002 | Sem SQL no core | grep limpo |
| RNF-CORE-003 | Sem I/O no `wyd-sim` | grep limpo |
| RNF-CORE-004 | UoW atômica | teste de sistema |
| RNF-CORE-005 | Cobertura mínima de 85% por capacidade | gate de cobertura |
| RNF-CORE-006 | Sem payload/PII em log | varredura de log |

*Página de planejamento. Nenhum requisito foi implementado.*