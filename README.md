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
- Xcode (full version from Mac App Store)
- iOS SDK (comes with Xcode)
- Valid Apple Developer Account (for device deployment)

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

## Development

### Running in Development Mode
```bash
cargo run
```

## Troubleshooting

### Common Issues
- **cargo-packager not found**: Ensure it's installed with `cargo install cargo-packager`
- **Build errors on macOS**: Make sure Xcode Command Line Tools are installed
