> Pergunta: "qual é a próxima fatia do `wyd-admin`, o que está bloqueado e onde está o DoD?"
> Público: produto, arquitetura, desenvolvimento e QA
> Dono: equipe do wyd-admin
> Base: `roadmap.md`; `../specs/requisitos.md`; `../arquitetura/contrato-c4.md`; `planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md`
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-26). Nenhuma fatia iniciada; as 10 fatias já existem como issues #208–#217 no board `WYD 1.0.0`.

A fila tem **10 fatias**, uma por página citada abaixo. Cada página é dona do estado, fase,
prioridade, entregável, DoD, fora de escopo, dependência, skills e evidência; este índice só cataloga
e aponta o campo.

## 1. Fila

A ordem é a de `roadmap.md` §2. A fase e a prioridade de cada issue são os campos `**Fase:**` e
`**Prioridade:**` da página dela; esta página não repete fila, fase nem prioridade.

## 2. O que está bloqueado

O bloqueio de cada issue é o `## Depende de` da página dela; esta página não repete o motivo nem a
lista de afetadas. Nenhuma issue depende de decisão aberta: as sete lacunas do `wyd-admin` fecharam
até 2026-11-22.

## 3. Como ler uma fatia

Cada página tem: título, estado, fase, prioridade, contexto, entregável, DoD, fora de escopo,
dependência, bloco de skills e evidência ao fechar. O DoD cita comando, arquivo, status HTTP ou teste.

## 4. Páginas de issue

| Issue | Título |
|---|---|
| [`ADM-01`](WYDADM-01.md) §Skills | esqueleto HTTP, config e saúde |
| [`ADM-02`](WYDADM-02.md) §Skills | autenticação de serviço e trace_id |
| [`ADM-03`](WYDADM-03.md) §Skills | idempotência, replay e consulta de ação |
| [`ADM-04`](WYDADM-04.md) §Skills | moderação de conta |
| [`ADM-05`](WYDADM-05.md) §Skills | ajuste de carteira |
| [`ADM-06`](WYDADM-06.md) §Skills | configuração de canal |
| [`ADM-07`](WYDADM-07.md) §Skills | imposto e preço |
| [`ADM-08`](WYDADM-08.md) §Skills | kick e broadcast |
| [`ADM-09`](WYDADM-09.md) §Skills | auditoria, log e cobertura |
| [`ADM-10`](WYDADM-10.md) §Skills | custódia de item do mercado |

## 5. O que esta página não cobre

- Prazo e custo: não há estimativa aprovada.
- Regra de negócio ou schema: são de `specs/`; o roadmap só fatia o trabalho.
- Criação de issue, label, milestone ou board: ato explícito do dono.
