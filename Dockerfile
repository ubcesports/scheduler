FROM rust:1 AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc AS runtime
COPY --from=builder /app/target/release/scheduler /usr/local/bin/scheduler
EXPOSE 5678
ENTRYPOINT ["/usr/local/bin/scheduler"]