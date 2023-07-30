ARG PROFILE="release"

FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG PROFILE
RUN apt update && apt install -y openssl libssl-dev pkg-config
COPY --from=planner /app/recipe.json recipe.json
# debug cook
RUN if [ "$PROFILE" = "debug" ]; then \
  cargo chef cook --recipe-path recipe.json \
  ; fi
# release cook
RUN if [ "$PROFILE" = "release" ]; then \
  cargo chef cook --release --recipe-path recipe.json \
  ; fi
COPY . .
# debug build
RUN if [ "$PROFILE" = "debug" ]; then \
  cargo build && mv ./target/debug/hatsu ./target/hatsu \
  ; fi
# release build
RUN if [ "$PROFILE" = "release" ]; then \
  cargo build --release && mv ./target/release/hatsu ./target/hatsu \
  ; fi

FROM debian:bookworm-slim AS rumtime
WORKDIR /app
RUN apt update && apt install -y openssl libssl-dev pkg-config
COPY --from=builder /app/target/hatsu /app/
EXPOSE 3939/tcp
CMD [ "/app/hatsu" ]
