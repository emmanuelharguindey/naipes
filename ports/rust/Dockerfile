# Multi-stage Dockerfile for the naipes Rust CLI.
#
# Stage 1 (builder): compile the binary from source using the official
# Rust toolchain. We use the slim variant of the official image to keep
# the build stage smaller.
#
# Stage 2 (runtime): copy only the compiled binary into a minimal
# Debian slim image. We use debian:bookworm-slim instead of scratch
# because the binary may depend on glibc and CA certificates, and
# alpine would require musl-target compilation. bookworm-slim gives
# us a small, well-supported runtime (~80MB total image).

# ============================================================
# Stage 1: builder
# ============================================================
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /build

# Copy manifest first to leverage Docker layer caching:
# if Cargo.toml/Cargo.lock don't change, dependencies don't rebuild.
COPY Cargo.toml Cargo.lock ./

# Create a dummy src/main.rs to compile dependencies in isolation.
# This is the standard Rust Docker pattern for dependency caching.
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --locked && \
    rm -rf src target/release/naipes target/release/naipes.d

# Now copy the actual source code and build.
COPY src ./src
COPY tests ./tests

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
