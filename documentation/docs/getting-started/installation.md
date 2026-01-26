# Installation

This guide covers the different ways to install Memista.

## As a Library

Add Memista to your `Cargo.toml`:

```toml
[dependencies]
memista = "0.1"
```

Then run:

```bash
cargo build
```

## As a CLI Application

### From crates.io

The simplest way to install Memista:

```bash
cargo install memista
```

After installation, start the server:

```bash
memista
```

### From Source

Clone the repository and build:

```bash
# Clone the repository
git clone https://github.com/sokratis-xyz/memista.git
cd memista

# Build in release mode
cargo build --release

# Run the binary
./target/release/memista
```

### Development Build

For development with debug symbols:

```bash
cargo build
cargo run
```

## Verifying Installation

After starting the server, verify it's running:

```bash
curl http://localhost:8083/openapi.json
```

You should receive an OpenAPI specification JSON response.

## Configuration

By default, Memista uses these settings:

| Setting | Default Value |
|---------|---------------|
| Host | `127.0.0.1` |
| Port | `8083` |
| Database Path | `memista.db` |
| Log Level | `info` |

To customize these settings, create a `.env` file or set environment variables. See [Configuration](../guide/configuration.md) for details.

## Next Steps

- [Quick Start](quickstart.md) - Insert and search your first chunks
- [Configuration](../guide/configuration.md) - Customize Memista settings
- [Build Requirements](requirements.md) - Troubleshoot build issues
