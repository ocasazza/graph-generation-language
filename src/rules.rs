use std::collections::{HashMap, HashSet};
use crate::types::{Graph, Node, Edge, MetadataValue};
use crate::parser::{Pattern, NodeDeclaration, EdgeDeclaration};

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub lhs: Pattern,
    pub rhs: Pattern,
}

impl Rule {
    pub fn apply(&self, graph: &mut Graph, iterations: usize) -> Result<(), String> {
        for _ in 0..iterations {
            let matches = self.find_matches(graph)?;
            if matches.is_empty() {
                break;
            }
            
            for m in matches {
                self.apply_transformation(graph, &m)?;
            }
        }
        
        Ok(())
    }
    
    fn find_matches(&self, graph: &Graph) -> Result<Vec<Match>, String> {
        let mut matches = Vec::new();
        let mut visited = HashSet::new();
        
        // For each node in the graph, try to match the LHS pattern starting from it
        for (node_id, _) in &graph.nodes {
            if visited.contains(node_id) {
                continue;
            }
            
            if let Some(m) = self.match_pattern_from_node(graph, node_id, &self.lhs)? {
                // Add all matched nodes to visited set
                visited.extend(m.node_mapping.values().cloned());
                matches.push(m);
            }
        }
        
        Ok(matches)
    }
    
    fn match_pattern_from_node(&self, graph: &Graph, start_node: &str, pattern: &Pattern) -> Result<Option<Match>, String> {
        let mut node_mapping = HashMap::new();
        let mut edge_mapping = HashMap::new();
        
        // Try to match the first node in the pattern to the start node
        if pattern.nodes.is_empty() {
            return Ok(Some(Match { node_mapping, edge_mapping }));
        }
        
        let first_pattern_node = &pattern.nodes[0];
        if !self.node_matches(graph, start_node, first_pattern_node)? {
            return Ok(None);
        }
        
        node_mapping.insert(first_pattern_node.id.clone(), start_node.to_string());
        
        // Try to extend the match to the rest of the pattern
        if self.extend_match(graph, pattern, &mut node_mapping, &mut edge_mapping)? {
            Ok(Some(Match { node_mapping, edge_mapping }))
        } else {
            Ok(None)
        }
    }
    
