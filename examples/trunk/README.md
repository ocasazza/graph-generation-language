# GGL WASM Basic Example

This example demonstrates how to use the Graph Generation Language (GGL) in a web browser using WebAssembly and Trunk.

## Prerequisites

- Rust with `wasm32-unknown-unknown` target installed
- Trunk installed (`cargo install trunk`)

## Running the Example

From the repository root:

```bash
# Start the development server
just serve

# Or run directly from this directory
cd examples/basic
trunk serve
```

Then open http://127.0.0.1:8080 in your browser.

## Building for Production

```bash
# From repository root
just build

# Or from this directory
trunk build --release
```

## Features

- Interactive GGL code editor
- Real-time graph generation
- Example templates
- JSON output formatting
- Error handling and reporting

## Project Structure

- `src/lib.rs` - WASM entry point and DOM manipulation
- `index.html` - Demo web page
- `Cargo.toml` - WASM-specific dependencies
- `Trunk.toml` - Trunk build configuration

## How it Works

1. Trunk compiles the Rust code to WebAssembly
2. The WASM module loads and initializes the GGL engine
3. JavaScript in the HTML page calls the WASM functions
4. GGL code is parsed and executed in the browser
5. Results are displayed as formatted JSON
