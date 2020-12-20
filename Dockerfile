FROM ubuntu:bionic

RUN apt-get update && apt-get install curl build-essential libssl-dev pkg-config -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install sqlx-cli  --no-default-features --features sqlite

WORKDIR /opt

COPY . .

ENV DATABASE_URL=sqlite:testdb.qlite

RUN sqlx database create && sqlx migrate run

RUN cargo build --release

FROM ubuntu:bionic

RUN apt-get update && \
    apt-get install gm -y && \
    apt-cache clear && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /opt

COPY --from=0 /opt/target/release/microbloggy /opt/microbloggy
COPY --from=0 /opt/migrations /opt/migrations
COPY --from=0 /opt/templates /opt/templates
COPY --from=0 /opt/static /opt/static

ENV BIND_HOST=0.0.0.0:8080

EXPOSE 8080

RUN apt-get update && \
    apt-get install gosu -y && \
    useradd -r microbloggy

CMD /opt/microbloggy
