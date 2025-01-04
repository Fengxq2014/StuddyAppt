FROM rust:1.83-alpine3.20
RUN cargo build --release
RUN mkdir "/app"
COPY target/release/watchman /app/
CMD ["/app/watchman"]