# wyd-docs — da fase 3 para a fase 4

> **Estado: proposta de planejamento (`P`, 2026-11-19).** Este arquivo guarda **só** o critério de
> saída da fase 4 e o que falta para ela fechar. A ordem de produção — fases, escopo e estado — é do
> plano diretor (`wyd-docs-1.0.0.md` §9); a fila de trabalho — ordem, custo, arquivos e bloqueios —
> mora em **um único lugar**: [`proximos-passos.md`](proximos-passos.md).
>
> **Dono:** o critério de saída da fase 4 é deste arquivo; a ordem de produção é de
> `wyd-docs-1.0.0.md` §9; a ordem dos passos é de `proximos-passos.md`; o estado de lacuna é de
> `lacunas.md`. Este arquivo não repete nenhum dos quatro.
>
> **Publicação:** este arquivo não entra no site (`preparar.py` só monta `**/docs/**`), então o
> `docs/index.md` publicado apenas aponta para cá, sem repetir fase nem critério.
>
> **Referência, não duplicação:** a tabela de fases, com escopo e estado de cada uma, vive em
> `wyd-docs-1.0.0.md` §9. Mudou a fase ou o escopo, muda lá; aqui só se aponta.

---

## 1. Fase

A ordem de produção — fases, escopo e estado de cada uma — é de `wyd-docs-1.0.0.md` §9; este mapa
não a reproduz. O que ele detalha é o critério de saída da fase 4, em §2.

## 2. Critério de saída da fase 4

A fase só fecha quando os quatro itens do gate de `wyd-docs-1.0.0.md` §9 valerem juntos:

1. **redirect decidido e implementado** — `LPDOC-06` fechada em 2026-11-19: mapa
   `wyd-docs/redirects.tsv`, stub no `preparar.py` e checagens no `verificar.py`;
2. **busca decidida** — `LPDOC-13` fechada em 2026-11-19: sem busca de texto completo na `1.0.0`; o
   filtro de navegação é a navegação (`lacunas.md` §2.4);
3. **métrica escrita e aceita** — qual número o site mede, onde e com que comando, com o mesmo rigor de
   evidência do resto do plano — escrita e **aceita em 2026-11-19** em `docs/planos/metricas.md` pelo dono de `documentação`;
4. **gate verde** — build e verificação no CI, publicação automática em `develop`, URL estável
   declarada e nenhum conteúdo interno no artefato.

## 3. O que este arquivo não repete

- **A fila não está aqui.** Itens, ordem por desbloqueio ÷ custo, tempo, arquivos tocados e o que
  cada passo destrava estão em [`proximos-passos.md`](proximos-passos.md). Se a fila mudar,
  só ela muda.
- **A contagem não está aqui.** Páginas montadas e censo por estado são de `wyd-docs-1.0.0.md` §9.1.
- **O estado das lacunas não está aqui.** Dono, marco, estado e evidência de cada uma estão em
  `lacunas.md`.

Até 2026-11-19 este arquivo também listava os passos e as decisões do usuário. Isso virou fila em
`proximos-passos.md`: duas listas da mesma coisa divergem na primeira edição.
