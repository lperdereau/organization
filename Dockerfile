FROM debian AS builder
RUN apt update && apt install --yes \
    libpq5 \
    libgssapi-krb5-2 \
    libldap-2.4-2 \
    libkrb5-3 \
    libkrb5-dev \
    libcom-err2 \
    libkeyutils1

FROM rust:1.60.0 as builder-rust
WORKDIR /app
COPY . /app
RUN cargo build --release --package organization


FROM gcr.io/distroless/cc
COPY --from=builder /usr/lib/aarch64-linux-gnu/ /usr/lib/aarch64-linux-gnu/
COPY --from=builder /usr/lib/aarch64-linux-gnu/libgssapi_krb5.so.2 /usr/lib/aarch64-linux-gnu/libgssapi_krb5.so.2
COPY --from=builder /usr/lib/aarch64-linux-gnu/libldap_r-2.4.so.2 /usr/lib/aarch64-linux-gnu/libldap_r-2.4.so.2
COPY --from=builder /usr/lib/aarch64-linux-gnu/libkrb5.so.3 /usr/lib/aarch64-linux-gnu/libkrb5.so.3
COPY --from=builder /usr/lib/aarch64-linux-gnu/libk5crypto.so.3 /usr/lib/aarch64-linux-gnu/libk5crypto.so.3
COPY --from=builder /usr/lib/aarch64-linux-gnu/libcom_err.so /usr/lib/aarch64-linux-gnu/libcom_err.so
COPY --from=builder /lib/aarch64-linux-gnu/libkeyutils.so.1 /lib/aarch64-linux-gnu/libkeyutils.so.1
COPY --from=builder-rust /app/target/release/organization /
CMD ["/organization"]
