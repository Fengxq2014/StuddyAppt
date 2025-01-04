FROM rust:1.83-alpine3.20
WORKDIR /app
RUN cargo build --release
COPY target/release/watchman /app/
CMD ["/app/watchman"]