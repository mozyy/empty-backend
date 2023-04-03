FROM alpine:latest
# RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY target/release/empty-blog .

EXPOSE 3003

CMD ["./empty-blog"]