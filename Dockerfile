# ---- build stage ----
FROM rust:1-bookworm AS builder
WORKDIR /app

# Cache dependencies: build with dummy main first.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Build real code.
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ---- run stage ----
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Must match `name` in Cargo.toml [package].
ARG BIN_NAME=bot_discord
COPY --from=builder /app/target/release/${BIN_NAME} /usr/local/bin/bot

CMD ["bot"]
