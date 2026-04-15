# ============================================================
# ETAPA 1: BUILD
# ============================================================
FROM rust:1.91-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Cache de dependências do workspace
COPY Cargo.toml Cargo.lock ./
COPY crates/chickie_core/Cargo.toml ./crates/chickie_core/Cargo.toml
COPY crates/api/Cargo.toml ./crates/api/Cargo.toml
COPY crates/worker/Cargo.toml ./crates/worker/Cargo.toml
COPY crates/scheduler/Cargo.toml ./crates/scheduler/Cargo.toml
COPY crates/cli/Cargo.toml ./crates/cli/Cargo.toml

# Build dummy para cache de dependências
RUN mkdir -p crates/chickie_core/src crates/api/src crates/worker/src crates/scheduler/src crates/cli/src && \
    echo "fn main() {}" > crates/api/src/main.rs && \
    echo "fn main() {}" > crates/worker/src/main.rs && \
    echo "fn main() {}" > crates/scheduler/src/main.rs && \
    echo "fn main() {}" > crates/cli/src/main.rs && \
    echo "pub fn dummy() {}" > crates/chickie_core/src/lib.rs

RUN cargo build --release -p scheduler
RUN rm -rf crates target/release/.fingerprint target/release/build target/release/deps

# Build real
COPY crates ./crates
COPY scheduler.toml ./scheduler.toml
ENV CARGO_INCREMENTAL=0
RUN cargo build --release -p scheduler

# ============================================================
# ETAPA 2: RUNTIME
# ============================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libc6 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN useradd -r -u 1000 appuser

COPY --from=builder /app/target/release/scheduler /app/scheduler
COPY scheduler.toml /app/scheduler.toml

RUN chown -R appuser:appuser /app
USER appuser

ENV RUST_LOG=info
ENV TZ=America/Sao_Paulo
ENV SCHEDULER_PORT=8080

EXPOSE 8080

CMD ["/app/scheduler"]
