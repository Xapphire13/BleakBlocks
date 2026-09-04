#!/usr/bin/env bash
# Build Bleak Blocks for a physical iPhone, sign it, install it, and launch it.
#
# One-time setup before this will work:
#   1. Connect the iPhone via cable (or have it paired for wireless dev) and
#      trust this Mac on the device when prompted.
#   2. Settings > Privacy & Security > Developer Mode must be ON (restarts the
#      device once to take effect).
#   3. Xcode must be signed in with an Apple ID (Xcode > Settings > Accounts).
#      A free "Personal Team" is enough. The team ID used for signing is set
#      in ios/project.yml (DEVELOPMENT_TEAM).
#
# Usage: scripts/ios-device.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUNDLE_ID="com.xapphire13.bleak-blocks"

cd "$ROOT"

echo "==> Regenerating Xcode project"
(cd ios && xcodegen generate)

echo "==> Finding connected device"
DEVICES_JSON="$(mktemp)"
trap 'rm -f "$DEVICES_JSON"' EXIT
xcrun devicectl list devices --json-output "$DEVICES_JSON" >/dev/null

DEVICE_COUNT="$(jq '[.result.devices[] | select(.connectionProperties.tunnelState == "connected")] | length' "$DEVICES_JSON")"
if [ "$DEVICE_COUNT" -eq 0 ]; then
  echo "No connected iOS device found." >&2
  echo "Plug in your iPhone, unlock it, and tap 'Trust' if prompted, then try again." >&2
  exit 1
fi

DEVICE_ID="$(jq -r '[.result.devices[] | select(.connectionProperties.tunnelState == "connected")][0].identifier' "$DEVICES_JSON")"
DEVICE_NAME="$(jq -r '[.result.devices[] | select(.connectionProperties.tunnelState == "connected")][0].deviceProperties.name' "$DEVICES_JSON")"
echo "    Using $DEVICE_NAME ($DEVICE_ID)"

echo "==> Building and signing for device (this also builds the Rust binary via a build phase)"
xcodebuild \
  -project ios/BleakBlocks.xcodeproj \
  -scheme BleakBlocks \
  -configuration Debug \
  -destination "id=$DEVICE_ID" \
  -allowProvisioningUpdates \
  build

APP_PATH="$(find "$HOME/Library/Developer/Xcode/DerivedData" -type d -name "BleakBlocks.app" -path "*Debug-iphoneos*" -print -quit)"
if [ -z "$APP_PATH" ]; then
  echo "Couldn't locate the built BleakBlocks.app under DerivedData." >&2
  exit 1
fi
echo "    Built: $APP_PATH"

echo "==> Installing on device"
xcrun devicectl device install app --device "$DEVICE_ID" "$APP_PATH"

echo "==> Launching"
xcrun devicectl device process launch --terminate-existing --device "$DEVICE_ID" "$BUNDLE_ID"
