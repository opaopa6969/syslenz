# Multi-stage build for syslenz
#
# Docker Hub:  docker pull opaopa6969/syslenz
# Web UI:      docker run --rm -p 3000:3000 --pid=host opaopa6969/syslenz --web 3000
# TUI server:  docker run --rm -p 9100:9100 --pid=host opaopa6969/syslenz --serve

ARG FEATURES=web

# ---- builder ----------------------------------------------------------------
FROM rust:slim AS builder

ARG FEATURES
ARG TARGETARCH

WORKDIR /build

RUN apt-get update && apt-get install -y --no-install-recommends \
    musl-tools \
    gcc-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN case "$TARGETARCH" in \
      arm64) \
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc; \
        cargo build --release --features "$FEATURES" --target aarch64-unknown-linux-musl; \
        cp target/aarch64-unknown-linux-musl/release/syslenz /syslenz ;; \
      *) \
        cargo build --release --features "$FEATURES" --target x86_64-unknown-linux-musl; \
        cp target/x86_64-unknown-linux-musl/release/syslenz /syslenz ;; \
    esac && \
    strip /syslenz

# ---- runtime ----------------------------------------------------------------
FROM scratch

COPY --from=builder /syslenz /syslenz

EXPOSE 3000 9100

ENTRYPOINT ["/syslenz"]
CMD ["--web", "3000"]
