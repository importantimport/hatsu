FROM rust:slim-bookworm AS builder

ARG TARGET = "release"

WORKDIR /app

COPY . .

RUN cargo build --${TARGET}

FROM debian:bookworm-slim

COPY --from=builder /app/target/${TARGET}/hatsu /app/

EXPOSE 3939/tcp

CMD [ "./app/hatsu" ]
