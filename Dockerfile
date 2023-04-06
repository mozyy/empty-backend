FROM rust as builder

WORKDIR /usr/src

COPY . .

RUN cargo install --path empty-blog

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/empty-blog /usr/local/bin/empty-blog
EXPOSE 3003
CMD ["empty-blog"]