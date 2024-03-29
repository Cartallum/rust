FROM ubuntu:20.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    g++ \
    make \
    ninja-build \
    file \
    curl \
    ca-certificates \
    python3 \
    git \
    cmake \
    sudo \
    gdb \
    libssl-dev \
    pkg-config \
    xz-utils

ENV RUSTUP_INIT_SKIP_PATH_CHECK="yes"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN PATH="${HOME}/.cargo/bin:${PATH}" \
    cargo install --git https://github.com/Cartallum/cargo-run-sbf-tests.git \
    --rev 9d45e79cf11b2fdf0e3572d916fe058823c7c438 \
    --bin cargo-run-sbf-tests --root /usr/local

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV RUST_CONFIGURE_ARGS \
    --set rust.lld \
    --set llvm.clang

ENV SCRIPT CARGO_TARGET_BPFEL_UNKNOWN_UNKNOWN_RUNNER=\"cargo-run-sbf-tests --heap-size 104857600\" \
    CARGO_TARGET_SBF_CARTALLUM_CBE_RUNNER=\"cargo-run-sbf-tests --heap-size 104857600\" \
    LLVM_HOME=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm \
    PATH="${HOME}/.cargo/bin:${PATH}" \
    python3 /checkout/x.py --stage 1 test --host='' --target bpfel-unknown-unknown,sbf-cartallum-cbe \
    library/core
