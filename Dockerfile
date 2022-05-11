FROM debian AS builder
RUN apt update && apt install --yes \
    libpq5 \
    libcom-err2 \
    libkeyutils1

FROM rust:1.60.0 as builder-rust
WORKDIR /app
COPY . /app
RUN cargo build --release --package organization

FROM gcr.io/distroless/cc
COPY --from=builder /usr/lib/aarch64-linux-gnu/ /lib/aarch64-linux-gnu/
COPY --from=builder /usr/lib/aarch64-linux-gnu/libpq.so.5 /lib/aarch64-linux-gnu/libpq.so.5
COPY --from=builder /lib/aarch64-linux-gnu/libcom_err.so.2 /lib/aarch64-linux-gnu/libcom_err.so.2
COPY --from=builder /lib/aarch64-linux-gnu/libkeyutils.so.1 /lib/aarch64-linux-gnu/libkeyutils.so.1
COPY --from=builder-rust /app/target/release/organization /
CMD ["/organization"]
