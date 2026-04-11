# ============================================================
# ETAPA 1: BUILD
# ============================================================
FROM rust:1.88-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Cache de dependências do workspace
COPY Cargo.toml Cargo.lock ./
COPY crates/core/Cargo.toml ./crates/core/Cargo.toml
COPY crates/api/Cargo.toml ./crates/api/Cargo.toml
COPY crates/worker/Cargo.toml ./crates/worker/Cargo.toml
COPY crates/scheduler/Cargo.toml ./crates/scheduler/Cargo.toml

# Build dummy para cache de dependências
RUN mkdir -p crates/core/src crates/api/src crates/worker/src crates/scheduler/src && \
    echo "fn main() {}" > crates/api/src/main.rs && \
    echo "fn main() {}" > crates/worker/src/main.rs && \
    echo "fn main() {}" > crates/scheduler/src/main.rs && \
    echo "pub fn dummy() {}" > crates/core/src/lib.rs

RUN cargo build --release -p chickie-api
RUN rm -rf crates target/release/.fingerprint target/release/build target/release/deps

# Build real
COPY crates ./crates
ENV CARGO_INCREMENTAL=0
RUN cargo build --release -p chickie-api

# ============================================================
# ETAPA 2: RUNTIME
# ============================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN useradd -r -u 1000 appuser

COPY --from=builder /app/target/release/chickie-api /app/chickie-api
COPY migrations /app/migrations

RUN chown -R appuser:appuser /app
USER appuser

ENV RUST_LOG=info
ENV TZ=America/Sao_Paulo

EXPOSE 3000
CMD ["/app/chickie-api"]