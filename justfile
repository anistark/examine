# Examine - Project Analysis Tool
# Just commands for common development tasks

# Default
default:
    @just --list

# Build
build:
    cargo build --release

# Run all tests
test:
    cargo test

# Run tests and show coverage
test-coverage:
    cargo tarpaulin

# Run the simple usage example
run-simple:
    cargo run --example simple_usage

# Run the CLI tool example with current directory
run-cli:
    cargo run --example cli_tool analyze .

# Run the CLI tool example with a specific path
run-cli-path path=".":
    cargo run --example cli_tool analyze {{path}}

# Format code using rustfmt
format:
    cargo fmt

# Check formatting without making changes
format-check:
    cargo fmt -- --check

# Run clippy linter
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run clippy and fix automatically fixable issues
lint-fix:
    cargo clippy --fix

# Check code without building
check:
    cargo check

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# Generate documentation
docs:
    cargo doc --open

# Generate documentation without opening
docs-build:
    cargo doc

# Release preparation
prepare-release: clean format lint test docs-build
    @echo "✅ Ready for release!"

# Size analysis of the binary
size:
    cargo build --release
    @echo "Release binary size:"
    @ls -lh target/release/examine 2>/dev/null || echo "Binary not found. Run 'just build-release' first."

# Show project statistics
stats:
    @echo "✨ Project Statistics:"
    @echo "Lines of code:"
    @find src -name "*.rs" -exec wc -l {} + | tail -1
    @echo "Dependencies:"
    @cargo tree --depth 1
    @echo "Examples:"
    @ls examples/*.rs | wc -l
    @echo "Test coverage:"
    @cargo tarpaulin --count 2>/dev/null | grep -E "^[0-9]+\.[0-9]+%" || echo "Run 'just test-coverage' first"

# Validate the Cargo.toml and lock file
validate:
    cargo check --locked
    cargo verify-project

# Publish to crates.io
publish:
    cargo publish
