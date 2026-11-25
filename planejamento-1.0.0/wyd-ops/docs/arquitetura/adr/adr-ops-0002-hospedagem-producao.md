# ADR-OPS-0002 — Hospedagem de produção em VPS paga com Compose

> Pergunta: "qual decisão estrutural esta ADR registra e como verificá-la?"
> Público: Arquitetura, `wyd-ops` e dono do produto
> Dono: `wyd-ops` (com aprovação do dono do produto)
> Base: `docs/proposta-decisoes-ops.md` §4 e §6; `proposta-gestao-github-e-ambientes.md` §7.3; `plano-ci-real.md` §3; `AGENTS.md` §9
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-25). `Q-OPS-2` = A aprovada pelo dono em 2026-11-24; provedor, contrato, VPS e backup ainda não aplicados.

## Contexto

O `wyd-ops` precisa de um host com processo persistente, porta TCP e estado em memória para o servidor de jogo. O free tier hiberna, expira ou não oferece compute suficiente; por isso não serve para produção de jogo (`AGENTS.md` §9). O runtime já decidido é `docker compose` em host com systemd, não Kubernetes (`plano-ci-real.md` §3). Nada está publicado e não há custo, VPS ou backup medidos (`V`, 2026-11-25).

## Decisão

Produção roda em **VPS paga com processo persistente e deploy Compose**, escolhida por custo e controle. O provedor, a região, o contrato e a rotina de backup são aplicação posterior, não reabertura desta decisão.

## Consequências

- Positivas: mantém o runtime Compose já decidido; não depende de crédito temporário; permite processo persistente e porta TCP.
- Negativas/custos: custo recorrente; exige contrato, provisionamento, backup, RPO/RTO, monitoramento e operação.
- O que muda nos outros sistemas: `wyd-server` e `wyd-db` passam a ter um host de produção declarado; `wyd-ops` é dono do provisionamento e do rollback. Nenhum outro sistema muda de contrato por esta ADR.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| B — máquina própria | controle total, mas exige rede, energia, backup e operação local. |
| C — crédito de provedor | adia o custo, mas expira e não é plano de produção estável. |

## Como verificar

```bash
# quando aplicado, no host de produção
docker compose ps
curl -fsS http://127.0.0.1:<porta-de-saude>/health
# rollback e restore são provados pelo plano de operação, não por esta ADR
```

A decisão está em vigor quando existe host provisionado, Compose ativo, health check verde e restore de backup provado. Hoje **não existe** host, contrato, VPS ou backup.

## Referências

- Issue: não criada.
- Lacuna fechada: `Q-OPS-2` (decisão), aplicação em `OPS-04`.
- Achado da auditoria: `wyd-ops/auditoria-de-organizacao.md` §3.
- Seção de especificação: `AGENTS.md` §9; `wyd-1.0.0.md` §8.
- Documentos donos: `docs/proposta-decisoes-ops.md`; `proposta-gestao-github-e-ambientes.md`; `plano-ci-real.md`.