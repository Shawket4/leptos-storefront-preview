# syntax=docker/dockerfile:1.7
# The storefront: one Rust server (Leptos SSR on Axum) with the TypeScript client bundled into it,
# plus the site assets (hashed CSS, fonts) and the catalogue data (JSON, WebP, search synonyms).
# Built like MadarRust's image: cargo-chef caches the dependencies in their own layer, the release
# profile is fat LTO with one codegen unit, and CI targets the VPS's CPU (x86-64-v4).
#
# madar-shared (git dependency) is PRIVATE. Each cargo RUN mounts the BuildKit secret
# `madar_shared_token` (docker build --secret id=madar_shared_token,env=...) and, only inside that
# RUN, points git at it via GIT_CONFIG_* env (nothing is written to disk, so no layer or cache entry
# holds the token). Secret absent or empty: an anonymous fetch.

# ── Stage 1: cargo-chef recipe ─────────────────────────────────────────────────
FROM rust:1.98-slim-bookworm AS chef
RUN apt-get update && apt-get install -y --no-install-recommends \
      pkg-config ca-certificates curl git nodejs npm \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install --locked cargo-chef --version 0.1.73
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ── Stage 2: build ─────────────────────────────────────────────────────────────
FROM chef AS build
# cargo-leptos still builds an (empty) WASM placeholder crate; no page loads it.
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked cargo-leptos --version 0.3.11

# LTO_MODE=fat (default, CI/prod) or thin (a quick local image). Set for `cook` AND the build:
# Cargo fingerprints the profile, so dependencies cooked under one value would be compiled again
# under another (MadarRust's lesson).
ARG LTO_MODE=fat
# TARGET_CPU: CI passes x86-64-v4 (AVX-512) for the VPS's AMD EPYC; the default is the baseline,
# so a local image runs anywhere. It goes only to x86_64 target artifacts: the server is built for
# an explicit --target (LEPTOS_BIN_TARGET_TRIPLE), which keeps the flag off build scripts and proc
# macros (they run on the build machine, whose CPU may lack v4) and off the WASM placeholder.
ARG TARGET_CPU=x86-64
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C target-cpu=${TARGET_CPU}" \
    CARGO_PROFILE_RELEASE_LTO=${LTO_MODE} \
    LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-gnu \
    # Content-hashed CSS + hash.txt (Cargo.toml keeps hash-files off for `cargo leptos watch`).
    LEPTOS_HASH_FILES=true \
    # leptos and leptos_config read this at COMPILE time (`option_env!`), and cargo-leptos sets it
    # for its build: set for `cook` too, or the cooked leptos crates are rebuilt.
    LEPTOS_OUTPUT_NAME=storefront

# Dependencies only — this layer changes when Cargo.lock does. Same package, features, profile and
# target as cargo-leptos's server build below, so it reuses every artifact.
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=secret,id=madar_shared_token \
    set -e; \
    if [ -s /run/secrets/madar_shared_token ]; then \
      export CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 \
        GIT_CONFIG_KEY_0="url.https://x-access-token:$(cat /run/secrets/madar_shared_token)@github.com/Shawket4/madar-shared.insteadOf" \
        GIT_CONFIG_VALUE_0="https://github.com/Shawket4/madar-shared"; \
    fi; \
    cargo chef cook --release --target x86_64-unknown-linux-gnu --package server \
      --no-default-features --recipe-path recipe.json

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
    cargo leptos build --release; \
    mkdir -p /out; \
    cp target/x86_64-unknown-linux-gnu/release/server /out/server; \
    cp "$(find target -name hash.txt -path '*release*' | head -1)" /out/hash.txt; \
    cp -r target/site /out/site; \
    # cargo-leptos also builds the empty WASM placeholder crate; no page loads it, so don't ship it.
    rm -f /out/site/pkg/*.wasm /out/site/pkg/*.js /out/site/pkg/*.ts

# ── Stage 3: runtime ───────────────────────────────────────────────────────────
FROM debian:bookworm-slim
RUN useradd --system --uid 10001 --no-create-home storefront
WORKDIR /app
# The catalogue (JSON, ~290 MB of WebP, synonyms) changes rarely: its own layer, ahead of the
# binary, so a code-only deploy re-pushes a few MB.
COPY --chown=storefront data /app/data
COPY --from=build --chown=storefront /out/site /app/site
COPY --from=build /out/server /app/server
# cargo-leptos's map of hashed file names (hash-files), read next to the binary.
COPY --from=build /out/hash.txt /app/hash.txt

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
