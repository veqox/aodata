FROM rust as builder

ENV SQLX_OFFLINE=true

WORKDIR /aodata-nats-histories
COPY . .
COPY .env.prod .env
RUN cargo install --path .

FROM ubuntu:latest

COPY --from=builder /usr/local/cargo/bin/aodata-nats-histories /usr/local/bin/aodata-nats-histories

CMD ["aodata-nats-histories"]