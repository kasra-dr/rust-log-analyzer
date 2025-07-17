# --- Stage 1: The Builder ---
FROM rust:1-alpine AS builder

WORKDIR /app 

COPY . .

RUN cargo build --release

# --- Stage 2: The Final Image ---
FROM alpine:latest

COPY --from=builder /app/target/release/log_analyzer /usr/local/bin/log_analyzer

ENTRYPOINT ["log_analyzer"]
