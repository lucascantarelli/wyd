# Imagem do portal Next.js (estrutura inicial de 2026-11-26).
#
# Contexto: a raiz do repositório, como as demais imagens do `wyd-ops/`; o
# `.dockerignore` da raiz mantém `node_modules`, `.next` e o cliente fora do contexto.
#
#   docker build -f wyd-ops/wyd-portal.Dockerfile -t wyd/portal:dev .

FROM node:22-alpine@sha256:1314fb30ec37592d5845e5a160d522c0e680972aa534549940b8d73bde88481a AS deps
WORKDIR /app
COPY wyd-portal/package.json wyd-portal/package-lock.json ./
RUN npm ci

FROM node:22-alpine@sha256:1314fb30ec37592d5845e5a160d522c0e680972aa534549940b8d73bde88481a AS build
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY wyd-portal/ ./
RUN npm run build

FROM node:22-alpine@sha256:1314fb30ec37592d5845e5a160d522c0e680972aa534549940b8d73bde88481a AS runtime
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
