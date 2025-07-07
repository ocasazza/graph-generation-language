//! # Graph Generation Language (GGL)
//!
//! GGL is a domain-specific language for creating and manipulating graphs through declarative syntax.
//!
//! ## Overview
//!
//! GGL allows you to:
//!
//! * Define graph structures using intuitive node and edge declarations
//! * Generate common graph topologies with built-in generators
//! * Apply transformation rules to modify graph structure
//! * Export graphs in standard JSON format
//!
//! ## Quick Example
//!
//! ```ggl
//! graph social_network {
//!     // Define nodes with types and attributes
//!     node alice :person [name="Alice", age=30];
//!     node bob :person [name="Bob", age=25];
//!
//!     // Create relationships
//!     edge friendship: alice -- bob [strength=0.8];
//!
//!     // Generate additional structure
//!     generate complete {
//!         nodes: 5;
//!         prefix: "user";
//!     }
//!
//!     // Apply transformation rules
//!     rule add_metadata {
//!         lhs { node N :person; }
//!         rhs { node N :person [active=true]; }
//!     }
//!
//!     apply add_metadata 10 times;
//! }
//! ```
//!
//! ## Features
//!
//! * **Declarative Syntax**: Define graphs using intuitive node and edge declarations
//! * **Built-in Generators**: Create common graph structures (complete, path, cycle, grid, star, tree, scale-free)
//! * **Transformation Rules**: Apply pattern-based rules to modify graph structure
//! * **Rich Attributes**: Support for typed nodes and edges with metadata
//! * **JSON Output**: Export graphs in standard JSON format
//!
//! ## Getting Started
//!
//! ### Installation
//!
//! Prerequisites:
//! * Rust 1.70 or later
//! * Cargo (comes with Rust)
//!
//! Building from source:
//! ```bash
//! git clone https://github.com/ocasazza/graph-generation-language.git
//! cd graph-generation-language
//! cargo build --release
//! ```
//!
//! ### Your First Graph
//!
//! Create a simple graph:
//! ```ggl
//! graph hello_world {
//!     node alice;
//!     node bob;
//!     edge friendship: alice -- bob;
//! }
//! ```
//!
//! ## Modules
//!
//! * [`types`] - Core data structures for nodes, edges, and graphs
//! * [`parser`] - GGL language parser and AST definitions
//! * [`generators`] - Built-in graph generators for common topologies
//! * [`rules`] - Transformation rule engine for graph manipulation

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub mod generators;
pub mod parser;
pub mod rules;
pub mod types;

use crate::generators::get_generator;
use crate::parser::{parse_ggl, GGLStatement};
use crate::types::{Edge, Graph, Node};

/// Sets up panic hook for better error reporting in WebAssembly environments.
///
/// This function should be called once when initializing the WASM module to ensure
/// that panics are properly reported to the JavaScript console.
#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// The main GGL engine for parsing and executing GGL programs.
///
/// `GGLEngine` maintains the state of a graph and associated transformation rules,
/// allowing you to build complex graph structures through GGL programs.
///
/// # Examples
///
/// ```rust
/// use graph_generation_language::GGLEngine;
///
/// let mut engine = GGLEngine::new();
/// let ggl_code = r#"
///     graph example {
///         node alice :person;
///         node bob :person;
///         edge: alice -- bob;
///     }
/// "#;
///
/// let result = engine.generate_from_ggl_native(ggl_code).unwrap();
/// println!("Generated graph: {}", result);
/// ```
#[wasm_bindgen]
pub struct GGLEngine {
    graph: Graph,
    rules: HashMap<String, rules::Rule>,
}

impl Default for GGLEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl GGLEngine {
    /// Creates a new GGL engine with an empty graph and no rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use graph_generation_language::GGLEngine;
    ///
    /// let engine = GGLEngine::new();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        GGLEngine {
            graph: Graph::new(),
            rules: HashMap::new(),
        }
    }

    /// Parses and executes a GGL program, returning the resulting graph as JSON.
    ///
    /// This is the WebAssembly-compatible version that returns JavaScript-compatible
    /// error types. For native Rust usage, prefer [`generate_from_ggl_native`].
    ///
    /// # Arguments
    ///
    /// * `ggl_code` - A string containing the GGL program to execute
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// - `Ok(String)` - JSON representation of the generated graph
    /// - `Err(JsValue)` - Error message wrapped for JavaScript compatibility
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const engine = new GGLEngine();
    /// const gglCode = `
    ///     graph simple {
    ///         node a;
    ///         node b;
    ///         edge: a -- b;
    ///     }
    /// `;
    ///
    /// try {
    ///     const result = engine.generate_from_ggl(gglCode);
    ///     console.log("Graph:", JSON.parse(result));
    /// } catch (error) {
    ///     console.error("Error:", error);
    /// }
    /// ```
    ///
    /// [`generate_from_ggl_native`]: #method.generate_from_ggl_native
    pub fn generate_from_ggl(&mut self, ggl_code: &str) -> Result<String, JsValue> {
        self.generate_from_ggl_native(ggl_code)
            .map_err(|e| JsValue::from_str(&e))
    }
}

