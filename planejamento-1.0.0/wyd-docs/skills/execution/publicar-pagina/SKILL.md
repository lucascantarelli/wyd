---
name: publicar-pagina
description: Publica ou move uma página do planejamento no site MkDocs do WYD, registrando-a no mapa da pasta, no índice e na navegação gerada. Use ao criar página em docs/, mover pasta de sistema ou corrigir página órfã apontada pelo gate.
---

# Publicar página

## Quando usar

- Página nova em `planejamento-1.0.0/<sistema>/docs/`.
- Página movida/renomeada e o gate acusa órfã.

## Pré-requisitos

- Página com cabeçalho de 5 linhas e `**Estado:**` (molde de `wyd-db/docs/convencoes.md`).
- `wyd-docs/preparar.py` e `wyd-docs/docs/convencoes.md` lidos.

## Passos

1. Confirme que a página é de **`docs/`**: `migrations/`, `skills/`, `tests/`, `prompts/` e `src/`
   não são publicados (`D-6`).
2. Inclua a página no **Mapa de páginas** de `docs/index.md` do sistema (ou da pasta) — página fora do
   mapa é órfã.
3. Sistema novo só entra publicando se estiver em `SISTEMAS` de `preparar.py`; inclusão ali é decisão
   de arquitetura, não de documentação.
4. Rode o build: `make gate` (ou `make gate-fast` no ciclo curto).
5. Confira a navegação gerada: a página aparece sob o sistema certo, com título legível.

## Como verificar

```bash
cd wyd-docs && py -3 -m gate fast --details
cd wyd-docs && py -3 -m gate full --details   # build + links
```

Gate verde prova build e link; ele **não** prova que o texto está correto.

## Quando parar e perguntar

- A publicação exige mudar `SISTEMAS` (sistema novo ou renomeado).
- A página precisa de conteúdo que o dono do sistema não aceitou.

## Armadilhas

- **Link relativo que só funciona no GitHub** e quebra no MkDocs.
- **Página em `docs/` sem entrada no mapa** — órfã silenciosa até o gate.
- **Copiar página para dois sistemas**: um fato, um dono.
