> Pergunta: "qual é a próxima fatia do `wyd-server`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-server`
> Base: `wyd-server-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada.

A fila tem **7 fatias** em 4 fases.

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDSRV-001`–`002` | baseline do crate e composição |
| 2 | `WYDSRV-003`–`004` | um processo por canal e controle HTTP |
| 3 | `WYDSRV-005`–`006` | integração com net/core/db/admin |
| 4 | `WYDSRV-007` | suíte, observabilidade e anti-skip |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| captura S7 | `WYDSRV-003`–`004` | `LSRV-05` em C com condição reprovada (peer de A1) |
| contrato admin | `WYDSRV-005` | canal de controle HTTP síncrono com ack (`LAC-ADM-03`) |
| CI real | `WYDSRV-007` | workflow stub |

## 3. Como ler uma fatia

Cada `WYDSRV-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.