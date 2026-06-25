# STAGE 1: Compute dependency recipe
# -------------------------------
FROM rust:1.96-bullseye AS planner

WORKDIR /app
COPY . .
RUN cargo install cargo-chef
RUN cargo chef prepare --recipe-path recipe.json

# STAGE 2: Build dependencies and API binary
# -------------------------------
FROM rust:1.96-bullseye AS builder

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo install cargo-chef
# Build and cache dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Copy actual source code and compile the real binary
COPY . .
RUN cargo build --release --bin server

# STAGE 3: Minimal runtime image
# -------------------------------
FROM debian:bullseye-slim AS runtime

ARG ARS_ENV \
   DATABASE_URL
ENV PORT=7357 \
    RUST_LOG=debug

WORKDIR /app
COPY --from=builder /app/target/release/server /app/ars_server

EXPOSE $PORT

CMD ["./ars_server"]
