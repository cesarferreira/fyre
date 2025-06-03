# Fyre - Flutter Development Automation Tool

A powerful Rust-based CLI tool for automating Flutter development workflows. This is a Rust port of the original Ruby CLI tool with additional features like fuzzy search.

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
git clone <repository-url>
cd fyre
cargo build --release
```

The binary will be available at `target/release/fyre`.

### Add to PATH

To use the `fyre` command globally:

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$PATH:/path/to/fyre/target/release"
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

### Dependencies

The project uses several key Rust crates:

- `clap` - Command line argument parsing
- `colored` - Colorized terminal output
- `comfy-table` - Beautiful table formatting
- `opener` - Cross-platform URL opening
- `regex` - Regular expressions for version parsing
- `skim` - Fuzzy finder implementation
- `anyhow` - Error handling

## Comparison with Original Ruby Tool

### Improvements

- **Performance**: Significantly faster execution times
- **Fuzzy Search**: New interactive command selection
- **Self-contained**: No need for Ruby/gem dependencies
- **Better Error Handling**: More informative error messages
- **Native Version Bumping**: Implemented in Rust instead of shell script

### Compatibility

The Rust version maintains full command compatibility with the original Ruby tool while adding new features.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Your chosen license]

## Acknowledgments

- Original Ruby CLI tool inspiration
- Flutter team for the excellent development tools
- Rust community for the amazing ecosystem 