    fn extend_match(
        &self,
        graph: &Graph,
        pattern: &Pattern,
        node_mapping: &mut HashMap<String, String>,
        edge_mapping: &mut HashMap<String, String>
    ) -> Result<bool, String> {
        // Match remaining nodes
        for pattern_node in pattern.nodes.iter().skip(1) {
            let mut found_match = false;
            
            // Try each unmapped graph node
            for (graph_node_id, _) in &graph.nodes {
                if node_mapping.values().any(|v| v == graph_node_id) {
                    continue;
                }
                
                if self.node_matches(graph, graph_node_id, pattern_node)? {
                    node_mapping.insert(pattern_node.id.clone(), graph_node_id.clone());
                    found_match = true;
                    break;
                }
            }
            
            if !found_match {
                return Ok(false);
            }
        }
        
        // Match edges
        for pattern_edge in &pattern.edges {
            let mut found_match = false;
            
            // Get the mapped source and target nodes
            let source = node_mapping.get(&pattern_edge.source)
                .ok_or_else(|| "Invalid source node in pattern".to_string())?;
            let target = node_mapping.get(&pattern_edge.target)
                .ok_or_else(|| "Invalid target node in pattern".to_string())?;
            
            // Look for a matching edge in the graph
            for (graph_edge_id, graph_edge) in &graph.edges {
                if edge_mapping.values().any(|v| v == graph_edge_id) {
                    continue;
                }
                
                if graph_edge.source == *source && graph_edge.target == *target {
                    edge_mapping.insert(pattern_edge.id.clone(), graph_edge_id.clone());
                    found_match = true;
                    break;
                }
            }
            
            if !found_match {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    fn node_matches(&self, graph: &Graph, graph_node_id: &str, pattern_node: &NodeDeclaration) -> Result<bool, String> {
        let graph_node = graph.get_node(graph_node_id)
            .ok_or_else(|| format!("Node {} not found in graph", graph_node_id))?;
        
        // Check node type if specified
        if let Some(ref node_type) = pattern_node.node_type {
            if graph_node.r#type != *node_type {
                return Ok(false);
            }
        }
        
        // Check attributes if specified
        for (key, value) in &pattern_node.attributes {
            match graph_node.metadata.get(key) {
                Some(graph_value) if graph_value == value => continue,
                _ => return Ok(false),
            }
        }
        
        Ok(true)
    }
    
    fn apply_transformation(&self, graph: &mut Graph, m: &Match) -> Result<(), String> {
        // Create new nodes from RHS pattern
        let mut new_nodes = HashMap::new();
        
        for node in &self.rhs.nodes {
            let node_id = if let Some(mapped_id) = m.node_mapping.get(&node.id) {
                // This is a preserved node from LHS
                mapped_id.clone()
            } else {
                // This is a new node
                node.id.clone()
            };
            
            if !m.node_mapping.values().any(|v| v == &node_id) {
                // Only create the node if it doesn't already exist
                let mut new_node = Node::new(node_id.clone());
                if let Some(ref node_type) = node.node_type {
                    new_node = new_node.with_type(node_type.clone());
                }
                for (key, value) in &node.attributes {
                    new_node = new_node.with_metadata(key.clone(), value.clone());
                }
                new_nodes.insert(node_id.clone(), new_node);
            }
        }
        
        // Remove nodes that are in LHS but not in RHS
        for (pattern_id, graph_id) in &m.node_mapping {
            if !self.rhs.nodes.iter().any(|n| &n.id == pattern_id) {
                graph.remove_node(graph_id);
            }
        }
        
        // Add new nodes
        for (_, node) in new_nodes {
            graph.add_node(node);
        }
        
        // Create new edges from RHS pattern
        for edge in &self.rhs.edges {
            let source = if let Some(s) = m.node_mapping.get(&edge.source) {
                s.clone()
            } else {
                edge.source.clone()
            };
            
            let target = if let Some(t) = m.node_mapping.get(&edge.target) {
                t.clone()
            } else {
                edge.target.clone()
            };
            
            let mut new_edge = Edge::new(edge.id.clone(), source, target);
            for (key, value) in &edge.attributes {
                new_edge = new_edge.with_metadata(key.clone(), value.clone());
            }
            graph.add_edge(new_edge);
        }
        
        Ok(())
    }
}

#[derive(Debug)]
struct Match {
    node_mapping: HashMap<String, String>,  // Pattern node ID -> Graph node ID
    edge_mapping: HashMap<String, String>,  // Pattern edge ID -> Graph edge ID
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NodeDeclaration, EdgeDeclaration, Pattern};

    #[test]
    fn test_simple_rule() {
        // Create a rule that replaces a single node with two connected nodes
        let rule = Rule {
            name: "split".to_string(),
            lhs: Pattern {
                nodes: vec![
                    NodeDeclaration {
                        id: "A".to_string(),
                        node_type: None,
                        attributes: HashMap::new(),
                    }
                ],
                edges: vec![],
            },
            rhs: Pattern {
                nodes: vec![
                    NodeDeclaration {
                        id: "B1".to_string(),
                        node_type: None,
                        attributes: HashMap::new(),
                    },
                    NodeDeclaration {
                        id: "B2".to_string(),
                        node_type: None,
                        attributes: HashMap::new(),
                    }
                ],
                edges: vec![
                    EdgeDeclaration {
                        id: "e".to_string(),
                        source: "B1".to_string(),
                        target: "B2".to_string(),
                        directed: true,
                        attributes: HashMap::new(),
                    }
                ],
            },
        };

        // Create a test graph
        let mut graph = Graph::new();
        graph.add_node(Node::new("n1"));

        // Apply the rule
        rule.apply(&mut graph, 1).unwrap();

        // Check the result
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_typed_node_rule() {
        // Create a rule that matches nodes by type
        let mut type_attrs = HashMap::new();
        type_attrs.insert("type".to_string(), MetadataValue::String("A".to_string()));

        let rule = Rule {
            name: "type_match".to_string(),
            lhs: Pattern {
                nodes: vec![
                    NodeDeclaration {
                        id: "N".to_string(),
                        node_type: Some("A".to_string()),
                        attributes: HashMap::new(),
                    }
                ],
                edges: vec![],
            },
            rhs: Pattern {
                nodes: vec![
                    NodeDeclaration {
                        id: "N".to_string(),
                        node_type: Some("B".to_string()),
                        attributes: HashMap::new(),
                    }
                ],
                edges: vec![],
            },
        };

        // Create a test graph
        let mut graph = Graph::new();
        graph.add_node(Node::new("n1").with_type("A"));
        graph.add_node(Node::new("n2").with_type("C"));

        // Apply the rule
        rule.apply(&mut graph, 1).unwrap();

        // Check that only the type A node was transformed
        assert!(graph.get_node("n1").unwrap().r#type == "B");
        assert!(graph.get_node("n2").unwrap().r#type == "C");
    }
}
