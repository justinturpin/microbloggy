FROM ubuntu:bionic

RUN apt-get update && \
    apt-get install curl build-essential libssl-dev pkg-config -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install sqlx-cli  --no-default-features --features sqlite

WORKDIR /opt
