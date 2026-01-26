# Build Requirements

This page covers the system dependencies required to build Memista from source.

## Rust Toolchain

- Rust 1.56 or later
- Cargo package manager

Install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## System Dependencies

### For usearch crate

The `usearch` crate requires:

- C++ compiler with C++17 support
- SIMD instruction set support (SSE, AVX, NEON, etc.)

### For torch-sys crate (if enabled)

The `torch-sys` crate requires:

- C++ compiler with C++14 support
- CMake 3.12 or later
- Python 3.6 or later
- PyTorch C++ library (libtorch)

## Platform-Specific Instructions

### macOS

```bash
# Install Xcode command line tools
xcode-select --install

# Install dependencies via Homebrew
brew install cmake python
```

### Ubuntu/Debian

```bash
# Install build tools and dependencies
sudo apt-get update
sudo apt-get install build-essential cmake python3
```

### Windows

1. Install Visual Studio with C++ development tools
2. Install CMake from [cmake.org](https://cmake.org/download/)
3. Install Python from [python.org](https://www.python.org/downloads/)

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| Missing C++ compiler | Install the appropriate compiler for your platform |
| CMake not found | Install CMake from [cmake.org](https://cmake.org/download/) |
| Python not found | Install Python from [python.org](https://www.python.org/downloads/) |

### macOS Specific Issues

| Issue | Solution |
|-------|----------|
| Xcode command line tools not installed | Run `xcode-select --install` |
| Header files not found | Ensure Xcode is properly installed |

## Building the Project

Once all dependencies are installed:

```bash
# Clone the repository
git clone https://github.com/sokratis-xyz/memista.git
cd memista

# Build the project
cargo build

# Run the project
cargo run
```

## Verifying the Build

After building, run the tests:

```bash
cargo test
```

Start the server to verify everything works:

```bash
cargo run
```

Then test the API:

```bash
curl http://localhost:8083/openapi.json
```
