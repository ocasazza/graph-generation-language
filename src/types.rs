use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MetadataValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub r#type: String,
    pub metadata: HashMap<String, MetadataValue>,
    pub x: f64,
    pub y: f64,
}

impl Node {
    pub fn new(id: String) -> Self {
        Node {
            id,
            r#type: String::new(),
            metadata: HashMap::new(),
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn with_type(mut self, node_type: String) -> Self {
        self.r#type = node_type;
        self
    }

    pub fn with_metadata(mut self, key: String, value: MetadataValue) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_metadata_map(mut self, metadata: HashMap<String, MetadataValue>) -> Self {
        self.metadata.extend(metadata);
        self
    }

    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub r#type: String,
    pub metadata: HashMap<String, MetadataValue>,
}

impl Edge {
    pub fn new(id: String, source: String, target: String) -> Self {
        Edge {
            id,
            source,
            target,
            r#type: String::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_type(mut self, edge_type: String) -> Self {
        self.r#type = edge_type;
        self
    }

    pub fn with_metadata(mut self, key: String, value: MetadataValue) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_metadata_map(mut self, metadata: HashMap<String, MetadataValue>) -> Self {
        self.metadata.extend(metadata);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: HashMap<String, Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge.id.clone(), edge);
    }

    pub fn remove_node(&mut self, id: &str) {
        self.nodes.remove(id);
        // Remove any edges connected to this node
        self.edges.retain(|_, edge| edge.source != id && edge.target != id);
    }

    pub fn remove_edge(&mut self, id: &str) {
        self.edges.remove(id);
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_builder() {
        let node = Node::new("test".to_string())
            .with_type("person".to_string())
            .with_metadata("age".to_string(), MetadataValue::Integer(30))
            .with_position(10.0, 20.0);

        assert_eq!(node.id, "test");
        assert_eq!(node.r#type, "person");
        assert_eq!(node.x, 10.0);
        assert_eq!(node.y, 20.0);
        assert_eq!(node.metadata.len(), 1);
        assert!(matches!(node.metadata.get("age"), Some(MetadataValue::Integer(30))));
    }

    #[test]
    fn test_edge_builder() {
        let edge = Edge::new("e1".to_string(), "n1".to_string(), "n2".to_string())
            .with_type("friend".to_string())
            .with_metadata("weight".to_string(), MetadataValue::Float(1.0));

        assert_eq!(edge.id, "e1");
        assert_eq!(edge.source, "n1");
        assert_eq!(edge.target, "n2");
        assert_eq!(edge.r#type, "friend");
        assert_eq!(edge.metadata.len(), 1);
        assert!(matches!(edge.metadata.get("weight"), Some(MetadataValue::Float(1.0))));
    }

    #[test]
    fn test_graph_operations() {
        let mut graph = Graph::new();

        // Add nodes
        let node1 = Node::new("n1".to_string());
        let node2 = Node::new("n2".to_string());
        graph.add_node(node1);
        graph.add_node(node2);

        // Add edge
        let edge = Edge::new("e1".to_string(), "n1".to_string(), "n2".to_string());
        graph.add_edge(edge);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);

        // Remove node and verify connected edge is removed
        graph.remove_node("n1");
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);
    }
}
