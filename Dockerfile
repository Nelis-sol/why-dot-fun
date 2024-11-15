# Use Cargo Chef for dependency caching and build stages
FROM lukemathwalker/cargo-chef:latest-rust-1.82.0 as chef
WORKDIR /app
RUN apt update && apt install -y lld clang

# Planner stage for caching dependencies
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage for building the application
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy application source and build the final binary
COPY . .

RUN cargo build --release
FROM lukemathwalker/cargo-chef:latest-rust-1.82.0 AS runtime
WORKDIR /app

# Install only necessary runtime dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user and switch to it
RUN useradd -m appuser
USER appuser

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/gamecall gamecall

# Expose the port your application is listening on (replace 8080 with your actual port)
EXPOSE 8080

# Set the command to run your API
CMD ["./gamecall"]
