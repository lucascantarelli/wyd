---
name: revisar-pagina-publicada
description: Revisa o site montado do WYD — link quebrado, página órfã, navegação, censo e contagem — depois do build do MkDocs. Use ao fechar mudança que toca docs/, ao investigar gate-docs vermelho, ou antes de publicar release do site.
---

# Revisar página publicada

## Quando usar

- PR que altera `planejamento-1.0.0/*/docs/`, `wyd-docs/` ou `SISTEMAS`.
- `gate-docs` vermelho no CI.

## Pré-requisitos

- Build local disponível (`make gate` usa a imagem `wyd-docs:gate`).

## Passos

1. **Link**: rode `links` pelo gate completo; link relativo tem de resolver dentro do site, não só no
   repositório.
2. **Órfã**: toda página de `docs/` alcançável pelo mapa de `index.md` da pasta e pelo índice.
3. **Navegação**: ordem e título conferem com o sistema; página nova não aparece no fim do site por
   falta de mapa.
4. **Censo e contagem**: número de páginas do site no `contagens` bate com o dashboard dono; se
   divergir, corrija o dono do número, não a citação.
5. **Publicação**: `develop` publica no Pages; release é ato do dono, não do build.

## Como verificar

```bash
cd wyd-docs && py -3 -m gate full --details
```

Saída esperada: `gate: PASS (full, …)` com `build`, `links`, `contagens` e `censos` PASS.

## Quando parar e perguntar

- O número medido diverge do declarado no dashboard (é o dono que muda, não a página que cita).
- A correção exige mexer em `SISTEMAS` ou no tema.

## Armadilhas

- **Consertar o link no HTML gerado** em vez de na fonte.
- **Apagar página órfã** em vez de decidir se ela deve existir.
- **Tratar `gate-fast` como prova de build** — ele não monta o site.
