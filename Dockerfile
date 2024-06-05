FROM bitnami/minideb:bookworm

ARG TARGETARCH

WORKDIR /app

COPY target_${TARGETARCH}/hatsu /app/hatsu

RUN install_packages openssl libssl-dev ca-certificates curl && \
  # https://github.com/casey/just#pre-built-binaries
  curl -sSf https://just.systems/install.sh | bash -s -- --tag 1.23.0 --to /usr/local/bin && \
  chmod +x /app/hatsu

ENV HATSU_LISTEN_PORT=3939
EXPOSE $HATSU_LISTEN_PORT

HEALTHCHECK CMD [ "curl", "--fail", "http://localhost:${HATSU_LISTEN_PORT}/api/v0/generate_204" ]

ENTRYPOINT [ "/app/hatsu" ]

STOPSIGNAL SIGTERM
