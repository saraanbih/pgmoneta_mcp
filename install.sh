#!/bin/sh
## Copyright (C) 2026 The pgmoneta community
##
## This program is free software: you can redistribute it and/or modify
## it under the terms of the GNU General Public License as published by
## the Free Software Foundation, either version 3 of the License, or
## (at your option) any later version.
##
## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
## GNU General Public License for more details.
##
## You should have received a copy of the GNU General Public License
## along with this program. If not, see <https://www.gnu.org/licenses/>.
# Usage: curl -fsSL https://raw.githubusercontent.com/pgmoneta/pgmoneta_mcp/main/install.sh | sh
# Override install directory: INSTALL_DIR=/usr/local/bin sh install.sh
set -e

REPO="${REPO:-pgmoneta/pgmoneta_mcp}"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

if command -v curl >/dev/null 2>&1; then
    fetch()    { curl -fsSL "$1"; }
    fetch_to() { curl -fsSL -o "$2" "$1"; }
elif command -v wget >/dev/null 2>&1; then
    fetch()    { wget -qO- "$1"; }
    fetch_to() { wget -qO "$2" "$1"; }
else
    echo "error: curl or wget is required" >&2; exit 1
fi

command -v tar >/dev/null 2>&1 || { echo "error: tar is required" >&2; exit 1; }

# Detect OS and architecture
case "$(uname -s)" in
    Linux)  OS="linux" ;;
    Darwin) OS="darwin" ;;
    *)      echo "error: unsupported OS: $(uname -s)" >&2; exit 1 ;;
esac

case "$(uname -m)" in
    x86_64)        ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *)             echo "error: unsupported architecture: $(uname -m)" >&2; exit 1 ;;
esac

# On Linux, detect glibc vs musl
if [ "$OS" = "linux" ]; then
    if ldd --version 2>&1 | grep -qi musl; then LIBC="musl"; else LIBC="gnu"; fi
    TARGET="${ARCH}-unknown-linux-${LIBC}"
else
    TARGET="${ARCH}-apple-darwin"
fi

echo "Platform: ${TARGET}"

# Resolve latest release tag
LATEST_VERSION=$(fetch "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
[ -n "$LATEST_VERSION" ] || { echo "error: could not fetch latest release" >&2; exit 1; }
echo "Latest:   ${LATEST_VERSION}"

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

download_asset() {
    tag="$1"
    candidate_asset="pgmoneta-mcp-${tag}-${TARGET}.tar.gz"
    candidate_url="https://github.com/${REPO}/releases/download/${tag}/${candidate_asset}"

    echo "Downloading ${candidate_asset}..."
    if fetch_to "$candidate_url" "${TMP}/${candidate_asset}"; then
        VERSION="$tag"
        ASSET="$candidate_asset"
        return 0
    fi

    return 1
}

if ! download_asset "$LATEST_VERSION"; then
    echo "warning: latest release does not contain ${TARGET} asset, searching older releases..." >&2
    FOUND=0
    for tag in $(fetch "https://api.github.com/repos/${REPO}/releases?per_page=50" \
        | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p'); do
        [ "$tag" = "$LATEST_VERSION" ] && continue
        if download_asset "$tag"; then
            FOUND=1
            break
        fi
    done

    [ "$FOUND" -eq 1 ] || {
        echo "error: no prebuilt asset found for target ${TARGET} in recent releases of ${REPO}" >&2
        echo "hint: publish release binaries for this target, or set REPO=<owner/repo> that has them" >&2
        exit 1
    }
fi

echo "Using:    ${VERSION}"

tar -xzf "${TMP}/${ASSET}" -C "$TMP" || { echo "error: could not extract ${ASSET}" >&2; exit 1; }
for b in pgmoneta-mcp-server pgmoneta-mcp-admin pgmoneta-mcp-client pgmoneta-mcp-inspector; do
    [ -f "${TMP}/${b}" ] || { echo "error: binary not found in archive: ${b}" >&2; exit 1; }
done

mkdir -p "$INSTALL_DIR"
[ -w "$INSTALL_DIR" ] || { echo "error: ${INSTALL_DIR} is not writable - try sudo or set INSTALL_DIR" >&2; exit 1; }

for b in pgmoneta-mcp-server pgmoneta-mcp-admin pgmoneta-mcp-client pgmoneta-mcp-inspector; do
    cp "${TMP}/${b}" "${INSTALL_DIR}/${b}"
    chmod +x "${INSTALL_DIR}/${b}"
done

echo "Installed:"
echo "  ${INSTALL_DIR}/pgmoneta-mcp-server"
echo "  ${INSTALL_DIR}/pgmoneta-mcp-admin"
echo "  ${INSTALL_DIR}/pgmoneta-mcp-client"
echo "  ${INSTALL_DIR}/pgmoneta-mcp-inspector"

case ":${PATH}:" in
    *":${INSTALL_DIR}:"*) ;;
    *) echo "warning: ${INSTALL_DIR} is not in your PATH - add: export PATH=\"${INSTALL_DIR}:\$PATH\"" ;;
esac

echo ""
echo "Run 'pgmoneta-mcp-client --help' to get started."
echo "Run 'pgmoneta-mcp-server --help' to see server options."