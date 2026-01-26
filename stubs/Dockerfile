FROM rust:1.75 as builder
WORKDIR /workspace

# Copy workspace files
COPY Cargo.toml Cargo.lock* ./
COPY stubs ./stubs

# Build the stubs binary
RUN cargo build --release --package stubs

# Runtime stage
FROM debian:bookworm-slim
LABEL authors="alvaro"

COPY --from=builder /workspace/target/release/stubs /usr/local/bin/stubs

ENTRYPOINT ["stubs"]
