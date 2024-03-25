FROM fedora:41 as chef
WORKDIR /build

# compile openssl for static linking
RUN dnf -y install gcc-c++ pkg-config musl-gcc git perl-core binaryen
RUN git clone git://git.openssl.org/openssl.git
RUN cd openssl && git checkout OpenSSL_1_1_1-stable
RUN cd openssl && ./config -fPIC no-weak-ssl-ciphers no-async --prefix=/usr/local/ssl --openssldir=/usr/local/ssl
RUN cd openssl && make && make install
ENV OPENSSL_STATIC true
ENV OPENSSL_DIR /usr/local/ssl

# install rust
RUN curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install 1.77.0

# add compilation targets
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef

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

FROM ubuntu:latest@sha256:77906da86b60585ce12215807090eb327e7386c8fafb5402369e421f44eff17e AS ubuntu
RUN addgroup --gid 8779 s3cdn && useradd -g 8779 -M -u 8779 -s /usr/sbin/nologin s3cdn
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
