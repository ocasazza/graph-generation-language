# Nix Development Environment Setup

This project includes a complete Nix flake configuration that provides a reproducible development environment with all necessary tools and dependencies.

## Prerequisites

1. **Install Nix** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
   ```

2. **Install direnv** (recommended for automatic environment loading):
   ```bash
   # On macOS with Homebrew
   brew install direnv

   # On Linux
   sudo apt install direnv  # Ubuntu/Debian
   sudo dnf install direnv  # Fedora
   ```

3. **Configure direnv** in your shell:
   ```bash
   # For bash, add to ~/.bashrc:
   eval "$(direnv hook bash)"

   # For zsh, add to ~/.zshrc:
   eval "$(direnv hook zsh)"

   # For fish, add to ~/.config/fish/config.fish:
   direnv hook fish | source
   ```

## Quick Start

1. **Clone the repository**:
   ```bash
   git clone https://github.com/ocasazza/graph-generation-language.git
   cd graph-generation-language
   ```

2. **Allow direnv** (if using direnv):
   ```bash
   direnv allow
   ```

   This will automatically load the Nix environment when you enter the directory.

3. **Or manually enter the Nix shell**:
   ```bash
   nix develop
   ```

## Available Commands

Once in the development environment, you have access to these custom commands:

### Build Commands
- `build` - Build the Rust project
- `build-wasm` - Build WASM target using wasm-pack
- `check` - Run cargo check for quick compilation verification

### Testing Commands
- `test` - Run all Rust tests
- `clippy` - Run clippy linter for code quality checks
- `fmt` - Format code using rustfmt

### Documentation Commands
- `docs` - Build Sphinx documentation
- `docs-serve` - Build and serve documentation locally on port 8000

## What's Included

The Nix environment provides:

### Rust Toolchain
- **Rust 1.88.0** with stable toolchain
- **cargo** for package management and building
- **rustfmt** for code formatting
- **clippy** for linting
- **rust-src** for IDE support
- **WASM target** (wasm32-unknown-unknown) for WebAssembly builds
- **wasm-pack** for WASM packaging

### Python Environment
- **Python 3.13.4** with pip
- **Sphinx 8.2.3** for documentation generation
- **sphinx-rtd-theme** for Read the Docs styling
- **myst-parser** for Markdown support in Sphinx

### Development Tools
- **git** for version control
- **make** for running documentation builds
- All necessary build dependencies

### Environment Variables
- `RUST_SRC_PATH` - Points to Rust source for IDE integration
- `RUST_BACKTRACE=1` - Enables detailed error backtraces
- `RUST_LOG=debug` - Sets debug logging level
- `CARGO_INCREMENTAL=1` - Enables incremental compilation

## Usage Examples

### Development Workflow
```bash
# Enter the project directory (with direnv)
cd graph-generation-language

# Check code quality
clippy

# Format code
fmt

# Run tests
test

# Build the project
build

# Build documentation and serve locally
docs-serve
```

### Manual Nix Commands
```bash
# Enter development shell
nix develop

# Run a single command in the environment
nix develop --command test

# Build the project package
nix build

# Check flake configuration
nix flake check
```

## Troubleshooting

### Direnv Not Working
If direnv isn't automatically loading:
1. Make sure direnv is installed and hooked into your shell
2. Run `direnv allow` in the project directory
3. Check that `.envrc` exists and contains `use flake`

### Nix Build Issues
If you encounter build issues:
1. Ensure you have Nix flakes enabled
2. Try `nix flake update` to update dependencies
3. Clear the build cache with `nix store gc`

### Missing Dependencies
The flake should handle all dependencies automatically. If something is missing:
1. Check that you're in the Nix environment (`nix develop`)
2. Verify the flake.lock file is present
3. Try rebuilding with `nix develop --rebuild`

## IDE Integration

### VS Code
For optimal VS Code integration:
1. Install the Rust Analyzer extension
2. Make sure you're in the Nix environment when starting VS Code
3. The `RUST_SRC_PATH` environment variable will be set automatically

### Other IDEs
Most Rust-compatible IDEs should work well with the Nix environment. Ensure:
1. The IDE is started from within the Nix shell
2. Rust Analyzer or similar language server is configured
3. The Rust toolchain path points to the Nix-provided tools

## Contributing

When contributing to this project:
1. Use the provided Nix environment for consistency
2. Run `fmt` before committing to ensure consistent formatting
3. Run `clippy` to catch potential issues
4. Run `test` to verify all tests pass
5. Update documentation with `docs` if needed

The Nix environment ensures all contributors have identical development setups, reducing "works on my machine" issues.
