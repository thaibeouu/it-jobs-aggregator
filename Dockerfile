FROM node:16.3.0-buster-slim as CLIENT

WORKDIR /tmp
COPY ./client ./

RUN \
    npm ci && \
    npm run build

FROM rustlang/rust:nightly-buster-slim as SERVER

WORKDIR /tmp
RUN \
    apt-get -y update && \
    apt-get install -y pkg-config libssl-dev libc-dev sqlite3 libsqlite3-dev
COPY ./Cargo* ./
COPY ./Rocket* ./
COPY ./db ./db
COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim
RUN \
    apt-get update && \
    apt-get install -y libssl-dev openssl sqlite3 libsqlite3-dev ca-certificates && \
    apt-get clean

WORKDIR /app
COPY --from=SERVER /tmp/target/release/it-jobs-aggregator .
COPY ./Rocket* ./
COPY ./db ./db
COPY --from=CLIENT /tmp/public ./client/public

EXPOSE 8000

ENTRYPOINT [ "/app/it-jobs-aggregator" ]
