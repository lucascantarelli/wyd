---
name: revisar-documentacao
description: Revisa uma página do planejamento do wyd-db contra cabeçalho, marcação de evidência, dono do fato e lista do que reprova. Use antes de aceitar página nova, depois de editar migration ou dicionário, e quando o gate de documentação apontar link, contagem ou dono duplicado.
---

# Revisar documentação

## Quando usar

- Página nova em `planejamento-1.0.0/wyd-db/` (migration, dicionário, arquitetura, lacuna).
- Página editada por mudança de DDL, contagem ou decisão.

Não use para revisar SQL: isso é `review/revisar-schema`.

## Pré-requisitos

- `planejamento-1.0.0/wyd-db/docs/convencoes.md` aberto (cabeçalho, marcas, o que reprova).
- O dono do fato que a página descreve (DDL, `plano-migrations.md`, `lacunas.md`).

## Passos

1. **Cabeçalho de 5 linhas**: `> Pergunta:`, `> Público:`, `> Dono:`, `> Base:`, `> Revisado em:`.
   `Base:` aponta o bloco de análise; página feita só a partir de comando usa `—`.
2. **Linha de estado** logo depois do cabeçalho: `**Estado:** <estado> (AAAA-MM-DD)`.
3. **Toda afirmação tem marca**: `V` com comando/arquivo e data, `H` com ID da lacuna que a fecha, ou
   `P` de planejado. Presente do indicativo sem `V` reprova.
4. **Um fato, um dono**: número, DDL e regra moram em um arquivo; as outras páginas citam a seção.
   Contagem repetida em duas páginas é defeito por construção.
5. **Seções obrigatórias do molde** presentes (para migration: identificação até "o que não decide").
6. **Links resolvem** e apontam para o caminho real (`planejamento-1.0.0/wyd-db/...`).
7. **Marcador de arquivo ausente**: página que cita arquivo não versionado usa o bloco
   `**Sem arquivo no repositório:**` do molde, não link quebrado.

## Como verificar

```bash
make gate-fast          # layout, skills, lacunas, contagens e censos
cd wyd-docs && py -3 -m gate fast --details
```

O `--details` aponta arquivo e linha do que reprovou.

## Quando parar e perguntar

- A página precisa declarar decisão que o dono não tomou: escreva como **proposta** com alternativas.
- O número medido diverge do declarado no dashboard: corrija o dono do número, não a página que cita.
- A revisão pede reescrita de mais de uma página: uma issue por página, sem refatoração em lote.

## Armadilhas

- **Aceitar página sem `Base:`** ou com `Revisado em:` antigo depois de editar.
- **Afirmação nova sem marca** — o erro mais comum e o que o gate não pega sozinho.
- **Copiar tabela de outro documento** para "facilitar a leitura".
- **Publicar `migrations/`, `skills/` ou `tests/` no site**: só `docs/` vai para o MkDocs (`D-6`).
