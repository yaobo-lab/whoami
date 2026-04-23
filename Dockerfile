FROM rust:1.52.1 AS builder

WORKDIR /opt

COPY src /opt/src
COPY Cargo.lock /opt/Cargo.lock
COPY Cargo.toml /opt/Cargo.toml

RUN cargo install --path .

FROM ubuntu:20.04

COPY --from=builder /opt/target/release/rust-whoami /opt/rust-whoami

EXPOSE 8080

ENTRYPOINT ["/opt/rust-whoami"]