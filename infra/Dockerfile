# syntax=docker/dockerfile:1
FROM rust:1.69 as builder
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    touch /app/src/main.rs && cargo build --release

# FIXME: okteto build では multi stage を使うとエラーになるので一旦ビルド用のイメージをそのまま使う
ENTRYPOINT ["/app/target/release/tabi_plan"]

#FROM gcr.io/distroless/cc:latest
#COPY --from=builder /app/target/release/tabi_plan /tabi_plan
#ENTRYPOINT ["/tabi_plan"]
