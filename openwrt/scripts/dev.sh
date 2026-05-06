#!/bin/bash
set -e

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
bash "$SCRIPT_DIR/build.sh"

HOST="192.168.1.1"
USER="root"
APK="$SCRIPT_DIR/../dist/luci-app-giwifi-0.2.0-r2_aarch64-unknown-linux-musl.apk"
REMOTE_PATH="/root"

[ -f "$APK" ] || { echo "APK not found: $APK"; exit 1; }

APK_NAME=$(basename "$APK")
SSH_OPTS="-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null"

echo "Pushing $APK_NAME to $USER@$HOST..."
scp $SSH_OPTS "$APK" "$USER@$HOST:$REMOTE_PATH/"

ssh $SSH_OPTS "$USER@$HOST" "
    apk del -f luci-app-giwifi 2>/dev/null || true
    cd $REMOTE_PATH && apk add --allow-untrusted $APK_NAME
    apk list --installed | grep -i giwifi
"
echo "Done."