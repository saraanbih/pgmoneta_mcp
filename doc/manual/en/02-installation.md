\newpage

# Installation

Install pgmoneta_mcp by building from source or using release artifacts.

## Install with the script

The repository includes an `install.sh` script that downloads the latest release
binaries for your platform and installs them into `~/.local/bin` by default.

```bash
curl -fsSL https://raw.githubusercontent.com/pgmoneta/pgmoneta_mcp/main/install.sh | sh
```

You can override the install directory if you want the binaries somewhere else:

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/pgmoneta/pgmoneta_mcp/main/install.sh | sh
```

The script installs these binaries:

- `pgmoneta-mcp-server`
- `pgmoneta-mcp-admin`
- `pgmoneta-mcp-client`
- `pgmoneta-mcp-inspector`

## Build from source

```bash
cargo build
cargo build --release
```

The build produces the server, client, and inspector binaries in the Cargo target
directory.

## Install from release artifacts

Download the release archive for your platform, unpack it, and place the binaries
on your `PATH`.

## External requirements

- Rust toolchain for building from source
- PostgreSQL and pgmoneta for runtime use
- A configured pgmoneta admin user and master key for pgmoneta_mcp


