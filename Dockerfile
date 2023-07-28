FROM rust:1.71 as builder

WORKDIR /usr/src

RUN apt update && apt install -y protobuf-compiler libprotobuf-dev

RUN cargo new --bin empty-backend

WORKDIR /usr/src/empty-backend

COPY .cargo/config.toml.bak .cargo/config.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml.bak ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo check

RUN rm src/*.rs

COPY . .

RUN cargo build -r --bin lottery


FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/empty-backend/target/release/lottery /usr/local/bin/lottery
EXPOSE 3003
CMD ["lottery"]