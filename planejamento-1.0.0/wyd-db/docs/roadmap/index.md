> Pergunta: "qual é a próxima tarefa, o que está bloqueado e onde está o DoD?"
> Público: Produto, desenvolvimento, QA e operação
> Dono: equipe do `wyd-db`
> Base: `D-7` aprovada em 2026-11-22; `fases.md`; `wyd-db-1.0.0.md` §6.10
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

A fila tem **40 issues** em sete fases. `FASE` agrupa escopo; `P-n` ordena prioridade. O roadmap não
é uma lista de desejos: cada linha aponta uma página `WYDDB-nnn` com entregável, DoD e evidência de
fechamento. Issue sem página com DoD executável não entra na fila.

## 1. Fila por prioridade

| Prioridade | Issues | Foco |
|---|---|---|
| `P1` | `WYDDB-001`–`016` | baseline, contratos e portas; sem isso o restante não tem fronteira |
| `P2` | `WYDDB-017`–`026` | filas, save, shutdown e diagnóstico |
| `P3` | `WYDDB-027`–`034` | suíte, fixtures e anti-skip |
| `P3` | `WYDDB-035`–`040` | série, contrato de migrations e troca |

As fases e os critérios de saída estão em [`fases.md`](fases.md). A prioridade não muda a dependência:
uma issue `P1` da fase 4 continua esperando a fase 3.

## 2. O que está bloqueado

| Bloco | Issues afetadas | Por quê |
|---|---|---|
| seed de conteúdo | `WYDDB-035`–`040` e o corte vertical | os seis seeds `0016`–`0018`, `0020`, `0023` e `0027` esperam dado de origem conferido |
| captura do protocolo | `WYDDB-036`–`039` | `LPRT-09`, `LPRT-03` e `LPRT-06` não têm captura datada do cliente 7662 |
| CI real | `WYDDB-027`–`034` | o workflow é stub; o gate de docs não é suíte do crate |
| matriz de capacidade | `WYDDB-034` | `CAP-nn` não existe; a página de anti-skip declara a lacuna |

O bloqueio é de **dado ou ambiente**, não de decisão: `D-7` está aprovada e as 40 páginas existem.
Nenhuma issue bloqueada é marcada como concluída por ter página.

## 3. Como ler uma issue

Cada `WYDDB-nnn` tem: título, fase, prioridade, contexto, entregável, DoD, fora de escopo, risco,
dependência e evidência ao fechar. O DoD cita comando, arquivo ou documento de saída; a evidência fica
na issue do GitHub e o número é registrado no relatório de entrega.

## 4. O que esta página não cobre

- **Números medidos de progresso:** `planos/metricas.md` ainda não foi escrito; não há percentual.
- **Prazo e custo:** não há estimativa aprovada.
- **Regra de negócio ou schema:** são de `specs/` e do dicionário; o roadmap só fatia o trabalho.
