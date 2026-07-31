# Contributing

Contributions are welcome! Here's how to get started.

## Development Setup

1. Follow the [Building from Source](guides/building-from-source.md) guide
2. Make your changes
3. Run `cargo build` to verify compilation
4. Submit a pull request

## Documentation

Documentation is built with [mdBook](https://rust-lang.github.io/mdBook/).

```bash
# Install mdBook
cargo install mdbook

# Serve locally with live reload
mdbook serve docs --open

# Or use the convenience script
./scripts/serve-docs.sh

# One-time build
mdbook build docs
```

## Code Style

- Follow Rust edition 2021 conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common issues

## Project Structure

All crates live in the workspace root. See [Architecture Overview](reference/architecture.md) for the crate map.

## Pull Requests

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Open a PR against `main`
