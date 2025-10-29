# Multi-stage build for a minimal final image
# Stage 1: Builder
FROM rust:1.75-slim as builder

WORKDIR /usr/src/hashy

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY tests ./tests

# Build the application with optimizations
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/hashy/target/release/hashy /usr/local/bin/hashy

# Create a non-root user
RUN useradd -m -u 1000 hashy && \
    chown hashy:hashy /usr/local/bin/hashy

USER hashy
WORKDIR /home/hashy

ENTRYPOINT ["hashy"]
CMD ["--help"]

