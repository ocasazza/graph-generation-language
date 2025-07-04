# Graph Generation Language (GGL)

A domain-specific language for creating and manipulating graphs through declarative syntax. GGL allows you to define graph structures, generate common graph topologies, and apply transformation rules to evolve graphs over time.

## Features

- **Declarative Syntax**: Define graphs using intuitive node and edge declarations
- **Built-in Generators**: Create common graph structures (complete, path, cycle, grid, star, tree, scale-free)
- **Transformation Rules**: Apply pattern-based rules to modify graph structure
- **Rich Attributes**: Support for typed nodes and edges with metadata
- **JSON Output**: Export graphs in standard JSON format

## Quick Start

### Installation

#### Option 1: Using Nix (Recommended)

If you have [Nix](https://nixos.org/download.html) and [direnv](https://direnv.net/) installed:

```bash
git clone https://github.com/ocasazza/graph-generation-language.git
cd graph-generation-language
direnv allow  # This will automatically set up the development environment
```

The Nix flake provides a complete development environment with:
- Rust toolchain with WASM support
- Python with Sphinx for documentation
- All necessary build tools
- Custom build scripts

Available commands in the Nix environment:
- `build` - Build the Rust project
- `build-wasm` - Build WASM target
- `test` - Run all tests
- `check` - Run cargo check
- `fmt` - Format code
- `clippy` - Run clippy linter
- `docs` - Build documentation
- `docs-serve` - Build and serve docs locally

#### Option 2: Manual Installation

```bash
git clone https://github.com/ocasazza/graph-generation-language.git
cd graph-generation-language
cargo build --release
```

For documentation building, you'll also need:
```bash
pip install -r docs/requirements.txt
```

### Basic Example

```ggl
graph social_network {
    // Define nodes with types and attributes
    node alice :person [name="Alice", age=30];
    node bob :person [name="Bob", age=25];
    node company :organization [name="Tech Corp"];

    // Create relationships
    edge friendship: alice -- bob [strength=0.8];
    edge employment: alice -> company [role="Engineer"];

    // Generate additional structure
    generate complete {
        nodes: 5;
        prefix: "user";
    }

    // Apply transformation rules
    rule add_metadata {
        lhs { node N :person; }
        rhs { node N :person [active=true]; }
    }

    apply add_metadata 10 times;
}
```

### Running

```bash
# Run with your GGL file
cargo run -- your_graph.ggl

# Run tests
cargo test
```

## Language Overview

### Node Declarations

```ggl
node simple_node;                           // Basic node
node typed_node :person;                    // Typed node
node detailed_node :person [                // Node with attributes
    name="Alice",
    age=30,
    active=true
];
```

### Edge Declarations

```ggl
edge friendship: alice -- bob;              // Undirected edge
edge reports_to: employee -> manager;       // Directed edge
edge weighted: a -- b [weight=0.5];         // Edge with attributes
```

### Graph Generators

| Generator | Description | Parameters |
|-----------|-------------|------------|
| `complete` | All nodes connected to all others | `nodes`, `prefix`, `directed` |
| `path` | Linear chain of nodes | `nodes`, `prefix`, `directed` |
| `cycle` | Circular chain of nodes | `nodes`, `prefix` |
| `grid` | 2D grid topology | `rows`, `cols`, `prefix`, `periodic` |
| `star` | Central hub with spokes | `nodes`, `prefix`, `directed` |
| `tree` | Hierarchical tree structure | `branching`, `depth`, `prefix` |
| `barabasi_albert` | Scale-free network | `nodes`, `edges_per_node`, `prefix` |

### Transformation Rules

```ggl
rule promote_employee {
    lhs { node N :employee; }               // Pattern to match
    rhs { node N :manager; }                // Replacement pattern
}

apply promote_employee 5 times;            // Apply rule
```

## Usage

See [EXAMPLES.md](EXAMPLES.md) for comprehensive examples including social networks, infrastructure networks, organizational hierarchies, and advanced patterns.

## Documentation

- **[Nix Development Environment](NIX_SETUP.md)**: Complete setup guide for Nix and direnv
- **[Examples & Usage Guide](EXAMPLES.md)**: Comprehensive examples and patterns
- **[Test Documentation](tests/README.md)**: Test structure and examples

## Project Structure

```
├── src/
│   ├── lib.rs          # Main library interface
│   ├── parser.rs       # GGL language parser
│   ├── types.rs        # Core graph data structures
│   ├── generators.rs   # Built-in graph generators
│   ├── rules.rs        # Rule application engine
│   └── ggl.pest        # Grammar definition
├── tests/              # Comprehensive test suite
├── EXAMPLES.md         # Detailed usage examples
└── README.md           # This file
```

## Language Grammar

GGL uses a clean, intuitive syntax:

```pest
// Basic structure
graph graph_name {
    statement*
}

// Statements
statement = node_decl | edge_decl | generate_stmt | rule_def | apply_rule

// Node declaration
node_decl = "node" ident node_type? attributes? ";"

// Edge declaration
edge_decl = "edge" ident? ":" ident edge_op ident attributes? ";"

// Rule definition
rule_def = "rule" ident "{" "lhs" pattern "rhs" pattern "}"
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Testing

The project includes comprehensive tests:

```bash
cargo test                    # Run all tests
cargo test --test rule_tests  # Run specific test suite
cargo test -- --nocapture    # Run with output
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [Pest](https://pest.rs/) parser
- Inspired by graph rewriting systems and domain-specific languages
- Supports common graph algorithms and network analysis patterns
