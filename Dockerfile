FROM rust:slim-bookworm AS builder

ARG PROFILE="release"

WORKDIR /app

COPY . .

RUN apt update && apt install -y openssl libssl-dev pkg-config

# debug build
RUN if [ "$PROFILE" = "debug" ]; then \
  cargo build && mv ./target/debug/hatsu ./target/hatsu \
  ; fi

# release build
RUN if [ "$PROFILE" = "release" ]; then \
  cargo build --release && mv ./target/release/hatsu ./target/hatsu \
  ; fi

FROM debian:bookworm-slim

COPY --from=builder /app/target/hatsu /app/

EXPOSE 3939/tcp

CMD [ "./app/hatsu" ]
