# Build stage
FROM rust:slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    curl \
    libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set root working directory
WORKDIR /app

# Copy full project
COPY . .

# Build from workspace root, specifying the binary
RUN cargo build --release --bin api_server

# Runtime stage (for now using slim base for debugging)
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/api_server /usr/local/bin/api_server
RUN if ["$(ls -A /did_library/resources)"]; then \
        echo "Copying non-empty resources..."; \
        cp -r  /did_library/resources /did_library/resources \
    else \
        echo "Resources is empty. Skipping..." \
    fi
 ENV DID_REGISTRY_PATH="/did_library/resources/did_registry.json"
EXPOSE 8080

CMD ["api_server"]
