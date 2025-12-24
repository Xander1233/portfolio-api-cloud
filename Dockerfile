# ---- builder ----
FROM rust:1.89.0-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ---- runtime ----
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/portfolio-api-cloud /app/portfolio-api-cloud

LABEL org.opencontainers.image.source=https://github.com/xander1233/portfolio-api-cloud
LABEL org.opencontainers.image.description="API for the cloud deployment of my portfolio"
LABEL org.opencontainers.image.licenses=MIT

ENV APP_ENVIRONMENT=production
EXPOSE 8080
CMD ["/app/portfolio-api-cloud"]