/// Native implementation for testing and non-WASM usage
impl GGLEngine {
    /// Parses and executes a GGL program, returning the resulting graph as JSON.
    ///
    /// This is the native Rust version that returns standard Rust error types.
    /// For WebAssembly usage, use [`generate_from_ggl`] instead.
    ///
    /// # Arguments
    ///
    /// * `ggl_code` - A string containing the GGL program to execute
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// - `Ok(String)` - JSON representation of the generated graph
    /// - `Err(String)` - Error message describing what went wrong
    ///
    /// # Examples
    ///
    /// ```rust
    /// use graph_generation_language::GGLEngine;
    ///
    /// let mut engine = GGLEngine::new();
    /// let ggl_code = r#"
    ///     graph social_network {
    ///         node alice :person [name="Alice", age=30];
    ///         node bob :person [name="Bob", age=25];
    ///         edge friendship: alice -- bob [strength=0.8];
    ///
    ///         generate complete {
    ///             nodes: 3;
    ///             prefix: "user";
    ///         }
    ///
    ///         rule add_metadata {
    ///             lhs { node N :person; }
    ///             rhs { node N :person [active=true]; }
    ///         }
    ///
    ///         apply add_metadata 5 times;
    ///     }
    /// "#;
    ///
    /// match engine.generate_from_ggl_native(ggl_code) {
    ///     Ok(json) => println!("Generated graph: {}", json),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    ///
    /// # Processing Steps
    ///
    /// The method processes GGL programs in the following order:
    ///
    /// 1. **Parse** - Convert GGL source code into an abstract syntax tree
    /// 2. **Reset State** - Clear any existing graph and rules
    /// 3. **Process Statements** - Execute each statement in order:
    ///    - Node declarations create new nodes with types and attributes
    ///    - Edge declarations create connections between nodes
    ///    - Generate statements invoke built-in graph generators
    ///    - Rule definitions register transformation patterns
    ///    - Apply statements execute transformation rules
    /// 4. **Serialize** - Convert the final graph to JSON format
    ///
    /// # Error Handling
    ///
    /// This method can return errors for various reasons:
    /// - **Parse errors**: Invalid GGL syntax
    /// - **Generator errors**: Invalid generator parameters or unknown generators
    /// - **Rule errors**: Pattern matching failures or transformation errors
    /// - **Serialization errors**: JSON conversion failures
    ///
    /// [`generate_from_ggl`]: #method.generate_from_ggl
    pub fn generate_from_ggl_native(&mut self, ggl_code: &str) -> Result<String, String> {
        // Parse GGL code
        let statements = parse_ggl(ggl_code).map_err(|e| format!("Parse error: {}", e))?;

        // Reset graph state
        self.graph = Graph::new();
        self.rules.clear();

        // Process statements
        for stmt in statements {
            match stmt {
                GGLStatement::NodeDecl(node) => {
                    self.graph.add_node(
                        Node::new(node.id.clone())
                            .with_type(node.node_type.unwrap_or_default())
                            .with_metadata_map(node.attributes),
                    );
                }
                GGLStatement::EdgeDecl(edge) => {
                    self.graph.add_edge(
                        Edge::new(edge.id, edge.source, edge.target)
                            .with_metadata_map(edge.attributes),
                    );
                }
                GGLStatement::GenerateStmt(gen) => {
                    if let Some(generator) = get_generator(&gen.name) {
                        let generated = generator(&gen.params)
                            .map_err(|e| format!("Generator error: {}", e))?;

                        // Merge generated graph into current graph
                        for (_, node) in generated.nodes {
                            self.graph.add_node(node);
                        }
                        for (_, edge) in generated.edges {
                            self.graph.add_edge(edge);
                        }
                    } else {
                        return Err(format!("Unknown generator: {}", gen.name));
                    }
                }
                GGLStatement::RuleDefStmt(rule_def) => {
                    let rule = rules::Rule {
                        name: rule_def.name.clone(),
                        lhs: rule_def.lhs,
                        rhs: rule_def.rhs,
                    };
                    self.rules.insert(rule_def.name, rule);
                }
                GGLStatement::ApplyRuleStmt(apply) => {
                    if let Some(rule) = self.rules.get(&apply.rule_name) {
                        rule.apply(&mut self.graph, apply.iterations)
                            .map_err(|e| format!("Rule application error: {}", e))?;
                    } else {
                        return Err(format!("Unknown rule: {}", apply.rule_name));
                    }
                }
            }
        }

        // Serialize final graph to JSON
        serde_json::to_string(&self.graph).map_err(|e| format!("Serialization error: {}", e))
    }
}

// Initialize panic hook
#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}
