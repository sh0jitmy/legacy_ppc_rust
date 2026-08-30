FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies for host & cross compilation
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    cmake \
    file \
    bc \
    gawk \
    bison \
    flex \
    texinfo \
    ca-certificates \
    gcc-powerpc-linux-gnu \
    g++-powerpc-linux-gnu \
    libc6-dev-powerpc-cross \
    libstdc++-12-dev-powerpc-cross \
    qemu-user-static \
    qemu-system-ppc \
    busybox-static \
    && rm -rf /var/lib/apt/lists/*

# Install Rust Nightly with rust-src
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --default-toolchain nightly \
    --profile minimal \
    && rustup component add rust-src clippy rustfmt \
    && chmod -R a+w /usr/local/rustup /usr/local/cargo

# Build & install OpenSSL for PowerPC (for HTTPS/TLS verification)
ENV OPENSSL_PPC_PREFIX=/opt/openssl-powerpc
RUN mkdir -p /tmp/openssl-src && cd /tmp/openssl-src \
    && curl -sSL https://www.openssl.org/source/openssl-3.0.14.tar.gz | tar xz --strip-components=1 \
    && ./Configure linux-ppc no-shared no-async no-tests \
        --prefix=${OPENSSL_PPC_PREFIX} \
        --cross-compile-prefix=powerpc-linux-gnu- \
        -mcpu=8548 \
    && make -j$(nproc) \
    && make install_sw \
    && rm -rf /tmp/openssl-src

# Environment variables for Rust OpenSSL crate
ENV POWERPC_UNKNOWN_LINUX_GNUSPE_OPENSSL_DIR=${OPENSSL_PPC_PREFIX} \
    POWERPC_UNKNOWN_LINUX_GNUSPE_OPENSSL_STATIC=1 \
    PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /workspace
CMD ["/bin/bash"]
