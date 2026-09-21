#!/bin/bash

set -euo pipefail

# Configuration
REPO="kiamazi/kateb"
BINARY_NAME="kateb"
VERSION="v2.0.2"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
  echo -e "${GREEN}ℹ${NC} $1"
}

log_error() {
  echo -e "${RED}✗${NC} $1" >&2
}

log_warn() {
  echo -e "${YELLOW}⚠${NC} $1"
}

# Detect OS and architecture
detect_platform() {
  local os arch

  # Detect OS
  case "$(uname -s)" in
    Linux*)
      os="linux"
      ;;
    Darwin*)
      os="darwin"
      ;;
    *)
      log_error "Unsupported OS: $(uname -s)"
      exit 1
      ;;
  esac

  # Detect architecture
  case "$(uname -m)" in
    x86_64)
      arch="x86_64"
      ;;
    aarch64)
      arch="arm64"
      ;;
    arm64)
      arch="arm64"
      ;;
    *)
      log_error "Unsupported architecture: $(uname -m)"
      exit 1
      ;;
  esac

  echo "${os}-${arch}"
}

# Determine bin directory based on privileges and OS
get_bin_dir() {
  local os="$1"

  if [ "$EUID" -eq 0 ]; then
    # Running with sudo/as root
    echo "/usr/local/bin"
  else
    # Running as regular user
    echo "${HOME}/.local/bin"
  fi
}

# # Get the latest release version
# get_latest_version() {
#   local version
#   version=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)

#   if [ -z "$version" ]; then
#     log_error "Failed to fetch latest version"
#     exit 1
#   fi

#   echo "$version"
# }

# Determine the binary name based on platform
get_binary_name() {
  local platform="$1"

  case "$platform" in
    linux-x86_64)
      echo "kateb-linux"
      ;;
    linux-arm64)
      echo "kateb-linux-arm64"
      ;;
    darwin-x86_64)
      echo "kateb-darwin"
      ;;
    darwin-arm64)
      echo "kateb-darwin-arm64"
      ;;
    *)
      log_error "Unknown platform: $platform"
      exit 1
      ;;
  esac
}

# Download file with retry logic
download_file() {
  local url="$1"
  local output="$2"
  local max_attempts=3
  local attempt=1

  while [ $attempt -le $max_attempts ]; do
    log_info "Downloading from $url (attempt $attempt/$max_attempts)..."

    if curl -fsSL "$url" -o "$output"; then
      return 0
    fi

    attempt=$((attempt + 1))
    if [ $attempt -le $max_attempts ]; then
      sleep 2
    fi
  done

  log_error "Failed to download after $max_attempts attempts"
  return 1
}

# Verify checksum
verify_checksum() {
  local binary="$1"
  local checksums_file="checksums.json"

  log_info "Verifying checksum..."

  if ! command -v jq &> /dev/null; then
    log_warn "jq not found, skipping checksum verification"
    return 0
  fi

  # Extract the binary name without path for JSON lookup
  local binary_name
  binary_name=$(basename "$binary")

  # Get expected checksum from JSON
  local expected_checksum
  expected_checksum=$(jq -r ".\"$binary_name\"" "$checksums_file" 2>/dev/null)

  if [ "$expected_checksum" = "null" ] || [ -z "$expected_checksum" ]; then
    log_warn "Checksum not found for $binary_name, skipping verification"
    return 0
  fi

  # Calculate actual checksum
  local actual_checksum
  actual_checksum=$(sha256sum "$binary" | awk '{print $1}')

  if [ "$expected_checksum" = "$actual_checksum" ]; then
    log_info "Checksum verified ✓"
    return 0
  else
    log_error "Checksum mismatch!"
    log_error "Expected: $expected_checksum"
    log_error "Actual:   $actual_checksum"
    return 1
  fi
}

# Create bin directory if it doesn't exist
ensure_bin_dir() {
  local bin_dir="$1"

  if [ ! -d "$bin_dir" ]; then
    log_info "Creating directory: $bin_dir"
    mkdir -p "$bin_dir"
  fi
}

# Check if bin directory is in PATH
check_path() {
  local bin_dir="$1"
  local euid="$2"

  if [[ ":$PATH:" != *":$bin_dir:"* ]]; then
    log_warn "$bin_dir is not in your PATH"

    if [ "$euid" -eq 0 ]; then
      log_info "Since you ran with sudo, $bin_dir should already be in PATH"
    else
      log_info "Add this line to your shell config (~/.bashrc, ~/.zshrc, etc.):"
      echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
  fi
}

# Main installation
main() {
  log_info "Installing kateb..."

  # Detect platform
  local platform
  platform=$(detect_platform)
  log_info "Detected platform: $platform"

  # Determine bin directory
  local bin_dir
  bin_dir=$(get_bin_dir "${platform%-*}")

  if [ "$EUID" -eq 0 ]; then
    log_info "Running with elevated privileges"
  fi
  log_info "Installation directory: $bin_dir"

  # # Get latest version
  # local version
  # version=$(get_latest_version)
  # log_info "Latest version: $version"

  # Get binary name
  local binary_name
  binary_name=$(get_binary_name "$platform")
  log_info "Binary name: $binary_name"

  # Create temporary directory
  local temp_dir
  temp_dir=$(mktemp -d)
  trap "rm -rf $temp_dir" EXIT

  cd "$temp_dir"

  # Download binary
  # local download_url="https://github.com/${REPO}/releases/download/${version}/${binary_name}"
  local download_url="https://github.com/${REPO}/releases/download/${VERSION}/${binary_name}"
  download_file "$download_url" "$binary_name"

  # Download checksums
  # local checksums_url="https://github.com/${REPO}/releases/download/${version}/checksums.json"
  local checksums_url="https://github.com/${REPO}/releases/download/${VERSION}/checksums.json"
  if ! download_file "$checksums_url" "checksums.json"; then
    log_warn "Could not download checksums.json, proceeding without verification"
  else
    verify_checksum "$binary_name" || exit 1
  fi

  # Make binary executable
  chmod +x "$binary_name"

  # Ensure bin directory exists
  ensure_bin_dir "$bin_dir"

  # Copy binary to bin directory
  log_info "Installing to $bin_dir/$BINARY_NAME"
  cp "$binary_name" "$bin_dir/$BINARY_NAME"

  # Set proper permissions
  chmod 755 "$bin_dir/$BINARY_NAME"

  # Check PATH
  check_path "$bin_dir" "$EUID"

  log_info "Installation complete! ✓"
  log_info "Run 'kateb version' to verify the installation"
}

main "$@"
