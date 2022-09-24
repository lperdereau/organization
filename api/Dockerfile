FROM rust:1.60.0 as builder-rust
WORKDIR /app
COPY . /app
RUN cargo build --release --package organization

FROM debian AS builder
RUN apt update && apt install --yes libpq5
COPY --from=builder-rust /app/target/release/organization /
COPY copy-so-files.sh /
RUN sh /copy-so-files.sh /organization

FROM gcr.io/distroless/cc
ENV LD_LIBRARY_PATH=/lib/linux-gnu/
COPY --from=builder /tmp/linux-gnu/ /lib/linux-gnu/
COPY --from=builder-rust /app/target/release/organization /
CMD ["/organization"]
