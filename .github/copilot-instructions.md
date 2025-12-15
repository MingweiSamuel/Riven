# Copilot Instructions for Riven

## Project Overview

Riven is a Rust library for the [Riot Games API](https://developer.riotgames.com/). The project's goals are **speed**, **reliability**, and **maintainability**. Riven handles rate limits and large requests with ease, providing an asynchronous, thread-safe interface for interacting with the Riot API.

Key features:
- Fast, asynchronous, thread-safe API client
- Automatic retry logic for failed requests
- Support for all Riot API endpoints
- Auto-generated data structures from Riot API schema
- WebAssembly (Wasm) compilation support
- Strictly forbids unsafe code

## Project Structure

This is a Cargo workspace with the following structure:

- **`riven/`** - Main Rust library crate
  - `src/` - Hand-written and generated Rust source code
  - `srcgen/` - NodeJS code generator using doT.js templates
  - `tests/` - Integration tests
  - `examples/` - Example code (e.g., proxy example)
- **`.github/workflows/`** - CI/CD workflows (CI, publish)
- **Root scripts** - `test.bash`, `test-full.bash`, `test-min-deps.bash`, `test-wasm.bash`, `doc.bash`

## Code Generation

The project uses **NodeJS** to generate Rust code from the Riot API schema:
- Code generation is in `riven/srcgen/`
- Templates use [doT.js](https://olado.github.io/doT/index.html)
- Run code generation: `node riven/srcgen` from repository root
- Requires NodeJS and dependencies installed via `npm ci` in `riven/srcgen/`

## Development Workflow

### Prerequisites

1. **Rust** - MSRV is 1.71.1 (specified in `riven/Cargo.toml`)
2. **API Key** - Place Riot API key in `RGAPI_KEY` environment variable or `apikey.txt` file for testing
3. **NodeJS** - Required for code generation (srcgen)

### Building

```bash
# Standard build
cargo build

# Build with all features
cargo build --all-features

# Build with nightly features
cargo build --features riven/nightly,riven/deny-unknown,riven/eserde,riven/__proxy
```

### Testing

```bash
# Run tests (requires RGAPI_KEY)
cargo test --features riven/nightly,riven/tracing,riven/eserde,riven/deny-unknown

# Run tests with script
./test.bash

# Test WebAssembly compilation
wasm-pack test --node -- --features nightly,deny-unknown,eserde
```

**Important**: Tests require a valid Riot API key in `RGAPI_KEY` environment variable or `apikey.txt` file.

### Linting and Formatting

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run Clippy
cargo clippy --all-features --all-targets -- -D warnings
```

**Formatting configuration** is in `rustfmt.toml` with specific settings like:
- `group_imports = "StdExternalCrate"`
- `imports_granularity = "Module"`
- `use_field_init_shorthand = true`

## Code Standards

1. **No unsafe code** - The project uses `#![forbid(unsafe_code)]`
2. **Follow rustfmt configuration** - Use the settings in `rustfmt.toml`
3. **Pass Clippy with warnings as errors** - All code must pass Clippy checks
4. **Maintain MSRV compatibility** - Code must compile on Rust 1.71.1+
5. **Use async/await** - The library is fully asynchronous using Tokio (or gloo-timers for Wasm)
6. **Thread-safe design** - All code should be thread-safe

## Feature Flags

When writing code, be aware of these feature flags:

- **`nightly`** - Enables nightly-only optimizations (parking_lot)
- **`default-tls`** / **`native-tls`** / **`rustls-tls`** - TLS backend selection
- **`tracing`** - Use tracing instead of log for diagnostics
- **`metrics`** - Enable metrics collection
- **`eserde`** - Enhanced serde support
- **`deny-unknown`** - Error on unknown fields/enum variants (useful for strict validation)
- **`__proxy`** - Required for proxy example

## Testing Guidelines

1. **Integration tests** are in `riven/tests/`
2. Tests require a valid Riot API key
3. Use appropriate feature flags when testing:
   - `nightly` for nightly features
   - `tracing` for span-based logging
   - `deny-unknown` for strict validation
   - `eserde` for enhanced serialization
4. **Wasm tests** must be run with `wasm-pack`
5. Tests should handle rate limits and API errors gracefully

## CI/CD

The project uses GitHub Actions with these workflows:

- **CI** (`ci.yml`) - Runs on push, PR, and daily schedule
  - Checks code with stable and nightly Rust
  - Runs tests with multiple feature combinations
  - Runs lints (fmt, clippy)
  - Builds documentation
  - Verifies MSRV
  - Tests Wasm compilation
- **Publish** (`publish.yml`) - Publishes to crates.io

## Documentation

- **Main docs**: https://docs.rs/riven/
- **API reference**: Auto-generated from code
- Documentation is built with `cargo docs-rs -p riven`
- Doc warnings are treated as errors in CI

## Dependencies and Versioning

- **Semantic versioning** - With caveats due to Riot API changes
- **Breaking changes** in models/endpoints increment MINOR version (not MAJOR)
- Keep dependencies up-to-date but maintain MSRV compatibility
- Key dependencies: `reqwest`, `serde`, `tokio`, `parking_lot`

## Common Tasks

### Adding a new endpoint or model
1. Update `riven/srcgen/` templates if needed
2. Run `node riven/srcgen` to regenerate code
3. Test with appropriate feature flags
4. Ensure code passes fmt and clippy

### Fixing a bug
1. Write a test that reproduces the issue
2. Make minimal changes to fix
3. Verify fix with tests
4. Ensure no clippy warnings

### Updating dependencies
1. Check MSRV compatibility
2. Update `Cargo.toml`
3. Run full test suite
4. Verify Wasm compilation still works

## Best Practices

1. **Minimize external dependencies** - Only add when truly necessary
2. **Write async code** - Use async/await, avoid blocking
3. **Handle errors properly** - Return `Result<T>` or `Result<Option<T>>`
4. **Document public APIs** - All public items should have doc comments
5. **Test edge cases** - Especially API errors, rate limits, missing data
6. **Consider Wasm compatibility** - Avoid platform-specific code without cfg gates
7. **Use type safety** - Leverage Rust's type system for correctness
8. **Follow existing patterns** - Match the style of existing code

## Getting Help

- Review existing code in `riven/src/` for patterns
- Check tests in `riven/tests/` for usage examples
- Refer to Riot API documentation: https://developer.riotgames.com/
- Check issues: https://github.com/MingweiSamuel/Riven/issues
