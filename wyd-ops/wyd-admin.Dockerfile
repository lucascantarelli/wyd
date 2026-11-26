# Imagem do `wyd-admin` (esqueleto de 2026-11-26). O contrato C4, a autenticação de
# serviço e o banco não estão implementados: o binário sobe só as sondas.
#
#   docker build -f wyd-ops/wyd-admin.Dockerfile -t wyd/admin:dev .

FROM rust:1.100-slim-bookworm AS build
WORKDIR /src
COPY crates/wyd-admin/Cargo.toml crates/wyd-admin/Cargo.lock ./crates/wyd-admin/
COPY crates/wyd-admin/src ./crates/wyd-admin/src
RUN cargo build --release --locked --manifest-path crates/wyd-admin/Cargo.toml

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 --no-create-home wyd
COPY --from=build /src/crates/wyd-admin/target/release/wyd-admin /usr/local/bin/wyd-admin
USER wyd
ENV WYD_ADMIN_BIND=0.0.0.0:7600
EXPOSE 7600
HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/wyd-admin", "--health"]
ENTRYPOINT ["/usr/local/bin/wyd-admin"]
