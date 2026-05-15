# Multi-stage Dockerfile for the naipes Rust CLI.
#
# Stage 1 (builder): compile the binary from source using the official
# Rust toolchain.
#
# Stage 2 (runtime): copy only the compiled binary into a minimal
# Debian slim image (~80MB total).

# ============================================================
# Stage 1: builder
# ============================================================
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /build

# Copy the full source tree.
# We don't use the "compile dependencies first" caching trick because
# this crate has both [lib] and [[bin]] targets, which makes the
# dummy-src approach unreliable. Plain copy-then-build is simpler
# and equally correct for our release cadence.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

# Build in release mode with locked dependencies for reproducibility.
RUN cargo build --release --locked && \
    strip target/release/naipes

# ============================================================
# Stage 2: runtime
# ============================================================
FROM debian:bookworm-slim

# Install only the bare minimum runtime libraries.
# ca-certificates: in case future versions make HTTPS calls.
# Cleanup apt cache to keep the image small.
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage.
COPY --from=builder /build/target/release/naipes /usr/local/bin/naipes

# Create a non-root user for running the application.
# Security best practice: don't run as root inside containers.
RUN useradd --create-home --shell /bin/bash naipes
USER naipes
WORKDIR /home/naipes

# Default entrypoint: the naipes binary.
# Arguments to "docker run naipes/naipes ..." are passed to the binary.
# Example: docker run naipes/naipes play brisca --seed 42
ENTRYPOINT ["naipes"]

# Default CMD if no arguments given: show help.
CMD ["--help"]

# Image metadata.
LABEL org.opencontainers.image.title="naipes"
LABEL org.opencontainers.image.description="CLI for traditional card games (brisca, tute, mus, chinchón)"
LABEL org.opencontainers.image.url="https://naipes.com"
LABEL org.opencontainers.image.source="https://github.com/naipes-com/naipes"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.authors="Manuel Harguindey"
