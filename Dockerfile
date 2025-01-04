FROM rust:1.83-alpine3.20
CMD cargo build --release
ADD target/release/watchman /app
RUN /app/watchman