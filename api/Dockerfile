FROM nixos/nix:2.29.0 AS builder

WORKDIR /workspace

COPY flake.nix rust-toolchain.toml ./
COPY api ./api

RUN nix --extra-experimental-features 'nix-command flakes' \
    develop --accept-flake-config --command \
    cargo build --release --target x86_64-unknown-linux-musl --manifest-path api/Cargo.toml

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /workspace/api/target/x86_64-unknown-linux-musl/release/service-status-api /usr/local/bin/service-status-api

EXPOSE 3000

ENV BIND_ADDR=0.0.0.0:3000
ENV RUST_LOG=info

CMD ["/usr/local/bin/service-status-api"]
