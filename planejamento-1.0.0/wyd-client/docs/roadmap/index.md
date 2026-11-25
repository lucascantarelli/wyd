> Pergunta: "qual é a próxima fatia do `wyd-client`, o que está bloqueado e onde está o DoD?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: equipe do `wyd-client`
> Base: `wyd-client-1.0.0.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada; nenhuma issue criada.

A fila tem **5 fatias** em 3 fases. Uso/distribuição está em proposta (responsável nominal, distribuição
congelada, `Q-IP-1`/`Q-IP-2` com pedido externo iniciado, `Q-IP-2` no escopo B).

## 1. Fila por fase

| Fase | Fatias | Foco |
|---|---|---|
| 1 | `WYDCLI-001`–`002` | inventário e captura (A1 autorizada 2026-11-23) |
| 2 | `WYDCLI-003`–`004` | patches e arquivos de configuração do cliente |
| 3 | `WYDCLI-005` | documentação, lacunas e fechamento |

## 2. O que está bloqueado

| Bloco | Fatias afetadas | Por quê |
|---|---|---|
| pedido ao titular | `WYDCLI-001`–`002` | `Q-IP-1`/`Q-IP-2` com pedido externo iniciado, não enviado |
| segunda leitura | `WYDCLI-003` | `LCLI-03` (QA definido, operador na ficha) |
| CI real | `WYDCLI-005` | workflow stub |

## 3. Como ler uma fatia

Cada `WYDCLI-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar.

## 4. O que esta página não cobre

- **Prazo e custo:** não há estimativa aprovada.