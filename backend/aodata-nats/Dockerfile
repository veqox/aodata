FROM rust as builder

ENV SQLX_OFFLINE=true

WORKDIR /aodata-nats
COPY . .
RUN cargo install --path .

CMD ["aodata-nats"]