FROM rust:1.78-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release


FROM debian:buster-slim as app

RUN apt-get update && apt-get install -y \
    libpq-dev

WORKDIR /app

COPY --from=builder /app/target/release/squishlink_rs .
COPY --from=builder /app/data ./data

ENTRYPOINT ["./squishlink_rs"]
