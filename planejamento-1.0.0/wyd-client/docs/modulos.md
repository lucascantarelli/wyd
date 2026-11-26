# Módulos do `wyd-client` — detalhamento

> Pergunta: "o que cada módulo do `wyd-client` entrega, suas interfaces, dependências, testes, segurança, monitoramento e DoD?"
> Público: Desenvolvimento
> Dono: `wyd-client`
> Base: `docs/specs/requisitos.md`; `docs/specs/mapa-de-funcionalidades.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25).

## Módulos

### inventario
- **Objetivo:** inventário item a item da árvore do cliente.
- **Escopo:** hash/tamanho por arquivo.
- **Interfaces:** `docs/specs/inventario-da-arvore.md`.
- **Dependências:** cliente 7662.
- **Testes:** hash reproduzível.
- **Segurança:** sem segredo/PII.
- **Monitoramento:** n/a.
- **DoD:** inventário com hash/tamanho.

### pedido-titular
- **Objetivo:** pacote de pedido ao titular dos direitos.
- **Escopo:** `docs/specs/pedido-ao-titular.md`.
- **Interfaces:** documento.
- **Dependências:** inventário.
- **Testes:** revisão humana.
- **Segurança:** sem segredo.
- **Monitoramento:** n/a.
- **DoD:** pedido protocolado.

### patches
- **Objetivo:** patches de configuração do cliente.
- **Escopo:** arquivos de configuração.
- **Interfaces:** patch aplicável.
- **Dependências:** inventário.
- **Testes:** patch aplica sem quebrar cliente.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** teste de regressão verde.

### propriedade-intelectual
- **Objetivo:** política de uso/distribuição.
- **Escopo:** `propriedade-intelectual-e-distribuicao.md`.
- **Interfaces:** documento.
- **Dependências:** pedido ao titular.
- **Testes:** revisão legal.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** `Q-IP-3..6` fechadas.

### fechamento
- **Objetivo:** lacunas fechadas, documentação final, suíte.
- **Escopo:** `lacunas.md`.
- **Interfaces:** gate.
- **Dependências:** anteriores.
- **Testes:** cobertura ≥ 85% onde aplicável.
- **Segurança:** n/a.
- **Monitoramento:** n/a.
- **DoD:** `LCLI-02/03/05` resolvidas.