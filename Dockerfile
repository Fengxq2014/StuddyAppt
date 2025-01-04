FROM rust:1.83-alpine3.20 as build
WORKDIR /app
COPY src /app/src
RUN cargo build --release

FROM alpine:3.20
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tencent.com/g' /etc/apk/repositories
RUN apk add tzdata && cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo Asia/Shanghai > /etc/timezone
RUN apk add ca-certificates
WORKDIR /app
COPY --from=build /app/target/release/watchman .
EXPOSE 3000
CMD ["/app/watchman"]