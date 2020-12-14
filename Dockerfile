FROM rust

RUN cargo install sqlx-cli  --no-default-features --features sqlite

WORKDIR /opt

COPY . .

RUN cargo build --release

FROM ubuntu:bionic

WORKDIR /opt

COPY --from=0 /opt/target/release/microbloggy /opt/microbloggy
COPY --from=0 /opt/migrations /opt/migrations
COPY --from=0 /opt/static /opt/static
COPY --from=0 /root/.cargo/bin/sqlx /usr/local/bin/sqlx

ENV BIND_HOST=0.0.0.0:8080

EXPOSE 8080

CMD sqlx database create && sqlx migrate run && /opt/microbloggy
