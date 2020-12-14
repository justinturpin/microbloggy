FROM rust

RUN cargo install sqlx-cli  --no-default-features --features sqlite

WORKDIR /opt

COPY . .

ENV DATABASE_URL=sqlite:testdb.qlite

RUN sqlx database create && sqlx migrate run

RUN cargo build --release

FROM ubuntu:bionic

WORKDIR /opt

COPY --from=0 /opt/target/release/microbloggy /opt/microbloggy
COPY --from=0 /opt/migrations /opt/migrations
COPY --from=0 /opt/static /opt/static

ENV BIND_HOST=0.0.0.0:8080

EXPOSE 8080

RUN apt-get update &&
    apt-get install su-exec -y && \
    useradd -r -g microbloggy microbloggy

CMD sqlx database create && sqlx migrate run && /opt/microbloggy
