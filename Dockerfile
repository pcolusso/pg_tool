FROM rust:1.31 AS builder

WORKDIR /src/app
COPY . .
RUN cargo install --path .
RUN cargo build --release

FROM debian:stretch-slim

WORKDIR /app

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update \
    && mkdir -p /usr/share/man/man1 \
    && apt-get install -y --no-install-recommends postgresql-client \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/app/target/release/pg_tool /app
CMD ['/app/pg_tool']