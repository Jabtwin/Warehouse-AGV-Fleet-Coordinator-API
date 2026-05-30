# Stage 1: Build
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Run
FROM alpine:latest
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/agv_coordinator .
EXPOSE 3000
CMD ["./agv_coordinator"]
