> Pergunta: "como o `wyd-docs` prova montagem, gate e publicação sem inventar página?"
> Público: QA e desenvolvedor do `wyd-docs`
> Dono: equipe do wyd-docs
> Base: `AGENTS.md` §8; `wyd-docs-1.0.0.md` §7; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `preparar`, `verificar` | montagem, link/âncora | funções puras |
| Contrato | `tests/contrato/` | contagens, censos | montagem real |
| Integração | `tests/integracao/` | build completo | container Linux |
| Sistema | `tests/sistema/` | publicação, rollback | GitHub Pages |
| E2E | (n/a) | — | — |
| Propriedade | (n/a) | — | — |

## O que cada teste precisa provar

1. **Montagem:** preserva caminho relativo; não publica conteúdo interno.
2. **Gate:** link quebrado, âncora inexistente, nav ausente, título ausente, redirect inválido, sintaxe 1.x reprova.
3. **Contagens:** soma divergente reprova.
4. **Censos:** citação divergente reprova.
5. **Build:** roda em Linux com Python 3.15; MkDocs 2.0.dev6.
6. **Publicação:** workflow com permissões mínimas; rollback validado.
7. **Redirect:** origem duplicada reprova; destino inexistente reprova.

## Isolamento

- Testes Python com `pytest` ou execução direta; container Linux por build.
- Fixtures por montagem real; nenhum número inventado.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% por capacidade (onde aplicável).

## Comandos

```bash
python preparar.py && mkdocs build --dir .build/docs --output site && python verificar.py site && python verificar-contagens.py && python verificar-censos.py
```

## CI, cobertura e monitoramento

- **CI:** `docker compose --profile gate run --rm gate` e o job de documentação de `.github/workflows/docs.yml` rodam build, links, âncoras, contagem e censo; falha em qualquer um reprova o PR.
- **Cobertura:** A cobertura é de página, link, âncora e censo: as 471 páginas e as 469 fontes precisam fechar; não há percentual de linha para o site.
- **Monitoramento:** Duração do build, páginas publicadas, fontes fora do censo, link quebrado e status da publicação. O gate já falha em link, âncora, contagem e censo; não há SLO de disponibilidade antes de existir deploy.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.

## Critério de saída da `1.0.0`

Build e verificação verdes no CI; publicação automática em `develop`; URL estável; nenhum conteúdo interno no artefato; rollback validado. `LPDOC-01` fechada. `LPDOC-12` (Windows) segue aberto como decisão.