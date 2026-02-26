FROM rust:1.93.1-bullseye

# Install ARM cross-compiler
RUN apt-get update && apt-get install -y \
    gcc-arm-linux-gnueabihf \
    libc6-dev-armhf-cross \
    make \
 && rm -rf /var/lib/apt/lists/*

# Add Rust ARM target & trunk
RUN rustup target add \
    armv7-unknown-linux-gnueabihf \
    armv7-unknown-linux-musleabihf \
    wasm32-unknown-unknown
RUN cargo install trunk --locked
