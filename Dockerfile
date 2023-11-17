FROM rust:1.71.0
LABEL authors="mail@alexpts.dev"

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/shelly-save-to-db"]
