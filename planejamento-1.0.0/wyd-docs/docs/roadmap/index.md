> Pergunta: "qual é a próxima fatia do `wyd-docs`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-docs`
> Base: `wyd-docs-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Site implementado; plano aberto. Nenhuma issue criada.

A fila tem **5 fatias** em 3 fases.

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDDOC-001`–`002` | baseline e montagem |
| 2 | `WYDDOC-003`–`004` | gate, contagens, censos |
| 3 | `WYDDOC-005` | publicação, redirect e tema |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| publicação | `WYDDOC-005` | `LPDOC-01` fechada; `LPDOC-12` (Windows) aberta como decisão |
| build em Linux | `WYDDOC-001` | Python 3.15 + MkDocs 2.0.dev6 |

## 3. Como ler uma fatia

Cada `WYDDOC-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.