# Dockerfile for Sora AI Watermark Service
FROM rust:1.82-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy everything needed for build (except Cargo.lock to avoid version mismatch)
COPY Cargo.toml ./
COPY src ./src
COPY static ./static

# Build for release (cargo will generate a compatible lockfile automatically)
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install FFmpeg and runtime dependencies
RUN apt-get update && apt-get install -y \
    ffmpeg \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/webtest /app/webtest

# Copy static files and watermark
COPY static ./static
COPY sora-watermark.mp4 ./sora-watermark.mp4

# Railway will set PORT env var dynamically, but we expose a default
# Note: EXPOSE is just documentation, the app binds to whatever PORT env var is set
EXPOSE 8080

# Run the binary
CMD ["/app/webtest"]
