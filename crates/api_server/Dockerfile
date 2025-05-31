FROM rust:1.75 as builder

# Set the working directory to the project root (where the workspace Cargo.toml is)
WORKDIR /app

COPY . .

RUN cargo build --release --bin api_server

FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/api_server /usr/local/bin/api_server

# Copy the .env file and resources
COPY .env /
COPY crates/did_library/resources /did_library/resources

# Set the DID_REGISTRY_PATH environment variable
ENV DID_REGISTRY_PATH="/did_library/resources/did_registry.json"

EXPOSE 8080

# Run the API server
CMD ["api_server"]