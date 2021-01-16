FROM rust:1.31

WORKDIR /rust-api

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y default-mysql-client

RUN rustup install stable
RUN rustup install nightly
RUN rustup default nightly
RUN cargo +stable install diesel_cli --no-default-features --features mysql
