
#!/usr/bin/env bash
set -euo pipefail

URL="https://downloads.openwrt.org/releases/25.12.2/targets/x86/64/openwrt-sdk-25.12.2-x86-64_gcc-14.3.0_musl.Linux-x86_64.tar.zst"
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
SDK_DIR="$ROOT_DIR/.sdk"

mkdir -p "$ROOT_DIR/.dl"
DL_FILE="$ROOT_DIR/.dl/${URL##*/}"
[[ -f "$DL_FILE" ]] || curl -fL "$URL" -o "$DL_FILE"

if [[ ! -d "$SDK_DIR" ]]; then
    mkdir -p "$SDK_DIR"
    zstd -d -c "$DL_FILE" | tar -xf - -C "$SDK_DIR" --strip-components=1
fi

"$SDK_DIR/scripts/feeds" update -a
"$SDK_DIR/scripts/feeds" install luci-base
