# Mapa de funcionalidades do cliente WYD

> Pergunta: "quais funcionalidades do cliente estão mapeadas, documentadas e qual o estado de cada uma?"
> Público: Produto, arquitetura e desenvolvimento
> Dono: equipe do `wyd-client`
> Base: `docs/specs/requisitos.md` (RF-CLI/RNF-CLI); `docs/specs/inventario-da-arvore.md`; `docs/specs/pedido-ao-titular.md`; `docs/specs/propriedade-intelectual-e-distribuicao.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Mapa consolidado; captura e distribuição ainda em proposta.

## 1. Funcionalidades mapeadas

| # | Funcionalidade | Requisito | Estado | Onde |
|---|---|---|---|---|
| 1 | Inventário item a item da árvore do cliente | `RF-CLI-001` | proposto (hash/tamanho) | `docs/specs/inventario-da-arvore.md` |
| 2 | Pedido ao titular dos direitos | `RF-CLI-002` | iniciado (não enviado) | `docs/specs/pedido-ao-titular.md` |
| 3 | Patches e arquivos de configuração | `RF-CLI-003`,`RF-CLI-004` | proposto | `docs/specs/requisitos.md` |
| 4 | Uso e distribuição (IP) | `RF-CLI-005`,`RF-CLI-006` | em proposta (`Q-IP-3`–`Q-IP-6` decididas) | `docs/specs/propriedade-intelectual-e-distribuicao.md` |
| 5 | Lacunas e fechamento | `RF-CLI-007` | parcial (`LCLI-03`/`05` abertas; `LCLI-02` fechada em 2026-11-25) | `wyd-client-1.0.0.md`, `lacunas.md` |

## 2. O que não está mapeado

- Captura datada do cliente 7662 (A1 autorizada 2026-11-23; execução pendente).
- Segunda leitura com QA (`LCLI-03`).
- Distribuição pública (congelada; escopo B).

## 3. O que esta página não cobre

- Conteúdo do cliente (binário proprietário, ~344 MB, ignorado pelo git).
- Regra de negócio de jogo (dono: `wyd-core`/`wyd-sim`).
