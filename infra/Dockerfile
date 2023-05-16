# syntax=docker/dockerfile:1
FROM rust:1.69 as builder
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    touch /app/src/main.rs && cargo build --release && mv target/release/tabi_plan /tabi_plan
FROM gcr.io/distroless/cc:latest
COPY --from=builder /tabi_plan /tabi_plan
ENTRYPOINT ["/tabi_plan"]
