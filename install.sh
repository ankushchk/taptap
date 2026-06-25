#!/bin/sh
set -e

# CLI installer for taptap
# Detects OS and CPU architecture, downloads the latest binary, and installs it.

GITHUB_REPO="ankushchk/taptap"
BINARY_NAME="taptap"

echo "=== Installing taptap ==="

# 1. Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)     PLATFORM="unknown-linux-musl" ;;
    Darwin*)    PLATFORM="apple-darwin" ;;
    *)          echo "Unsupported OS: $OS"; exit 1 ;;
esac

# 2. Detect Architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64*)    TARGET_ARCH="x86_64" ;;
    arm64*|aarch64*) TARGET_ARCH="aarch64" ;;
    *)          echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

TARGET="${TARGET_ARCH}-${PLATFORM}"
echo "Detected target: $TARGET"

# 3. Determine download URL
# For production, we fetch from GitHub Releases:
# URL="https://github.com/${GITHUB_REPO}/releases/latest/download/${BINARY_NAME}-${TARGET}"
# For testing/reference, we use a generic placeholder download output:
DOWNLOAD_URL="https://github.com/${GITHUB_REPO}/releases/download/v0.1.0/${BINARY_NAME}-${TARGET}"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
TMP_BINARY="${TMP_DIR}/${BINARY_NAME}"

echo "Downloading pre-compiled binary..."
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$TMP_BINARY"
elif command -v wget >/dev/null 2>&1; then
    wget -qO "$TMP_BINARY" "$DOWNLOAD_URL"
else
    echo "Error: curl or wget is required to install taptap."
    exit 1
fi

chmod +x "$TMP_BINARY"

# 4. Move to system binary path
INSTALL_DIR="/usr/local/bin"
echo "Installing to ${INSTALL_DIR}..."

if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_BINARY" "${INSTALL_DIR}/${BINARY_NAME}"
else
    echo "Requires administrator privileges. Moving with sudo..."
    sudo mv "$TMP_BINARY" "${INSTALL_DIR}/${BINARY_NAME}"
fi

# 5. Download default soundpack
SOUNDPACKS_DIR="${HOME}/.taptap/soundpacks"
echo "Downloading default CherryMX Black soundpack..."
mkdir -p "${SOUNDPACKS_DIR}/cherrymx-black-abs"

CONFIG_URL="https://raw.githubusercontent.com/${GITHUB_REPO}/master/soundpacks/cherrymx-black-abs/config.json"
SOUND_URL="https://raw.githubusercontent.com/${GITHUB_REPO}/master/soundpacks/cherrymx-black-abs/sound.ogg"

if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$CONFIG_URL" -o "${SOUNDPACKS_DIR}/cherrymx-black-abs/config.json"
    curl -fsSL "$SOUND_URL" -o "${SOUNDPACKS_DIR}/cherrymx-black-abs/sound.ogg"
elif command -v wget >/dev/null 2>&1; then
    wget -qO "${SOUNDPACKS_DIR}/cherrymx-black-abs/config.json" "$CONFIG_URL"
    wget -qO "${SOUNDPACKS_DIR}/cherrymx-black-abs/sound.ogg" "$SOUND_URL"
fi

echo "=== taptap installed successfully! ==="
echo "Run 'taptap' in your terminal to start typing with mechanical sounds."
