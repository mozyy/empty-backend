FROM rust as builder

WORKDIR /usr/src

RUN cargo new --bin empty-backend

WORKDIR /usr/src/empty-backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml.bak ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release

RUN rm src/*.rs
RUN rm Cargo.toml

RUN apt update && apt install -y protobuf-compiler libprotobuf-dev

COPY . .

RUN cargo build -r --bin blog

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/empty-backend/target/release/blog /usr/local/bin/blog
EXPOSE 3003
CMD ["blog"]