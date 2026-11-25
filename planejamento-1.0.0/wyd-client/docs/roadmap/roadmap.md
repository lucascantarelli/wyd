> Pergunta: "em que ordem o `wyd-client` é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: equipe do `wyd-client`
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/componentes.md`; `wyd-portal/docs/roadmap/roadmap.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhuma fatia iniciada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Nenhuma informação de IP/credencial do titular entra em documento.
3. Toda fatia de captura inclui hash e tamanho do artefato.
4. Dependência de pedido externo é explícita.

## 2. Fatias

### F0 — Inventário e captura

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-CLI-001`, `RF-CLI-002` |
| Depende de | A1 autorizada; pedido ao titular |
| Entrega | inventário item a item (`docs/specs/inventario-da-arvore.md`), pacote do pedido (`docs/specs/pedido-ao-titular.md`) |
| DoD | inventário com hash/tamanho por arquivo; pedido enviado e protocolado |
| Fora | patches |

### F1 — Patches e configuração do cliente

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-CLI-003`, `RF-CLI-004` |
| Depende de | F0; `LCLI-02`/`LCLI-03` |
| Entrega | patches de configuração, arquivos de cliente, validação |
| DoD | patch aplica sem quebrar cliente; configuração documentada; teste de regressão verde |
| Fora | distribuição |

### F2 — Fechamento

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-CLI-005`, `RF-CLI-006`, `RF-CLI-007` |
| Depende de | F0–F1; CI real |
| Entrega | lacunas fechadas, documentação final, suíte |
| DoD | `LCLI-02`/`LCLI-03`/`LCLI-05` resolvidas ou com dono+prazo; cobertura ≥ 85% onde aplicável |
| Fora | distribuição pública |

## 3. Ordem e dependências

```text
F0 → F1 → F2
```

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F0 | pedido não respondido | escopo B; responsável nominal |
| F1 | config quebra cliente | teste de regressão |
| F2 | lacuna sem dono | `lacunas.md` com owner+prazo |

## 5. O que este roadmap não faz

- Não estima prazo em dias.
- Não cria issue, milestone ou board.