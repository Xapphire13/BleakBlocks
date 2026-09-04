# Bleak Blocks
Spooky block breaking game for iOS/MacOS

## Prerequisites

Before building and packaging Bleak Blocks, ensure you have the following installed:

### Required
- **Rust** (latest stable version)
  - Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **cargo-packager** - For creating platform-specific packages
  - Install via: `cargo install cargo-packager`

### Platform-Specific Requirements

#### macOS
- Xcode Command Line Tools: `xcode-select --install`

#### iOS
- Xcode (full version from Mac App Store), signed in with an Apple ID under Xcode > Settings > Accounts (a free Personal Team is enough for your own device)
- iOS SDK (comes with Xcode)
- [XcodeGen](https://github.com/yonaskolb/XcodeGen): `brew install xcodegen`
- `rustup target add aarch64-apple-ios aarch64-apple-ios-sim`
- For device installs: `jq` (`brew install jq`), and an iPhone with Settings > Privacy & Security > Developer Mode turned on

## Building

### Development Build
```bash
cargo run
```

### Release Build
```bash
cargo build --release
```

## Packaging

This project uses `cargo-packager` to create distributable packages. The packaging configuration is defined in `Cargo.toml` under `[package.metadata.packager]`.

### macOS App Bundle
```bash
cargo packager --release
```

This will create a `.app` bundle and `.dmg` package in the `target/release` directory that can be distributed to other macOS users.

### Available Packaging Options
- **macOS .app bundle**: `cargo packager --release --formats app`
- **DMG**: `cargo packager --release --formats dmg`
- **All formats**: `cargo packager --release --formats all`

### Package Output
Packaged applications will be created in:
```
target/release/
├── Bleak Blocks.app/     # macOS app bundle
└── ...                   # Other formats if specified
```

### iOS Simulator
```bash
scripts/ios-simulator.sh ["iPhone 17 Pro"]
```
Builds for `aarch64-apple-ios-sim`, assembles a `.app` bundle, boots the simulator, installs, and launches. No signing required.

### iOS Device
```bash
scripts/ios-device.sh
```
Builds for `aarch64-apple-ios`, signs, installs, and launches on a connected, trusted iPhone. Uses the Xcode project generated from `ios/project.yml` (via XcodeGen) purely as a signing vehicle — the actual game binary is still built by `cargo` and swapped in by a build-phase script before Xcode code-signs the bundle.

One-time setup:
1. Sign into Xcode with your Apple ID (Xcode > Settings > Accounts). A free Personal Team works.
2. If your team ID differs from the one in `ios/project.yml` (`DEVELOPMENT_TEAM`), update it there.
3. Enable Developer Mode on the iPhone (Settings > Privacy & Security > Developer Mode), connect it, unlock it, and tap "Trust" when prompted.

## Development

### Running in Development Mode
```bash
cargo run
```

## Troubleshooting

### Common Issues
- **cargo-packager not found**: Ensure it's installed with `cargo install cargo-packager`
- **Build errors on macOS**: Make sure Xcode Command Line Tools are installed
