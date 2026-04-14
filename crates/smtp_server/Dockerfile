# ============================================================
# Estágio 1: Build
# ============================================================
FROM rust:1.82-slim AS builder

WORKDIR /app

# Instala dependências de sistema para compilação
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Copia manifesto e faz cache das dependências
COPY Cargo.toml Cargo.lock ./

# Cria um main.rs placeholder para cachear a compilação das dependências
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# Copia o código-fonte real
COPY src ./src

# Compila o binário final
RUN cargo build --release

# ============================================================
# Estágio 2: Runtime
# ============================================================
FROM ubuntu:24.04

# Instala apenas o necessário em runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Cria usuário não-root para segurança
RUN useradd -r -u 1001 -g root smtp-server

WORKDIR /app

# Copia o binário compilado
COPY --from=builder /app/target/release/smtp-server /app/smtp-server

# Cria diretório para certificados
RUN mkdir -p /app/certs && chown -R smtp-server:root /app

# Gera certificados auto-assinados para desenvolvimento
RUN openssl req -x509 -newkey rsa:4096 \
    -keyout /app/certs/key.pem \
    -out /app/certs/cert.pem \
    -days 365 -nodes \
    -subj "/C=BR/ST=RJ/L=Rio de Janeiro/O=smtp-server/CN=localhost" \
    && chown smtp-server:root /app/certs/*.pem \
    && chmod 600 /app/certs/key.pem

USER smtp-server

# Expõe as portas SMTP
# Nota: portas < 1024 requerem root ou capabilities no Linux
EXPOSE 2525 4465 2587

# Variáveis de ambiente padrão
ENV RUST_LOG=smtp_server=info,info
ENV SMTP_HOSTNAME=localhost
ENV SMTP_PORT=2525
ENV SMTPS_PORT=4465
ENV SMTP_SUBMISSION_PORT=2587
ENV TLS_CERT_PATH=/app/certs/cert.pem
ENV TLS_KEY_PATH=/app/certs/key.pem
ENV REQUIRE_AUTH=true

CMD ["/app/smtp-server"]
