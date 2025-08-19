# Memista Build Requirements

## Rust Toolchain
- Rust 1.56 or later
- Cargo package manager

## System Dependencies

### For torch-sys crate
The `torch-sys` crate requires:
- C++ compiler with C++14 support
- CMake 3.12 or later
- Python 3.6 or later
- PyTorch C++ library (libtorch)

### For usearch crate
The `usearch` crate requires:
- C++ compiler with C++17 support
- SIMD instruction set support (SSE, AVX, NEON, etc.)

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
- Install Visual Studio with C++ development tools
- Install CMake
- Install Python

## Troubleshooting

### Common Issues
1. **Missing C++ compiler**: Install the appropriate compiler for your platform
2. **CMake not found**: Install CMake from https://cmake.org/download/
3. **Python not found**: Install Python from https://www.python.org/downloads/

### macOS Specific Issues
1. **Xcode command line tools not installed**: Run `xcode-select --install`
2. **Header files not found**: Ensure Xcode is properly installed

## Building the Project
Once all dependencies are installed:

```bash
# Clone the repository
git clone <repository-url>
cd memista

# Build the project
cargo build

# Run the project
cargo run
```