# Contributing to Hashy

Thank you for your interest in contributing to Hashy! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)
- Docker (optional, for containerized testing)

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/yourusername/hashy.git
cd hashy
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## Development Workflow

### Running the Application

```bash
# Run with cargo
cargo run -- hash --text "hello"

# Or use make
make run ARGS="hash --text hello"
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_hash_with_text

# Or use make
make test
```

### Code Quality

Before submitting a PR, ensure your code passes all checks:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run all checks
make check
```

### Docker Development

```bash
# Build Docker image
make docker-build

# Test Docker image
make docker-test

# Or use docker-compose
docker-compose up hashy
docker-compose run hashy-dev
```

## Code Style

- Follow the official Rust style guide
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Write meaningful commit messages

## Testing Guidelines

- Write unit tests for all new functions
- Write integration tests for CLI commands
- Ensure all tests pass before submitting a PR
- Aim for high test coverage (80%+ for new code)
- Use descriptive test names that explain what is being tested

Example:
```rust
#[test]
fn test_hash_with_sha256_matches_known_vector() {
    // Test implementation
}
```

## Commit Guidelines

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(hash): add BLAKE3 algorithm support

Add BLAKE3 hashing algorithm with streaming support.
Includes tests and documentation updates.

Closes #42
```

```
fix(cli): handle empty file input gracefully

Previously, empty files would cause a panic.
Now returns appropriate error message.

Fixes #38
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and checks (`make check`)
5. Commit your changes with descriptive messages
6. Push to your fork
7. Open a Pull Request

### PR Checklist

- [ ] Code builds without errors
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] New tests added for new functionality
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated (for significant changes)
- [ ] Commit messages follow guidelines

## Adding New Features

When adding new features:

1. Check the [Roadmap](README.md#roadmap) to see if it aligns with project goals
2. Open an issue to discuss the feature first (for large changes)
3. Follow the incremental plan outlined in the README
4. Write tests before implementation (TDD approach recommended)
5. Update documentation

## Bug Reports

When filing a bug report, include:

- Hashy version (`hashy --version`)
- Operating system and version
- Rust version (`rustc --version`)
- Minimal reproducible example
- Expected behavior
- Actual behavior
- Any error messages or logs

Use the `--verbose` flag to get more detailed output:
```bash
hashy --verbose hash --text "test"
```

## Feature Requests

When requesting a feature:

- Check if it's already on the roadmap
- Explain the use case
- Provide examples of how it would work
- Consider implementation complexity

## Security

If you discover a security issue, please email security@example.com instead of using the issue tracker.

## Questions?

- Open an issue for general questions
- Check existing issues for common problems
- Read the documentation in the README

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

