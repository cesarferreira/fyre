# Fyre - Flutter Development Automation Tool

A powerful Rust-based CLI tool for automating Flutter development workflows.

## Features

- 🚀 **Fast and Reliable**: Built with Rust for maximum performance and reliability
- 🔍 **Fuzzy Search**: Interactive command selection with fuzzy search capabilities
- 🎯 **Flutter-First**: Designed specifically for Flutter development workflows
- 🛠️ **Comprehensive Tooling**: Covers the entire development lifecycle
- 💎 **Beautiful Output**: Colorized output and formatted tables

## Installation

### Prerequisites

- Rust (latest stable version)
- Flutter SDK
- Fastlane (for iOS releases)
- CocoaPods (for iOS dependency management)

### Build from Source

```bash
git clone git@github.com:cesarferreira/fyre.git
cd fyre
cargo install --path .
```

## Usage

Run `fyre` without arguments to see the available commands:

```bash
fyre
```

### Commands Overview

| Command | Description |
|---------|-------------|
| `fyre clean` | Deep cleans the project and rebuilds it |
| `fyre fix` | Automatically identifies and corrects common issues in Dart code |
| `fyre generate swagger` | Generates Swagger/OpenAPI client |
| `fyre generate icon` | Generates app icons |
| `fyre generate assets` | Generates asset definitions using fluttergen |
| `fyre watch` | Watches for code changes and rebuilds |
| `fyre open apple` | Opens App Store Connect |
| `fyre open android` | Opens Google Play Console |
| `fyre release beta` | Releases to beta track |
| `fyre release production` | Releases to production track |
| `fyre bump major/minor/patch/build` | Bumps version numbers |
| `fyre search` | Interactive fuzzy search for commands |

### Detailed Command Usage

#### Code Generation

```bash
# Generate Swagger/OpenAPI client
fyre generate swagger

# Generate app icons
fyre generate icon

# Generate asset definitions
fyre generate assets

# Shorthand for swagger generation
fyre gen
```

#### Project Management

```bash
# Deep clean and rebuild
fyre clean

# Apply automatic Dart fixes
fyre fix

# Watch for changes
fyre watch
```

#### Version Management

```bash
# Bump major version (1.0.0 -> 2.0.0)
fyre bump major

# Bump minor version (1.0.0 -> 1.1.0)
fyre bump minor

# Bump patch version (1.0.0 -> 1.0.1)
fyre bump patch

# Bump build number (1.0.0+1 -> 1.0.0+2)
fyre bump build
```

#### Release Management

```bash
# Release to beta track
fyre release beta

# Release to production
fyre release production
```

#### External Services

```bash
# Open App Store Connect
fyre open apple

# Open Google Play Console
fyre open android
```

#### Interactive Mode

```bash
# Launch fuzzy search interface
fyre search
```

The fuzzy search allows you to:
- Type to filter commands
- Use arrow keys to navigate
- Press Enter to execute
- Press Escape to cancel

## Project Structure

The tool expects to be run in a Flutter project directory (containing `pubspec.yaml`).

### Required Files

- `pubspec.yaml` - Flutter project configuration
- `ios/Podfile` - iOS dependencies (for release commands)
- `ios/` directory - iOS project (for release commands)

## Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run with arguments
cargo run -- <command>
```

## License

MIT
