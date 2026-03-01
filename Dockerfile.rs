FROM lukemathwalker/cargo-chef:latest-rust-alpine3.20 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin json-schema

# We do not need the Rust toolchain to run the binary!
FROM alpine:3.20 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/json-schema /usr/local/bin
COPY --from=builder /app/schemas /app/schemas
COPY --from=builder /app/schema.json /app/schema.json
ENTRYPOINT ["/usr/local/bin/json-schema"]
