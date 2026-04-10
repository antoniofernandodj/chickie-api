# ============================================================
# ETAPA 1: BUILD
# ============================================================
FROM rust:1.91-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Cache de dependências
COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/worker.rs && \
    echo "fn main() {}" > src/scheduler.rs

RUN cargo build --release --bin chickie-scheduler
RUN rm -rf src target/release/chickie-scheduler target/release/chickie-scheduler.d

# Build real
COPY src ./src
COPY scheduler.toml ./scheduler.toml
ENV CARGO_INCREMENTAL=0
RUN cargo build --release --bin chickie-scheduler

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

COPY --from=builder /app/target/release/chickie-scheduler /app/chickie-scheduler
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/scheduler.toml /app/scheduler.toml

RUN chown -R appuser:appuser /app
USER appuser

ENV RUST_LOG=info
ENV TZ=America/Sao_Paulo

CMD ["/app/chickie-scheduler"]
