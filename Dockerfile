FROM rust:1.83
ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive
RUN ln -fs /usr/share/zoneinfo/${TZ} /etc/localtime && echo ${TZ} > /etc/timezone && dpkg-reconfigure --frontend noninteractive tzdata && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . /app/
COPY config.toml $CARGO_HOME/config.toml
RUN cargo build --release

RUN #sed -i 's/dl-cdn.alpinelinux.org/mirrors.tencent.com/g' /etc/apk/repositories
RUN #apk add tzdata && cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo Asia/Shanghai > /etc/timezone
RUN #apk add ca-certificates
#WORKDIR /app
#COPY /app/target/release/watchman /app/
CMD ["/app/target/release/StuddyAppt"]