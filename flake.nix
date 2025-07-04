{
  description = "Graph Generation Language - A Rust project with WASM support and Sphinx documentation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        pythonEnv = pkgs.python3.withPackages (ps: with ps; [
          sphinx
          sphinx-rtd-theme
          myst-parser
          pip
        ]);

        # Build scripts
        testScript = pkgs.writeShellScriptBin "test" ''
          echo "Running Rust tests..."
          cargo test
        '';

        docsScript = pkgs.writeShellScriptBin "docs" ''
          echo "Building documentation..."
          cd docs
          make html-dev
        '';

        docsServeScript = pkgs.writeShellScriptBin "docs-serve" ''
          echo "Building and serving documentation..."
          cd docs
          make serve
        '';

        buildScript = pkgs.writeShellScriptBin "build" ''
          echo "Building Rust project..."
          cargo build
        '';

        buildWasmScript = pkgs.writeShellScriptBin "build-wasm" ''
          echo "Building WASM target..."
          wasm-pack build --target web --out-dir pkg
        '';

        checkScript = pkgs.writeShellScriptBin "check" ''
          echo "Running cargo check..."
          cargo check
        '';

        fmtScript = pkgs.writeShellScriptBin "fmt" ''
          echo "Formatting code..."
          cargo fmt
        '';

        clippyScript = pkgs.writeShellScriptBin "clippy" ''
          echo "Running clippy..."
          cargo clippy -- -D warnings
        '';

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            wasm-pack

            # Python for documentation
            pythonEnv

            # Build tools
            gnumake

            # Development tools
            git

            # Custom scripts
            testScript
            docsScript
            docsServeScript
            buildScript
            buildWasmScript
            checkScript
            fmtScript
            clippyScript
          ];

          shellHook = ''
            echo "🦀 Graph Generation Language Development Environment"
            echo ""
            echo "Available commands:"
            echo "  build       - Build the Rust project"
            echo "  build-wasm  - Build WASM target"
            echo "  test        - Run all tests"
            echo "  check       - Run cargo check"
            echo "  fmt         - Format code"
            echo "  clippy      - Run clippy linter"
            echo "  docs        - Build documentation"
            echo "  docs-serve  - Build and serve docs locally"
            echo ""
            echo "Rust version: $(rustc --version)"
            echo "Python version: $(python --version)"
            echo ""
          '';

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";
        };

        # Default package (optional)
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "graph-generation-lang";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = with pkgs; [
            rustToolchain
          ];
        };
      });
}
