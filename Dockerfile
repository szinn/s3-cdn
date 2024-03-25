FROM messense/rust-musl-cross:x86_64-musl@sha256:b70411ff08096d628f44dbadebc18ec5e047799a0a936bd31988ab8cd813836f as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y protobuf-compiler libssl-dev
WORKDIR /s3-cdn

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /s3-cdn/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN musl-strip /s3-cdn/target/x86_64-unknown-linux-musl/release/s3-cdn

FROM ubuntu:latest@sha256:f9d633ff6640178c2d0525017174a688e2c1aef28f0a0130b26bd5554491f0da AS ubuntu
RUN addgroup --gid 8779 server && useradd -g 8779 -M -u 8779 -s /usr/sbin/nologin server
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /s3-cdn/target/x86_64-unknown-linux-musl/release/s3-cdn /s3-cdn
USER server
ENTRYPOINT ["/s3-cdn"]

LABEL org.opencontainers.image.source https://github.com/szinn/s3-cdn
LABEL org.opencontainers.image.description "A self-hosted S3 CDN server"
