FROM ghcr.io/szinn/rust-musl-chef:1.81.0 as chef
WORKDIR /build

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /build/target/x86_64-unknown-linux-musl/release/s3-cdn

FROM ubuntu:latest@sha256:ab64a8382e935382638764d8719362bb50ee418d944c1f3d26e0c99fae49a345 AS ubuntu
RUN groupadd --gid 8779 s3cdn && useradd -g 8779 -M -u 8779 -s /usr/sbin/nologin s3cdn
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates

# Create a new stage with a minimal image
FROM scratch
COPY --from=ubuntu /etc/passwd /etc/passwd
COPY --from=ubuntu /etc/group /etc/group
COPY --from=ubuntu /etc/ssl/ /etc/ssl/
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/s3-cdn /s3-cdn
USER s3cdn
ENTRYPOINT ["/s3-cdn"]

LABEL org.opencontainers.image.source https://github.com/szinn/s3-cdn
LABEL org.opencontainers.image.description "A self-hosted S3 CDN server"
