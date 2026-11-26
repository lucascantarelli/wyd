# Imagem do `wyd-server` (esqueleto de 2026-11-26). Constrói só o binário do crate,
# sem workspace na raiz: cada crate tem o próprio `Cargo.toml` e `Cargo.lock`.
#
# Este binário sobe as sondas `/health` e `/ready`; `EPROD` (CPSock), canal e ECAP ainda
# não existem. A imagem não publica porta de jogo — `EXPOSE 7100` é só a sonda.
#
#   docker build -f wyd-ops/wyd-server.Dockerfile -t wyd/server:dev .

FROM rust:1.100-slim-bookworm AS build
WORKDIR /src
COPY crates/wyd-server/Cargo.toml crates/wyd-server/Cargo.lock ./crates/wyd-server/
COPY crates/wyd-server/src ./crates/wyd-server/src
RUN cargo build --release --locked --manifest-path crates/wyd-server/Cargo.toml

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 --no-create-home wyd
COPY --from=build /src/crates/wyd-server/target/release/wyd-server /usr/local/bin/wyd-server
USER wyd
ENV WYD_SERVER_HEALTH_BIND=0.0.0.0:7100
EXPOSE 7100
HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/wyd-server", "--health"]
ENTRYPOINT ["/usr/local/bin/wyd-server"]
