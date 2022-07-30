FROM rust as builder

RUN USER=root
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt install -y cmake \
    && apt install -y protobuf-compiler

COPY . /app
WORKDIR /app
RUN cargo build --release

FROM debian:stable-slim as auth_server

WORKDIR /app
COPY --from=builder /app/target/release/authorization /usr/local/bin

EXPOSE 50051
ENTRYPOINT ["/usr/local/bin/authorization"]

FROM debian:stable-slim as trx_server

WORKDIR /app
COPY --from=builder /app/target/release/transaction /usr/local/bin

EXPOSE 50051
ENTRYPOINT ["/usr/local/bin/transaction"]