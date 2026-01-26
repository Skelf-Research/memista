# Getting Started

Welcome to Memista! This section will help you get up and running with Memista, whether you want to use it as a library in your Rust application or run it as a standalone HTTP server.

## Choose Your Path

### Use as a Library

If you want to embed Memista directly in your Rust application:

1. Add Memista to your `Cargo.toml`
2. Create a database pool and application state
3. Use the library functions directly or start the HTTP server

[Learn more about library usage](../guide/library-usage.md)

### Use as an HTTP Server

If you want to run Memista as a standalone service:

1. Install Memista via Cargo or build from source
2. Configure environment variables
3. Start the server and interact via HTTP API

[Learn more about HTTP server usage](../guide/http-server.md)

## Prerequisites

Before installing Memista, ensure you have:

- Rust 1.56 or later
- C++ compiler with C++17 support
- CMake 3.12 or later

See [Build Requirements](requirements.md) for detailed platform-specific instructions.

## Quick Links

- [Installation](installation.md) - Detailed installation instructions
- [Quick Start](quickstart.md) - Get running in 5 minutes
- [Build Requirements](requirements.md) - System dependencies
