# ============================================================
# ETAPA 1: BUILD
# ============================================================
FROM rust:1.88-bookworm AS builder

WORKDIR /app

# Dependências de build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache de dependências (só refaz se Cargo.toml mudar)
COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY database.secrets.env ./database.secrets.env

# Build "fake" para baixar deps
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Build real (só refaz se src mudar)
RUN rm -rf src
COPY src ./src
ENV CARGO_INCREMENTAL=0
RUN cargo build --release

# ============================================================
# ETAPA 2: RUNTIME
# ============================================================
FROM debian:bookworm-slim AS runtime

# ⚠️ CRÍTICO: Instalar libs que o Rust precisa para não crashar silenciosamente
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Criar usuário não-root
RUN useradd -r -u 1000 appuser

# Copiar apenas o binário e migrations
COPY --from=builder /app/target/release/chickie /app/chickie
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/database.secrets.env /app/database.secrets.env

# Permissões
RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 3000

# Entry point
CMD ["/app/chickie"]