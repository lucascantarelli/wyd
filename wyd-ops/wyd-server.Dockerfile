# Imagem do `wyd-server` (esqueleto de 2026-11-26). Constrói só o binário do crate,
# sem workspace na raiz: cada crate tem o próprio `Cargo.toml` e `Cargo.lock`.
#
# Este binário sobe as sondas `/health` e `/ready`; `EPROD` (CPSock), canal e ECAP ainda
# não existem. A imagem não publica porta de jogo — `EXPOSE 7100` é só a sonda.
#
#   docker build -f wyd-ops/wyd-server.Dockerfile -t wyd/server:dev .

FROM rust:1.100-slim-bookworm@sha256:bb875b0f8ce726053e259034848c471840e96bfe591fd52556178ae9a05e6126 AS build
WORKDIR /src
COPY crates/wyd-server/Cargo.toml crates/wyd-server/Cargo.lock ./crates/wyd-server/
COPY crates/wyd-server/src ./crates/wyd-server/src
RUN cargo build --release --locked --manifest-path crates/wyd-server/Cargo.toml

FROM debian:bookworm-slim@sha256:e67eeead736ff7bbe258ef088bff683635c561b1fcfefad0806c1df4ab49ce0e

# A base pinada por digest ainda não traz os pacotes que o Debian já publicou com
# correção; o upgrade aplica o que existe no arquivo, e o scan de CVE do CI (action
# `imagem`, Trivy, HIGH/CRITICAL com correção) exige zero.
RUN apt-get update \
    && apt-get upgrade --yes \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --uid 10001 --no-create-home wyd
COPY --from=build /src/crates/wyd-server/target/release/wyd-server /usr/local/bin/wyd-server
USER wyd
ENV WYD_SERVER_HEALTH_BIND=0.0.0.0:7100
EXPOSE 7100
HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/wyd-server", "--health"]
ENTRYPOINT ["/usr/local/bin/wyd-server"]
