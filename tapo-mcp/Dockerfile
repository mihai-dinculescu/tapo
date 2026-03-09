# ---- Stage 1: Chef ----
FROM rust:1.94-trixie AS chef
RUN cargo install cargo-chef
WORKDIR /app

# ---- Stage 2: Planner ----
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- Stage 3: Builder ----
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Cook dependencies (cached unless Cargo.toml/Cargo.lock change)
RUN cargo chef cook --release --package tapo-mcp --recipe-path recipe.json
# Copy full source and build
COPY . .
RUN cargo build --release --package tapo-mcp

# ---- Stage 4: Runtime ----
FROM gcr.io/distroless/cc-debian13 AS runtime
COPY --from=builder /app/target/release/tapo-mcp /usr/local/bin/tapo-mcp

# Bind to 0.0.0.0 inside container so the port is reachable from outside
ENV TAPO_MCP_HTTP_ADDR=0.0.0.0:3000
EXPOSE 3000

USER nonroot
ENTRYPOINT ["tapo-mcp"]
