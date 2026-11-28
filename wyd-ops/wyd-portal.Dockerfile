# Imagem do portal Next.js (estrutura inicial de 2026-11-26).
#
# Contexto: a raiz do repositório, como as demais imagens do `wyd-ops/`; o
# `.dockerignore` da raiz mantém `node_modules`, `.next` e o cliente fora do contexto.
#
#   docker build -f wyd-ops/wyd-portal.Dockerfile -t wyd/portal:dev .

FROM node:26-alpine@sha256:6bd9ab43c0ec0190ca7b2d193906cb8b7edf6e7f4afa43ddeccd6452710bffb5 AS deps
WORKDIR /app
COPY wyd-portal/package.json wyd-portal/package-lock.json ./
RUN npm ci

FROM node:26-alpine@sha256:6bd9ab43c0ec0190ca7b2d193906cb8b7edf6e7f4afa43ddeccd6452710bffb5 AS build
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY wyd-portal/ ./
RUN npm run build

FROM node:26-alpine@sha256:6bd9ab43c0ec0190ca7b2d193906cb8b7edf6e7f4afa43ddeccd6452710bffb5 AS runtime
WORKDIR /app
ENV NODE_ENV=production \
    PORT=3000 \
    HOSTNAME=0.0.0.0
RUN addgroup -S wyd && adduser -S wyd -G wyd
COPY --from=build --chown=wyd:wyd /app/.next/standalone ./
COPY --from=build --chown=wyd:wyd /app/.next/static ./.next/static
COPY --from=build --chown=wyd:wyd /app/public ./public
USER wyd
EXPOSE 3000
HEALTHCHECK --interval=10s --timeout=3s --start-period=15s --retries=3 \
    CMD ["node", "-e", "fetch('http://127.0.0.1:3000/').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"]
CMD ["node", "server.js"]
