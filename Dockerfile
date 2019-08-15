FROM rust:slim-stretch

ADD . .
RUN cargo test
