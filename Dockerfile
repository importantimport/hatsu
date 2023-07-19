FROM rust:slim-bookworm AS builder

RUN USER=root cargo new --bin app

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/hatsu /app/

CMD [ "./app/hatsu" ]
