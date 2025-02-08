#!/usr/bin/env bash -x

# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain nightly \
    --component rust-src \
    --profile default \
    -y

## Add cargo binaries to PATH for this session
source "$HOME/.cargo/env"

## Install required cargo tools
cargo install cargo-generate
cargo install ravedude

## Verify installations
rustc --version
cargo --version
cargo generate --version
ravedude --version