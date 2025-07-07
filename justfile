default:
    @just --list

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
    pre-commit run --all-files

# Run 'cargo run' on the project
run *ARGS:
    cargo run {{ARGS}}

# Run 'bacon' to run the project (auto-recompiles)
watch *ARGS:
	bacon --job run -- -- {{ ARGS }}

# Run all tests
test *ARGS:
    cargo test {{ARGS}}

# Run tests with bacon (auto-recompiles and re-runs tests)
test-watch *ARGS:
    bacon --job test -- {{ARGS}}

# Run specific test file
test-file FILE *ARGS:
    cargo test --test {{FILE}} {{ARGS}}

# Run tests with coverage
test-coverage:
    cargo test --all-features

# Serve WASM demo with Trunk (development server with hot reload)
serve:
    cd examples/basic && trunk serve

# Build WASM for production with Trunk
build:
    cd examples/basic && trunk build --release

# Clean Trunk build artifacts
clean:
    cd examples/basic && trunk clean
    rm -rf examples/basic/dist
