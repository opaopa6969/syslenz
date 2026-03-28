# Multi-stage build for minimal syslenz container
# Usage:
#   docker build -t syslenz .
#   docker run --rm -p 9100:9100 syslenz --serve
#   syslenz --connect localhost:9100

FROM rust:1.83-slim AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build static binary with musl for minimal image
RUN apt-get update && apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/syslenz

# Minimal runtime image
FROM alpine:3.20

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/syslenz /usr/local/bin/syslenz

# /proc is mounted from host by default in Docker
EXPOSE 9100

ENTRYPOINT ["syslenz"]
CMD ["--serve"]
