FROM rust as builder

ENV SQLX_OFFLINE=true

WORKDIR /aodata-nats
COPY Cargo.toml .
COPY Cargo.lock .
RUN \
    mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rvf src

COPY . .
RUN cargo build --release

FROM ubuntu:latest

ENV ENV=PROD

COPY --from=builder /aodata-nats/target/release/aodata-nats /usr/local/bin/aodata-nats

CMD ["aodata-nats"]