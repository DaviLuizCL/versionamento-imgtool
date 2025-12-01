# Etapa 1: build da aplicação em Rust
FROM rust:1.80 as builder

WORKDIR /app

# Copia manifests primeiro para otimizar cache
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Etapa 2: imagem final minimalista
FROM debian:bookworm-slim

WORKDIR /app

# Instala dependências básicas (se precisar no futuro)
RUN apt-get update && apt-get install -y \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Usuário não-root
RUN useradd -m appuser
USER appuser

# Copia binário
COPY --from=builder /app/target/release/img-tool /usr/local/bin/img-tool

ENTRYPOINT ["img-tool"]