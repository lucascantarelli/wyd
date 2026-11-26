# ADR-OPS-0006 — Gate de documentação com fonte única

> Pergunta: "como impedir que o gate do CI e o gate local divirjam?"
> Público: Arquitetura, desenvolvimento e agentes
> Dono: `wyd-ops` (aprovado pelo dono em 2026-11-26)
> Base: `docs/planos/pipeline-ci-cd.md`; `.github/actions/docs-check/action.yml`; `docker-compose.override.yml`
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

O gate tinha duas implementações: os sete steps do composite `docs-check` e o `command` do serviço `gate` no Compose. Elas podiam divergir — CI verde e gate local vermelho, ou o contrário — sem nenhum teste detectar.

## Decisão

O comando mora em `wyd-docs/gate.sh`. O composite instala as dependências e chama o script; o Compose chama o mesmo script dentro da imagem `:gate`. A raiz de evidência é um argumento (`/app` no Compose, `$GITHUB_WORKSPACE` no CI). A imagem copia a árvore de evidência no build, em vez de montar o checkout com bind mount.

## Consequências

- Positivas: uma única ordem de verificação; erro local reproduz o CI; menos YAML duplicado.
- Negativas/custos: o script precisa assumir que `python` e `mkdocs` existem; a instalação continua no composite/imagem, não no script.
- Contratos afetados: `docs-check` deixa de replicar steps; o serviço `gate` passa a `["bash","gate.sh","/app"]` e o `.dockerignore` define o que é evidência (crates, `.github`, `.agents`, raiz) sem incluir cliente/legado.
- Medição 2026-11-26: o bind mount do Docker Desktop no Windows levava 2m20s; a cópia na imagem + cache de resolução de links levou cerca de 1m20 (duas medições: 59s e 1m25s). O CI continua nativo, em `$GITHUB_WORKSPACE`.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Manter os dois | divergência silenciosa é exatamente o defeito |
| Rodar só o Compose no CI | exige build de imagem em todo PR e mistura gate com Docker |
| Manter bind mount no Windows | I/O do Docker Desktop domina o tempo e obriga a montar a árvore inteira |
| Um Makefile como fonte | o Makefile é fachada; a lógica precisa rodar dentro da imagem e do runner |

## Como verificar

`bash wyd-docs/gate.sh` verde na raiz; `docker compose --profile gate run --rm gate` verde; `docs.yml` chama `docs-check`, que chama o script. O serviço `gate` não tem `volumes:` no `docker compose --profile gate config`.

## Referências

- Decisão do dono: Q7 do plano de DevOps (2026-11-26)
- Arquivos: `wyd-docs/gate.sh`, `.github/actions/docs-check/action.yml`, `docker-compose.override.yml`
