FROM lukemathwalker/cargo-chef:0.1.62-rust-1-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder

RUN apt update && apt install -y protobuf-compiler libprotobuf-dev pkg-config
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build -r --bin app

# We do not need the Rust toolchain to run the binary!
FROM debian:12.1-slim AS runtime
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/app /usr/local/bin
EXPOSE 50051
CMD ["app"]
