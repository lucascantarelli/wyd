# Imagem do site de planejamento (MkDocs 2.0).
#
# Mora no `wyd-ops/` porque o `wyd-ops` é o dono das imagens (CI, imagem, deploy).
# Construída pelo CI em .github/workflows/docs-image.yml e consumida por docker-compose.yml.
# Dono da decisão: planejamento-1.0.0/wyd-ops/plano-imagem-e-compose-do-site.md
#
# Contexto de build é a raiz do repositório: o `preparar.py` lê `planejamento-1.0.0/`.
# Use a raiz como diretório de trabalho, não o diretório deste arquivo.
#
# Uso local (mesmo comando do CI):
#   docker build -f wyd-ops/wyd-docs.Dockerfile -t ghcr.io/lucascantarelli/wyd-docs:local .

# --- estágio 1: build -------------------------------------------------------
FROM python:3.15-slim AS build

WORKDIR /app

# Camada de dependência isolada: só é refeita quando requirements.txt muda.
COPY wyd-docs/requirements.txt wyd-docs/requirements.txt
RUN pip install --no-cache-dir --disable-pip-version-check -r wyd-docs/requirements.txt

# Fontes: `preparar.py` lê `planejamento-1.0.0/<sistema>/docs/` e o tema de `wyd-docs/tema/`.
COPY planejamento-1.0.0 planejamento-1.0.0
COPY wyd-docs wyd-docs

WORKDIR /app/wyd-docs

# Se o MkDocs reprovar o build, a imagem não é produzida.
RUN python preparar.py \
 && mkdocs build --dir .build/docs --output site \
 && rm -rf .build __pycache__

# --- estágio 2: runtime -----------------------------------------------------
# Só o site estático, sem pip, sem fontes e sem cache de build.
FROM python:3.15-slim

RUN useradd --system --uid 10001 --no-create-home site

COPY --from=build /app/wyd-docs/site /site

USER site
EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD python -c "import sys,urllib.request; sys.exit(0 if urllib.request.urlopen('http://127.0.0.1:8000/', timeout=2).status == 200 else 1)"

CMD ["python", "-m", "http.server", "8000", "--bind", "0.0.0.0", "--directory", "/site"]
