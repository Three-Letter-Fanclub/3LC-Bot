FROM rust:1.94-trixie AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release


COPY src ./src
COPY resources ./resources
RUN touch src/main.rs && cargo build --release

RUN cargo install gifski

FROM ubuntu:latest

RUN apt-get update && apt-get install -y ca-certificates
WORKDIR /app

COPY --from=builder /app/target/release/bot-3lc ./
COPY --from=builder /usr/local/cargo/bin/gifski /usr/local/bin/gifski
COPY resources ./resources

CMD ["./bot-3lc"]
