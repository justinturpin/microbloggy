FROM ubuntu:bionic

RUN apt-get update && \
    apt-get install graphicsmagick gosu -y && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /opt

COPY target/release/microbloggy /opt/microbloggy
COPY migrations /opt/migrations
COPY templates /opt/templates
COPY static /opt/static

ENV BIND_HOST=0.0.0.0:8080

EXPOSE 8080

RUN useradd -r microbloggy

CMD /opt/microbloggy
