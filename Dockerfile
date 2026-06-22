# ── build stage ──
FROM rust:1.89-alpine AS builder

ENV ARS_ENV=prod

RUN apk add --no-cache \
	musl-dev \
	perl \
	make \
	openssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY migrations/ migrations/
COPY diesel.toml ./

RUN cargo build --release --bin server

RUN rm -rf /var/cache/apk/*

COPY migrations/ /migrations/
COPY diesel.toml /diesel.toml
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh

RUN chmod +x /usr/local/bin/docker-entrypoint.sh

ARG DATABASE_URL
ENV PORT=7357

EXPOSE 7357

ENTRYPOINT ["docker-entrypoint.sh"]
