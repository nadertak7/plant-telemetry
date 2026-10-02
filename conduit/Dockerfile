FROM rust:1.96.0-slim-bookworm AS build

ARG PACKAGE_NAME
ENV SQLX_OFFLINE=true
WORKDIR /app
RUN --mount=type=bind,source=consumer,target=consumer \
    --mount=type=bind,source=shared,target=shared \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=migrations,target=migrations \
    --mount=type=bind,source=.sqlx,target=.sqlx \
    --mount=type=cache,target=/app/target/ \
    cargo build --release --locked -p "${PACKAGE_NAME}" && \
    cp "/app/target/release/${PACKAGE_NAME}" /usr/local/bin/service

FROM debian:bookworm-slim

ENV RUST_BACKTRACE=1
WORKDIR /app
RUN groupadd --gid 1000 appuser \
    && useradd --uid 1000 --gid 1000 -m -s /bin/bash appuser
COPY --from=build /usr/local/bin/service /app/service
USER appuser
CMD ["./service"]
