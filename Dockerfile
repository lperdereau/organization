FROM rust:1.60.0 as builder-rust
WORKDIR /app
COPY . /app
RUN cargo build --release --package organization

FROM debian AS builder
RUN apt update && apt install --yes libpq5
COPY --from=builder-rust /app/target/release/organization /
COPY copy-so-files.sh /
RUN sh /copy-so-files.sh

FROM gcr.io/distroless/cc
COPY --from=builder /tmp/linux-gnu/ /lib/aarch64-linux-gnu/
COPY --from=builder-rust /app/target/release/organization /
CMD ["/organization"]
