FROM rust:1-bookworm AS builder
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

ENV APP_HOST=0.0.0.0
ENV APP_PORT=3000
ENV ENABLE_SWAGGER=false

COPY --from=builder /app/target/release/nelf /usr/local/bin/nelf

EXPOSE 3000

CMD ["nelf"]
