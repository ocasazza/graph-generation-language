{ inputs, ... }:
{
  perSystem =
    {
      config,
      self',
      pkgs,
      lib,
      ...
    }:
    {
      devShells.default = pkgs.mkShell {
        name = "graph_generation_language-shell";
        inputsFrom = [
          self'.devShells.rust
          config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
        ];
        packages = with pkgs; [
          just
          nixd # Nix language server
          bacon
          # WASM development tools
          trunk # Modern WASM bundler and dev server
          rustup # Need rustup to install WASM target
          # Browser testing tools
          chromedriver
          geckodriver
        ];
        shellHook = ''
          # Install WASM target if not already installed
          if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
            echo "Installing wasm32-unknown-unknown target..."
            rustup target add wasm32-unknown-unknown
          fi
        '';
      };
    };
}
