#!/usr/bin/env bash
# Build Bleak Blocks for the iOS Simulator and run it.
#
# Usage: scripts/ios-simulator.sh ["iPhone 17 Pro"]

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEVICE="${1:-iPhone 17 Pro}"
BUNDLE_ID="com.xapphire13.bleak-blocks"
APP_DIR="$ROOT/target/ios-sim/BleakBlocks.app"

cd "$ROOT"

echo "==> Building for aarch64-apple-ios-sim"
cargo build --target aarch64-apple-ios-sim --release

echo "==> Assembling app bundle"
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR"
cp target/aarch64-apple-ios-sim/release/bleak-blocks "$APP_DIR/BleakBlocks"
cat > "$APP_DIR/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleExecutable</key>
	<string>BleakBlocks</string>
	<key>CFBundleIdentifier</key>
	<string>$BUNDLE_ID</string>
	<key>CFBundleName</key>
	<string>Bleak Blocks</string>
	<key>CFBundleDisplayName</key>
	<string>Bleak Blocks</string>
	<key>CFBundleVersion</key>
	<string>1</string>
	<key>CFBundleShortVersionString</key>
	<string>0.1.0</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleSupportedPlatforms</key>
	<array>
		<string>iPhoneSimulator</string>
	</array>
	<key>DTPlatformName</key>
	<string>iphonesimulator</string>
	<key>MinimumOSVersion</key>
	<string>13.0</string>
	<key>UIRequiredDeviceCapabilities</key>
	<array>
		<string>arm64</string>
	</array>
	<key>UILaunchStoryboardName</key>
	<string></string>
</dict>
</plist>
EOF

echo "==> Booting simulator: $DEVICE"
xcrun simctl boot "$DEVICE" 2>/dev/null || true
open -a Simulator

echo "==> Installing"
xcrun simctl install "$DEVICE" "$APP_DIR"

echo "==> Launching"
xcrun simctl terminate "$DEVICE" "$BUNDLE_ID" 2>/dev/null || true
xcrun simctl launch "$DEVICE" "$BUNDLE_ID"
