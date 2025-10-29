# Hashy 🔐

A fast, flexible CLI for hashing with multiple algorithms built in Rust.

## Features

- 🚀 **Fast**: Built in Rust for maximum performance
- 🔧 **Flexible**: Support for multiple hashing algorithms
- 🐳 **Dockerized**: Easy deployment with Docker
- ✅ **Tested**: Comprehensive unit and integration tests
- 📦 **Lightweight**: Minimal dependencies and small binary size

## Installation

### From Source

```bash
cargo install --path .
```

### Using Docker

```bash
docker build -t hashy .
docker run --rm hashy --help
```

## Usage

### Basic Commands

```bash
# Show help
hashy --help

# Show version
hashy --version

# Hash a text string (default: SHA-256)
hashy hash --text "hello world"

# Hash with a specific algorithm
hashy hash --algo blake3 --text "hello world"

# Hash a file
hashy hash --file myfile.txt

# Verbose output
hashy --verbose hash --text "hello"

# Quiet mode (errors only)
hashy --quiet hash --text "hello"
```

### Global Flags

- `--verbose, -v`: Enable verbose output
- `--quiet, -q`: Suppress non-error output
- `--version`: Print version information
- `--help`: Print help information

### Hash Command

```bash
hashy hash [OPTIONS]

Options:
  -a, --algo <ALGORITHM>    Hash algorithm to use [default: sha256]
  -t, --text <TEXT>         Text to hash directly
  -f, --file <FILE>         File to hash
  -h, --help                Print help
```

## Development

### Prerequisites

- Rust 1.75+ (recommended)
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_hash_with_text
```

### Docker Development

```bash
# Build the Docker image
docker build -t hashy:dev .

# Run tests in Docker
docker run --rm hashy:dev hash --help

# Hash text using Docker
docker run --rm hashy:dev hash --text "hello world"

# Hash a file using Docker (mount volume)
docker run --rm -v $(pwd):/data hashy:dev hash --file /data/myfile.txt
```

## Roadmap

### ✅ Completed (Steps 1-2)
- [x] Project scaffolding
- [x] CLI argument parsing with clap
- [x] Basic command structure
- [x] Global flags (--verbose, --quiet, --version)
- [x] Docker support
- [x] Unit and integration tests

### 🚧 Upcoming
- [ ] SHA-256 implementation (Step 3)
- [ ] File input with streaming (Step 4)
- [ ] Multiple algorithms (SHA1, SHA512, BLAKE3, MD5) (Step 5)
- [ ] STDIN support (Step 6)
- [ ] Output formatting options (Step 7)
- [ ] Batch mode (Step 8)
- [ ] Verification mode (Step 9)
- [ ] Performance & concurrency (Step 10)
- [ ] Progress bars (Step 11)
- [ ] Config file support (Step 12)
- [ ] Security defaults (Step 13)
- [ ] HMAC support (Step 14)
- [ ] Directory & manifest generation (Step 15)
- [ ] Testing, fuzzing, and benches (Step 16)
- [ ] Packaging & distribution (Step 17)
- [ ] Documentation & examples (Step 18)
- [ ] Telemetry-free analytics & crash-proofing (Step 19)

## Testing Strategy

The project includes:
- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test CLI behavior end-to-end
- **Property tests**: (Coming in Step 16)
- **Fuzz tests**: (Coming in Step 16)
- **Benchmarks**: (Coming in Step 16)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
- [RustCrypto](https://github.com/RustCrypto) - Cryptographic algorithms (coming soon)

