# Build
FROM rust:1.71.0 as builder
LABEL authors="mail@alexpts.dev"

WORKDIR /app
COPY . /app
RUN cargo build --release

# Run
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/shelly-save-to-db /

CMD ["./shelly-save-to-db"]
