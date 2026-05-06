#!/usr/bin/env bash
set -euo pipefail

RUST_TARGET="${RUST_TARGET:-aarch64-unknown-linux-musl}"
echo "RUST_TARGET: $RUST_TARGET"

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
SDK_DIR="$ROOT_DIR/.sdk"

if [[ ! -d "$SDK_DIR" ]]; then
    "$SCRIPT_DIR/sdk.sh"
fi

cd "$SDK_DIR"
rm -rf ./bin

cross build --manifest-path "$ROOT_DIR/../giwifi/Cargo.toml" --release --target "$RUST_TARGET"

cp "$ROOT_DIR/../giwifi/target/$RUST_TARGET/release/giwifi" "$ROOT_DIR/dist/giwifi_${RUST_TARGET}"
chmod 0755 "$ROOT_DIR/dist/giwifi_${RUST_TARGET}"

mkdir -p "$ROOT_DIR/package/luci-app-giwifi/root/usr/bin"
cp "$ROOT_DIR/../giwifi/target/$RUST_TARGET/release/giwifi" "$ROOT_DIR/package/luci-app-giwifi/root/usr/bin/giwifi"
chmod 0755 "$ROOT_DIR/package/luci-app-giwifi/root/usr/bin/giwifi"

mkdir -p package/local/luci-app-giwifi
cp -r "$ROOT_DIR/package/luci-app-giwifi/." package/local/luci-app-giwifi/
echo "CONFIG_PACKAGE_luci-app-giwifi=m" >> .config
make defconfig
make package/luci-app-giwifi/clean V=sc || true
make package/luci-app-giwifi/compile V=sc

DIST_DIR="$SCRIPT_DIR/../dist"
mkdir -p "$DIST_DIR"
find bin/packages -name 'luci-app-giwifi*.ipk' -exec bash -c 'cp -v "$1" "$2/$(basename "$1" .ipk)_'"$RUST_TARGET"'.ipk"' _ {} "$DIST_DIR" \;
find bin/packages -name 'luci-app-giwifi*.apk' -exec bash -c 'cp -v "$1" "$2/$(basename "$1" .apk)_'"$RUST_TARGET"'.apk"' _ {} "$DIST_DIR" \;
echo "Output: $DIST_DIR"
ls -la "$DIST_DIR/"