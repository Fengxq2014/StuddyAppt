FROM rust:1.83
ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive
RUN ln -fs /usr/share/zoneinfo/${TZ} /etc/localtime && echo ${TZ} > /etc/timezone && dpkg-reconfigure --frontend noninteractive tzdata && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . /app/
RUN touch ~/.cargo/config && echo '[source.crates-io]\
          replace-with = 'rsproxy-sparse'\
          [source.rsproxy]\
          registry = "https://rsproxy.cn/crates.io-index"\
          [source.rsproxy-sparse]\
          registry = "sparse+https://rsproxy.cn/index/"\
          [registries.rsproxy]\
          index = "https://rsproxy.cn/crates.io-index"\
          [net]\
          git-fetch-with-cli = true' > ~/.cargo/config
RUN cargo build --release

RUN #sed -i 's/dl-cdn.alpinelinux.org/mirrors.tencent.com/g' /etc/apk/repositories
RUN #apk add tzdata && cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo Asia/Shanghai > /etc/timezone
RUN #apk add ca-certificates
#WORKDIR /app
#COPY /app/target/release/watchman /app/
CMD ["/app/target/release/StuddyAppt"]