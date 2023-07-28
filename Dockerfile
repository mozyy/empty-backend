FROM rust as builder

WORKDIR /usr/src

RUN cargo new --bin empty-backend

WORKDIR /usr/src/empty-backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release

RUN rm src/*.rs

RUN apt update && apt install -y protobuf-compiler libprotobuf-dev

COPY . .

RUN cargo build -r --bin lottery

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/empty-backend/target/release/lottery /usr/local/bin/lottery
EXPOSE 3003
CMD ["lottery"]