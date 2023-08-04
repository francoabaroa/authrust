FROM rust:latest as builder

# Add PostgreSQL client libraries
RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/authrust ./authrust

# Runtime image
FROM debian:bullseye-slim

# Install libpq5 for Postgres support
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/authrust /app/authrust

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

COPY templates templates

# Run the app
CMD ./authrust
