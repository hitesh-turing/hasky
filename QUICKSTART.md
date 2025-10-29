# Hashy - Quick Start Guide

## Steps 1-2 Complete! ✅

You now have a fully functional Rust CLI boilerplate with:

### ✨ What's Built

1. **Project Scaffold** (Step 1)
   - Clean Rust CLI skeleton with cargo
   - Proper project structure
   - Dual MIT/Apache-2.0 licensing
   - Comprehensive README

2. **Argument Parsing** (Step 2)
   - Solid CLI ergonomics using clap
   - Global flags: `--version`, `--quiet`, `--verbose`
   - Primary `hash` command with flags: `--algo`, `--text`, `--file`
   - Help text and examples

### 🎯 Acceptance Criteria Met

✅ `hashy --help` prints a basic banner  
✅ `hashy hash --help` shows flags and examples  
✅ All tests passing (14 tests total)  
✅ Docker-ready with multi-stage build  
✅ CI/CD pipeline configured

## Quick Commands

```bash
# Build the project
cargo build

# Run tests
cargo test

# Show help
cargo run -- --help

# Show hash command help
cargo run -- hash --help

# Try the hash command (placeholder output for now)
cargo run -- hash --text "hello"
cargo run -- --verbose hash --algo sha256 --text "hello"
cargo run -- --quiet hash --file example.txt

# Using Make (recommended)
make build          # Build release
make test           # Run tests
make check          # Format, lint, and test
make docker-build   # Build Docker image
make help           # Show all available commands
```

## Docker Commands

```bash
# Build the image
docker build -t hashy .

# Run with help
docker run --rm hashy --help

# Hash text
docker run --rm hashy hash --text "hello world"

# Hash a file from host
docker run --rm -v $(pwd):/data hashy hash --file /data/myfile.txt

# Using docker-compose
docker-compose up hashy
docker-compose run hashy hash --help
```

## Project Structure

```
hashy/
├── src/
│   └── main.rs              # Main CLI entry point
├── tests/
│   └── cli_tests.rs         # Integration tests
├── .github/
│   └── workflows/
│       └── ci.yml           # GitHub Actions CI
├── Cargo.toml               # Dependencies & metadata
├── Dockerfile               # Multi-stage Docker build
├── docker-compose.yml       # Docker compose config
├── Makefile                 # Common development tasks
├── README.md                # Full documentation
├── CONTRIBUTING.md          # Contribution guidelines
├── CHANGELOG.md             # Version history
├── LICENSE-MIT              # MIT license
├── LICENSE-APACHE           # Apache 2.0 license
└── .gitignore               # Git ignore rules
```

## What's Implemented

### CLI Structure

```rust
hashy [GLOBAL_FLAGS] <COMMAND> [COMMAND_FLAGS]

Global Flags:
  -v, --verbose    Enable verbose output
  -q, --quiet      Suppress non-error output
  -V, --version    Print version
  -h, --help       Print help

Commands:
  hash             Compute hash of input
```

### Hash Command

```rust
hashy hash [OPTIONS]

Options:
  -a, --algo <ALGORITHM>    Hash algorithm [default: sha256]
  -t, --text <TEXT>         Text to hash
  -f, --file <FILE>         File to hash
```

## Testing Strategy

### Unit Tests (5 tests in main.rs)
- CLI structure validation
- Default algorithm selection
- Custom algorithm handling
- Flag parsing (verbose, quiet)

### Integration Tests (9 tests in cli_tests.rs)
- Help command output
- Version display
- Hash command help
- Text hashing
- Algorithm selection
- Verbose mode
- Quiet mode
- Conflicting flags validation

### Run Tests
```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_hash_with_text

# Test coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --verbose
```

## Development Workflow

### 1. Make Changes
```bash
# Edit files in src/
vim src/main.rs
```

### 2. Check Your Work
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Or use make for all checks
make check
```

### 3. Build & Run
```bash
# Debug build
cargo build
cargo run -- hash --text "test"

# Release build (optimized)
cargo build --release
./target/release/hashy --help
```

## Next Steps (Steps 3-19)

Ready to continue? Here's what's next:

### Step 3: First Algorithm & Text Input
- Add SHA-256 using RustCrypto (sha2 crate)
- Implement actual hashing of text
- Validate against known test vectors

**Add to Cargo.toml:**
```toml
[dependencies]
sha2 = "0.10"
hex = "0.4"
```

### Step 4: File Input (Single File)
- Stream file in chunks (64 KiB)
- Error handling via anyhow/thiserror
- Match sha256sum output

### Step 5: Multiple Algorithms
- SHA1, SHA512, BLAKE3, MD5
- Enum over digest trait
- Algorithm selection

Continue following the 19-step plan in the README!

## Troubleshooting

### Tests Failing?
```bash
# Clean and rebuild
cargo clean
cargo test
```

### Docker Build Issues?
```bash
# Check Docker is running
docker ps

# Build with no cache
docker build --no-cache -t hashy .
```

### Linter Warnings?
```bash
# Auto-fix formatting
cargo fmt

# See clippy suggestions
cargo clippy
```

## Resources

- [Clap Documentation](https://docs.rs/clap/)
- [RustCrypto](https://github.com/RustCrypto)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## Questions?

- Check the README.md for full documentation
- Read CONTRIBUTING.md for development guidelines
- Open an issue on GitHub

---

**You're all set!** The foundation is solid and ready for Steps 3-19. 🚀

