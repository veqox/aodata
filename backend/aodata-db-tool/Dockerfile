FROM rust as builder

ENV SQLX_OFFLINE=true

WORKDIR /aodata-db-tool
COPY . .
COPY .env.prod .env
RUN cargo install --path .

FROM ubuntu:latest

COPY --from=builder /usr/local/cargo/bin/aodata-db-tool /usr/local/bin/aodata-db-tool

CMD ["aodata-db-tool"]