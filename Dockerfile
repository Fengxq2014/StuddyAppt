FROM rust:1.83
WORKDIR /app
COPY . /app/
RUN cargo build --release

RUN #sed -i 's/dl-cdn.alpinelinux.org/mirrors.tencent.com/g' /etc/apk/repositories
RUN #apk add tzdata && cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo Asia/Shanghai > /etc/timezone
RUN #apk add ca-certificates
#WORKDIR /app
#COPY /app/target/release/watchman /app/
CMD ["/app/target/release/watchman"]