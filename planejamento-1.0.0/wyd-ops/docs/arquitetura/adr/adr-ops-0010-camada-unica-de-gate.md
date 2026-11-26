# ADR-OPS-0010 — Camada única de gate: pacote Python, contrato de saída e papéis das linguagens

> Pergunta: "como o gate deixa de ser uma coleção de scripts e passa a ter um contrato único, rápido e legível pelo agente?"
> Público: Arquitetura, desenvolvimento, operação e agentes
> Dono: `wyd-ops` (decisão solicitada pelo dono em 2026-11-26)
> Base: `docs/devops/index.md`; `wyd-docs/gate/`; `Makefile`; `AGENTS.md`; ADR-OPS-0005 e ADR-OPS-0006
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

O gate funcionava, mas era caro de operar e de ler: seis scripts Python independentes
(`verificar.py`, `verificar-lacunas.py`, `verificar-contagens.py`, `verificar-censos.py`,
`verificar-skills.py` e `preparar.py`), um `gate.sh` que apenas encadeava comandos e saídas de
centenas de linhas. Cada verificador varria a mesma árvore e repetia leituras; no Windows o bind
mount do Docker Desktop levava 2m20s. O agente não tinha um ponto único para escolher entre o ciclo
curto e o gate completo, nem uma saída estruturada.

O launcher é um produto Windows-only (EXE + Tauri/NSIS), enquanto `wyd-db` e `wyd-ops-signer` são
portáveis; a matriz antiga executava testes do launcher em Linux.

## Decisão

1. **Uma porta de entrada estável:** `wyd-docs/gate.sh` continua sendo o comando que o CI e o
   Compose chamam, mas agora delega a `python -m gate` (pacote `wyd-docs/gate/`). Nenhuma regra
   editorial vive no shell.
2. **Três modos explícitos:**
   - `full`: monta o site, roda `links` e todos os domínios;
   - `fast`: roda `skills`, `lacunas`, `contagens` e `censos` nativamente, sem MkDocs/links;
   - `check <domínio>`: roda um domínio isolado.
3. **Contrato de saída único:** sem opção, uma linha por domínio com PASS/FAIL, duração e resumo
   curto; `--json` devolve o mesmo contrato (`ok`, `mode`, `duration_ms`, `domains[]`); `--details`
   acrescenta problemas e linhas de evidência. Falha sempre retorna exit 1; erro de montagem/leitura
   que impede medir retorna exit 2.
4. **Índice compartilhado:** `gate.workspace.Workspace` mantém a raiz, um índice de nomes de arquivo
   e caches de texto/listas. Os checks recebem a mesma instância; nenhuma regra muda por causa disso.
5. **Papéis por linguagem, não uma linguagem só:**
   - **Python**: parsing e medição de Markdown/HTML, onde estão as regras e as fixtures;
   - **Bash**: entrada estável, variáveis de ambiente e comandos PostgreSQL; sem lógica de regra;
   - **Node**: API do GitHub (`sync-issue-status.mjs`), onde o SDK/API é natural; não faz parsing de
     documentação.
6. **Local vs CI:** `fast` é nativo e não exige Docker; `full` usa a imagem `:gate` no desenvolvimento
   porque o MkDocs 2.0.dev6 usa `PosixPath` e não roda no Windows. O CI roda `full` nativamente em
   Linux. O Docker não é removido: ele é o ambiente reprodutível do build/links fora do Linux.

## Consequências

- Positivas: um único ponto de descoberta; saída curta e JSON para agente; uma árvore lida uma vez;
  testes dos verificadores no CI; matriz do launcher coerente com o produto; `fast` em segundos.
- Negativas/custos: o pacote novo precisa ser mantido e testado; `python -m gate` exige que o
  diretório `wyd-docs/` esteja no `sys.path` (o `gate.sh` e o Makefile já garantem isso); a imagem
  `:gate` copia a árvore de evidência, então o `.dockerignore` é parte do contrato.
- Contratos afetados: `gate.sh`, `Makefile`, `.github/workflows/docs.yml`, `devops/index.md`,
  `AGENTS.md` e este índice.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Manter seis scripts com um wrapper de resumo | Não elimina leituras repetidas nem caminhos divergentes; o agente continua com seis comandos |
| Reescrever tudo em uma linguagem só | Bash/Node têm papéis específicos; forçar Python no GitHub API ou Node no parsing piora a solução |
| Rodar só Docker no CI | Exige build de imagem em todo PR e mistura gate com infraestrutura |
| Rodar o MkDocs nativo no Windows | O `mkdocs==2.0.dev6` falha com `PosixPath`; o container é a execução suportada |
| Um `Makefile` como fonte da regra | O Makefile é fachada; a regra precisa rodar no runner e na imagem |

## Como verificar

```bash
make gate-fast                         # sem Docker/MkDocs, exit 0
cd wyd-docs && python -m gate fast --json
make gate                              # container, exit 0
cd wyd-docs && python -m unittest discover -s tests -v
MSYS_NO_PATHCONV=1 docker run --rm -v "$(pwd -W):/repo:ro" -w /repo rhysd/actionlint:latest .github/workflows/*.yml
```

O `--json` deve ter `ok`, `mode` e um item por domínio com `duration_ms`; `make gate` deve terminar
com `gate: PASS (full, …)`; os testes de fixture devem cobrir link, lacuna, contagem, censo e skills.

## Referências

- Pedido do dono: padronizar scripts/verificação de ponta a ponta (2026-11-26)
- Arquivos: `wyd-docs/gate/`, `wyd-docs/gate.sh`, `Makefile`, `.github/workflows/docs.yml`
- ADRs relacionados: `ADR-OPS-0005`, `ADR-OPS-0006`
