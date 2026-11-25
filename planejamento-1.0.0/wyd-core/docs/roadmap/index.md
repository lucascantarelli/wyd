> Pergunta: "qual é a próxima fatia do `wyd-core`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-core`
> Base: `wyd-core-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada.

A fila tem **7 fatias** em 4 fases. Cada item aponta uma página `WYDCORE-nnn` com entregável, DoD e
evidência de fechamento.

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDCORE-001`–`002` | baseline do crate e contrato do tick |
| 2 | `WYDCORE-003`–`004` | ECS, cena e combate |
| 3 | `WYDCORE-005`–`006` | IA e capacidades (catálogo funcional) |
| 4 | `WYDCORE-007` | suíte, fixtures e anti-skip |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| escopo funcional | `WYDCORE-005`–`006` | depende do catálogo funcional (quests, skills, classes, itens) |
| captura do protocolo | `WYDCORE-003`–`004` | offsets de pacote de combate (`LPRT-03`/`LPRT-06`) |

## 3. Como ler uma fatia

Cada `WYDCORE-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.
- **Regra de negócio ou schema:** são de `specs/`; o roadmap só fatia o trabalho.