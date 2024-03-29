FROM bitnami/minideb:bookworm

ARG TARGETARCH

WORKDIR /app

COPY target_${TARGETARCH}/hatsu /app/hatsu

RUN install_packages openssl libssl-dev ca-certificates curl && \
  # https://github.com/casey/just#pre-built-binaries
  curl -sSf https://just.systems/install.sh | bash -s -- --tag 1.23.0 --to /usr/local/bin && \
  chmod +x /app/hatsu

ENTRYPOINT [ "/app/hatsu" ]

EXPOSE 3939

STOPSIGNAL SIGTERM
