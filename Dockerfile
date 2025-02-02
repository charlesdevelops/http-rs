# Builder stage
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/http-rs .
EXPOSE 8080
CMD ["./http-rs"]