# Build stage
FROM rust:1-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock rust-toolchain.toml build.rs ./
COPY migrations ./migrations
COPY src ./src
RUN cargo build --release --locked

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/oneticket-inventoryorders /usr/local/bin/inventoryorders
ENV INVENTORYORDERS_PORT=3000
EXPOSE 3000
CMD ["inventoryorders"]
