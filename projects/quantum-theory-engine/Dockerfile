# Use official Rust image
FROM rust:1.75 as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libopenblas-dev \
    libhdf5-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./
COPY crates/ ./crates/
COPY cli/ ./cli/
COPY python_bindings/ ./python_bindings/

# Build dependencies (cached layer)
RUN cargo build --release

# Copy source code
COPY . .

# Build release
RUN cargo build --release --bin qte-cli

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libopenblas0 \
    libhdf5-103 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/qte-cli /usr/local/bin/qte

# Copy examples
COPY dsl_examples/ /examples/

# Set working directory
WORKDIR /workspace

# Default command
ENTRYPOINT ["qte"]
CMD ["--help"]
