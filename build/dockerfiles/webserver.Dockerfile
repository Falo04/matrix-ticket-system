ARG RUST_VERSION=1.91.1

FROM rust:${RUST_VERSION}-slim-trixie AS buildrust

WORKDIR /app

RUN <<EOF
apt-get update
apt-get install -y openssl libssl-dev pkg-config
EOF

RUN --mount=type=bind,source=webserver/,target=webserver/ \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release -p webserver
cp ./target/release/webserver /bin/server
EOF


FROM debian:trixie-slim AS final

RUN <<EOF
apt-get update
apt-get install -y libssl-dev ca-certificates
EOF

# Copy startup script
COPY ./build/webserver/startup.sh /
RUN chmod +x /startup.sh

# Copy the executable from the "build" stage.
COPY --from=buildrust /bin/server /bin/

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=1000
RUN useradd \
    --home-dir "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser


RUN mkdir /var/lib/webserver && chown -R $UID /var/lib/webserver

# Copy migrations
COPY ./webserver/migrations /migrations

USER appuser

# What the container should run when it is started.
CMD ["/startup.sh"]