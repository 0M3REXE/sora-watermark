# Dockerfile for Sora AI Watermark Service
FROM rust:1.75-slim as builder

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

# Expose port
EXPOSE 8000

# Set environment variable for binding to all interfaces
ENV BIND_ADDRESS=0.0.0.0:8000

# Run the binary
CMD ["/app/webtest"]
