FROM rust:latest

WORKDIR /usr/src/crates-io-ops-bot
COPY . .

RUN cargo install --path .

CMD ["crates-io-ops-bot"]