ARG PROFILE="release"

FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG PROFILE
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --profile ${PROFILE} --recipe-path recipe.json
COPY . .
RUN apt update && apt install -y openssl libssl-dev pkg-config
RUN cargo build --profile ${PROFILE} && mv ./target/${PROFILE}/hatsu ./target/hatsu

FROM debian:bookworm-slim AS rumtime
RUN apt update && apt install openssl libssl-dev pkg-config
COPY --from=builder /app/target/hatsu /app/
EXPOSE 3939/tcp
CMD [ "/app/hatsu" ]
