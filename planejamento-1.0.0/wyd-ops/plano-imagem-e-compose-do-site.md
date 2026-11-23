# Imagem do site e `docker compose` do projeto — plano e decisão

> Pergunta: "quem constrói a imagem do site, quem a publica e como o Compose a consome?"
> Público: quem opera, arquitetura e quem edita o site
> Dono: frente `wyd-ops`
> Base: `plano-ci-real.md` §3 (runtime é Compose) e §4.1 (só `run:` e action própria); `wyd-docs-1.0.0.md` §9.1
> Revisado em: 2026-11-22

**Estado:** decidido pelo dono em **2026-11-22** (pedido de parar de reconstruir o MkDocs à mão e ter
o Compose consumindo a imagem). A implementação acompanha esta página: `wyd-ops/wyd-docs.Dockerfile`,
`docker-compose.yml`, `.dockerignore` e `.github/workflows/docs-image.yml`.

## 1. Problema

O site era construído por comando manual em container a cada verificação, e o repositório não tinha
Compose nenhum. Duas consequências: o build dependia da máquina de quem opera, e não existia artefato
reproduzível — `docker compose up` não subia nada.

## 2. Decisão

1. **O CI constrói a imagem, não a estação.** Um workflow dedicado (`docs-image`) roda o mesmo build
   do gate e publica a imagem.
2. **O registro é o GHCR**, imagem `ghcr.io/lucascantarelli/wyd-docs`, com duas marcas: `latest` e o
   SHA do commit.
3. **O Compose consome a imagem publicada.** O serviço `docs` referencia a imagem e mantém `build:`
   como fallback local — quem não tem a imagem constrói, quem tem faz `pull`.
4. **Sem action de terceiro.** Login e push usam `docker login` e `docker push` em `run:` com
   `GITHUB_TOKEN`, conforme `plano-ci-real.md` §4.1. Só `actions/checkout` entra, fixado por SHA.
5. **O servidor é o do Python**, `http.server`, sem nginx: o site é estático e não precisa de mais
   uma peça para manter.

## 3. O que esta decisão **não** faz

| Fora | Dono |
|---|---|
| publicar o site em host de produção | deploy real é decisão separada (`pg-lake`, `plano-ci-real.md` §3) |
| servir o portal Next.js pelo Compose | `wyd-portal` |
| publicar o site no GitHub Pages | `docs.yml`, inalterado por esta entrega |
| fixar `latest` por digest no Compose | fica `WYD_DOCS_TAG` como override; digest é próximo passo se houver necessidade |

## 4. Arquivos e responsabilidade

| Arquivo | Responsabilidade |
|---|---|
| `wyd-ops/wyd-docs.Dockerfile` | build em dois estágios: dependências, `preparar.py`, `mkdocs build`; runtime só com `site/` e `python -m http.server` |
| `docker-compose.yml` | serviço `docs`, porta `${WYD_DOCS_PORT:-8000}`, imagem `${WYD_DOCS_TAG:-latest}` com `build:` de fallback |
| `.dockerignore` | mantém fora do contexto `.git`, `deprecated/`, `wyd-client/` (567 MB), `**/site`, `**/.build`, `.venv`, `.env` |
| `.github/workflows/docs-image.yml` | build, fumaça HTTP 200 e push no GHCR (push só fora de PR) |

O **dono do layout** é `planejamento-1.0.0/wyd-1.0.0.md` §8: ele diz, para cada tipo de artefato,
onde mora, qual sistema responde e por que três arquivos ficam na raiz (Compose, `.dockerignore` e
`.github/workflows/`). Esta página não repete a tabela — só lista os arquivos desta entrega. Nenhum
deles pode ser copiado para `planejamento-1.0.0/wyd-ops/`: aquela árvore é documentação
(AGENTS.md §5.6).

## 5. Como verificar (DoD)

```bash
# 1. build igual ao do CI, na máquina
MSYS_NO_PATHCONV=1 docker build -f wyd-ops/wyd-docs.Dockerfile -t ghcr.io/lucascantarelli/wyd-docs:local .

# 2. sobe pelo Compose e responde
MSYS_NO_PATHCONV=1 docker compose up -d docs
curl -fsS -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8000/

# 3. o site servido é o do build, com a contagem do gate
curl -fsS http://127.0.0.1:8000/ | grep -o '<title>[^<]*</title>'
```

DoD do workflow: a primeira execução em `develop` termina verde, empurra `:<sha>` e `:latest`, e o
`docker compose pull docs` baixa a imagem sem construir.

## 6. Risco

| Risco | Efeito | Mitigação |
|---|---|---|
| imagem grande por copiar o repositório no estágio de build | pull lento | estágio de runtime recebe só `site/`; `.dockerignore` corta cliente e legado |
| `latest` mudando sozinho | site servido diverge do commit | `WYD_DOCS_TAG:<sha>` no ambiente que precisa de reprodutibilidade |
| pacote GHCR privado | `pull` sem `docker login` falha | imagem de documentação pública; se virar privada, o host precisa de credencial |
| workflow duplicar o gate | dois builds do mesmo commit | o gate continua em `docs.yml`; a imagem não reexecuta `verificar.py` |
