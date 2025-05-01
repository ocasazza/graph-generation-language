use wasm_bindgen::prelude::*;
use std::collections::HashMap;

mod parser;
mod generators;
mod rules;
mod types;

use crate::parser::{parse_ggl, GGLStatement};
use crate::generators::get_generator;
use crate::types::{Graph, Node, Edge};

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct GGLEngine {
    graph: Graph,
    rules: HashMap<String, rules::Rule>,
}

#[wasm_bindgen]
impl GGLEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        GGLEngine {
            graph: Graph::new(),
            rules: HashMap::new(),
        }
    }

    pub fn generate_from_ggl(&mut self, ggl_code: &str) -> Result<String, JsValue> {
        // Parse GGL code
        let statements = parse_ggl(ggl_code)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        // Reset graph state
        self.graph = Graph::new();
        self.rules.clear();

        // Process statements
        for stmt in statements {
            match stmt {
                GGLStatement::NodeDecl(node) => {
                    self.graph.add_node(Node::new(node.id.clone())
                        .with_type(node.node_type.unwrap_or_default())
                        .with_metadata_map(node.attributes));
                }
                GGLStatement::EdgeDecl(edge) => {
                    self.graph.add_edge(Edge::new(edge.id, edge.source, edge.target)
                        .with_metadata_map(edge.attributes));
                }
                GGLStatement::GenerateStmt(gen) => {
                    if let Some(generator) = get_generator(&gen.name) {
                        let generated = generator(&gen.params)
                            .map_err(|e| JsValue::from_str(&format!("Generator error: {}", e)))?;
                        
                        // Merge generated graph into current graph
                        for (_, node) in generated.nodes {
                            self.graph.add_node(node);
                        }
                        for (_, edge) in generated.edges {
                            self.graph.add_edge(edge);
                        }
                    } else {
                        return Err(JsValue::from_str(&format!("Unknown generator: {}", gen.name)));
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
                            .map_err(|e| JsValue::from_str(&format!("Rule application error: {}", e)))?;
                    } else {
                        return Err(JsValue::from_str(&format!("Unknown rule: {}", apply.rule_name)));
                    }
                }
            }
        }

        // Serialize final graph to JSON
        serde_json::to_string(&self.graph)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

// Initialize panic hook
#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}
