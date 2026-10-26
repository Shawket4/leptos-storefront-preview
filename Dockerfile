# syntax=docker/dockerfile:1.7
# The storefront: one Rust server (Leptos SSR on Axum) with the TypeScript client bundled into it,
# plus the site assets (hashed CSS, fonts) and the catalogue data (JSON, WebP, search synonyms).
#
# madar-shared (git dependency) is PRIVATE. The cargo RUN mounts the BuildKit secret
# `madar_shared_token` (docker build --secret id=madar_shared_token,env=...) and, only inside that
# RUN, points git at it via GIT_CONFIG_* env (nothing is written to disk, so no layer or cache entry
# holds the token) — the same scheme as MadarRust's Dockerfile.

# ── Stage 1: build ─────────────────────────────────────────────────────────────
FROM rust:1.98-slim-bookworm AS build
RUN apt-get update && apt-get install -y --no-install-recommends \
      pkg-config ca-certificates curl git nodejs npm \
    && rm -rf /var/lib/apt/lists/*
# cargo-leptos still builds an (empty) WASM placeholder crate; no page loads it.
RUN rustup target add wasm32-unknown-unknown
# Compiled once and cached as a layer (nothing above changes between deploys).
RUN cargo install --locked cargo-leptos --version 0.3.11

WORKDIR /app
# The client toolchain (esbuild + TypeScript, pinned by the lockfile) — its own layer.
COPY client/package.json client/package-lock.json client/
RUN npm ci --prefix client

COPY . .
RUN --mount=type=secret,id=madar_shared_token \
    set -e; \
    if [ -s /run/secrets/madar_shared_token ]; then \
      export CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 \
        GIT_CONFIG_KEY_0="url.https://x-access-token:$(cat /run/secrets/madar_shared_token)@github.com/Shawket4/madar-shared.insteadOf" \
        GIT_CONFIG_VALUE_0="https://github.com/Shawket4/madar-shared"; \
    fi; \
    cargo leptos build --release

# ── Stage 2: runtime ───────────────────────────────────────────────────────────
FROM debian:bookworm-slim
RUN useradd --system --uid 10001 --no-create-home storefront
WORKDIR /app
# The catalogue (JSON, ~290 MB of WebP, synonyms) changes rarely: its own layer, ahead of the
# binary, so a code-only deploy re-pushes a few MB.
COPY --from=build --chown=storefront /app/data /app/data
COPY --from=build --chown=storefront /app/target/site /app/site
COPY --from=build /app/target/release/server /app/server
# cargo-leptos's map of hashed file names (hash-files = true), read next to the binary.
COPY --from=build /app/target/release/hash.txt /app/hash.txt

ENV LEPTOS_OUTPUT_NAME=storefront \
    LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:8080 \
    LEPTOS_ENV=PROD \
    LEPTOS_HASH_FILES=true \
    DATA_DIR=/app/data \
    MEDIA_DIR=/app/data/media \
    RUST_LOG=info
USER storefront
EXPOSE 8080
CMD ["/app/server"]
