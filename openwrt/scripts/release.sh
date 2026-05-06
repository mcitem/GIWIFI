#!/usr/bin/env bash
set -e

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)

rm -rf "$ROOT_DIR/dist"
mkdir -p "$ROOT_DIR/dist"

cargo install cargo-cross

export RUST_TARGET="aarch64-linux-android"
cross build --manifest-path "$ROOT_DIR/../giwifi/Cargo.toml" --release --target "$RUST_TARGET"
cp "$ROOT_DIR/../giwifi/target/$RUST_TARGET/release/giwifi" "$ROOT_DIR/dist/giwifi_${RUST_TARGET}"
chmod 0755 "$ROOT_DIR/dist/giwifi_${RUST_TARGET}"

export RUST_TARGET="aarch64-unknown-linux-musl"
"$SCRIPT_DIR/build.sh"

export RUST_TARGET="x86_64-unknown-linux-musl"
"$SCRIPT_DIR/build.sh"

export RUST_TARGET="x86_64-pc-windows-gnu"
rustup target add "$RUST_TARGET"
sudo apt update
sudo apt install -y mingw-w64 gcc-mingw-w64-x86-64
cargo build --manifest-path "$ROOT_DIR/../giwifi/Cargo.toml" --release --target "$RUST_TARGET"
cp "$ROOT_DIR/../giwifi/target/$RUST_TARGET/release/giwifi.exe" "$ROOT_DIR/dist/giwifi_${RUST_TARGET}.exe"

export RUST_TARGET="x86_64-pc-windows-msvc"
cargo install cargo-xwin
rustup target add "$RUST_TARGET"
cargo xwin build --manifest-path "$ROOT_DIR/../giwifi/Cargo.toml" --release --target "$RUST_TARGET"
cp "$ROOT_DIR/../giwifi/target/$RUST_TARGET/release/giwifi.exe" "$ROOT_DIR/dist/giwifi_${RUST_TARGET}.exe"