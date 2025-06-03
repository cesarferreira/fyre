#!/bin/bash

# Fyre Installation Script

set -e

GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
RESET="\033[0m"

echo -e "${GREEN}🔥 Installing Fyre - Flutter Development Automation Tool${RESET}"
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo (Rust) is not installed. Please install Rust first:${RESET}"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build the project
echo -e "${YELLOW}📦 Building Fyre...${RESET}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Build successful!${RESET}"
else
    echo -e "${RED}❌ Build failed!${RESET}"
    exit 1
fi

# Get the current directory
CURRENT_DIR=$(pwd)
BINARY_PATH="$CURRENT_DIR/target/release/fyre"

echo
echo -e "${GREEN}🎉 Fyre has been built successfully!${RESET}"
echo -e "Binary location: ${YELLOW}$BINARY_PATH${RESET}"
echo

# Ask if user wants to add to PATH
read -p "$(echo -e "${YELLOW}Do you want to add Fyre to your PATH? (y/n): ${RESET}")" -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Detect shell
    if [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ]; then
        SHELL_RC="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ] || [ "$SHELL" = "/bin/bash" ]; then
        SHELL_RC="$HOME/.bashrc"
    else
        SHELL_RC="$HOME/.profile"
    fi
    
    # Add to PATH
    echo >> "$SHELL_RC"
    echo "# Fyre CLI Tool" >> "$SHELL_RC"
    echo "export PATH=\"\$PATH:$CURRENT_DIR/target/release\"" >> "$SHELL_RC"
    
    echo -e "${GREEN}✅ Added Fyre to PATH in $SHELL_RC${RESET}"
    echo -e "${YELLOW}💡 Restart your terminal or run: source $SHELL_RC${RESET}"
else
    echo -e "${YELLOW}💡 To use Fyre globally, add this to your shell profile:${RESET}"
    echo -e "   export PATH=\"\$PATH:$CURRENT_DIR/target/release\""
fi

echo
echo -e "${GREEN}🚀 Installation complete!${RESET}"
echo
echo -e "${YELLOW}Quick start:${RESET}"
echo -e "  • Run ${GREEN}fyre${RESET} to see available commands"
echo -e "  • Run ${GREEN}fyre search${RESET} for interactive command selection"
echo -e "  • Run ${GREEN}fyre --help${RESET} for detailed help"
echo 