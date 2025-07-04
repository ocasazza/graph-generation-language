API Reference
=============

This document provides reference information for the GGL library's programmatic interface.

Core Types
----------

Graph
~~~~~

The main graph data structure.

.. code-block:: rust

   pub struct Graph {
       pub nodes: HashMap<String, Node>,
       pub edges: HashMap<String, Edge>,
   }

**Methods:**

* ``new() -> Self`` - Creates a new empty graph
* ``add_node(&mut self, node: Node)`` - Adds a node to the graph
* ``add_edge(&mut self, edge: Edge)`` - Adds an edge to the graph
* ``remove_node(&mut self, id: &str)`` - Removes a node and its connected edges
* ``remove_edge(&mut self, id: &str)`` - Removes an edge
* ``get_node(&self, id: &str) -> Option<&Node>`` - Gets a node by ID
* ``get_edge(&self, id: &str) -> Option<&Edge>`` - Gets an edge by ID
* ``node_count(&self) -> usize`` - Returns the number of nodes
* ``edge_count(&self) -> usize`` - Returns the number of edges

Node
~~~~

Represents a graph node.

.. code-block:: rust

   pub struct Node {
       pub id: String,
       pub r#type: String,
       pub metadata: HashMap<String, MetadataValue>,
       pub x: f64,
       pub y: f64,
   }

**Methods:**

* ``new(id: String) -> Self`` - Creates a new node with the given ID
* ``with_type(self, node_type: String) -> Self`` - Sets the node type
* ``with_metadata(self, key: String, value: MetadataValue) -> Self`` - Adds metadata
* ``with_metadata_map(self, metadata: HashMap<String, MetadataValue>) -> Self`` - Adds multiple metadata entries
* ``with_position(self, x: f64, y: f64) -> Self`` - Sets the node position

Edge
~~~~

Represents a graph edge.

.. code-block:: rust

   pub struct Edge {
       pub id: String,
       pub source: String,
       pub target: String,
       pub r#type: String,
       pub metadata: HashMap<String, MetadataValue>,
   }

**Methods:**

* ``new(id: String, source: String, target: String) -> Self`` - Creates a new edge
* ``with_type(self, edge_type: String) -> Self`` - Sets the edge type
* ``with_metadata(self, key: String, value: MetadataValue) -> Self`` - Adds metadata
* ``with_metadata_map(self, metadata: HashMap<String, MetadataValue>) -> Self`` - Adds multiple metadata entries

MetadataValue
~~~~~~~~~~~~~

Represents attribute values.

.. code-block:: rust

   pub enum MetadataValue {
       String(String),
       Integer(i64),
       Float(f64),
       Boolean(bool),
   }

Parser Types
------------

Pattern
~~~~~~~

Represents a graph pattern used in rules.

.. code-block:: rust

   pub struct Pattern {
       pub nodes: Vec<NodeDeclaration>,
       pub edges: Vec<EdgeDeclaration>,
   }

NodeDeclaration
~~~~~~~~~~~~~~~

Represents a node declaration in a pattern.

.. code-block:: rust

   pub struct NodeDeclaration {
       pub id: String,
       pub node_type: Option<String>,
       pub attributes: HashMap<String, MetadataValue>,
   }

EdgeDeclaration
~~~~~~~~~~~~~~~

Represents an edge declaration in a pattern.

.. code-block:: rust

   pub struct EdgeDeclaration {
       pub id: String,
       pub source: String,
       pub target: String,
       pub directed: bool,
       pub attributes: HashMap<String, MetadataValue>,
   }

Rule System
-----------

Rule
~~~~

Represents a transformation rule.

.. code-block:: rust

   pub struct Rule {
       pub name: String,
       pub lhs: Pattern,
       pub rhs: Pattern,
   }

**Methods:**

* ``apply(&self, graph: &mut Graph, iterations: usize) -> Result<(), String>`` - Applies the rule to a graph

Graph Generators
----------------

Generator Functions
~~~~~~~~~~~~~~~~~~~

All generators follow the same signature:

.. code-block:: rust

   pub type GeneratorFn = fn(&HashMap<String, MetadataValue>) -> Result<Graph, String>;

**Available Generators:**

* ``generate_complete(params: &HashMap<String, MetadataValue>) -> Result<Graph, String>``
* ``generate_path(params: &HashMap<String, MetadataValue>) -> Result<Graph, String>``
* ``generate_cycle(params: &HashMap<String, MetadataValue>) -> Result<Graph, String>``
* ``generate_grid(params: &HashMap<String, MetadataValue>) -> Result<Graph, String>``
* ``generate_star(params: &HashMap<String, MetadataValue>) -> Result<Graph, String>``
* ``generate_tree(params: &HashMap<String, MetadataValue>) -> Result<Graph, String>``
* ``generate_barabasi_albert(params: &HashMap<String, MetadataValue>) -> Result<Graph, String>``

Generator Registry
~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   pub fn get_generator(name: &str) -> Option<GeneratorFn>

Returns the generator function for the given name, or ``None`` if not found.

Engine Interface
----------------

GGLEngine
~~~~~~~~~

The main engine for processing GGL programs.

.. code-block:: rust

   pub struct GGLEngine {
       // Internal state
   }

**Methods:**

* ``new() -> Self`` - Creates a new engine instance
* ``generate_from_ggl_native(&mut self, ggl_code: &str) -> Result<String, String>`` - Processes GGL code and returns JSON

Usage Examples
--------------

Creating Graphs Programmatically
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use graph_generation_lang::types::{Graph, Node, Edge, MetadataValue};
   use std::collections::HashMap;

   // Create a new graph
   let mut graph = Graph::new();

   // Add nodes
   let alice = Node::new("alice".to_string())
       .with_type("person".to_string())
       .with_metadata("age".to_string(), MetadataValue::Integer(30));

   let bob = Node::new("bob".to_string())
       .with_type("person".to_string())
       .with_metadata("age".to_string(), MetadataValue::Integer(25));

   graph.add_node(alice);
   graph.add_node(bob);

   // Add edge
   let friendship = Edge::new("friendship".to_string(), "alice".to_string(), "bob".to_string())
       .with_metadata("strength".to_string(), MetadataValue::Float(0.8));

   graph.add_edge(friendship);

Using Generators
~~~~~~~~~~~~~~~~

.. code-block:: rust

   use graph_generation_lang::generators::{get_generator};
   use graph_generation_lang::types::MetadataValue;
   use std::collections::HashMap;

   // Get the complete graph generator
   let generator = get_generator("complete").unwrap();

   // Set up parameters
   let mut params = HashMap::new();
   params.insert("nodes".to_string(), MetadataValue::Integer(5));
   params.insert("prefix".to_string(), MetadataValue::String("vertex".to_string()));

   // Generate the graph
   let graph = generator(&params).unwrap();

   println!("Generated graph with {} nodes and {} edges",
            graph.node_count(), graph.edge_count());

Processing GGL Code
~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use graph_generation_lang::GGLEngine;

   let mut engine = GGLEngine::new();

   let ggl_code = r#"
       graph example {
           node alice :person [name="Alice"];
           node bob :person [name="Bob"];
           edge friendship: alice -- bob;
       }
   "#;

   match engine.generate_from_ggl_native(ggl_code) {
       Ok(json) => println!("Generated JSON: {}", json),
       Err(e) => eprintln!("Error: {}", e),
   }

Working with Rules
~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use graph_generation_lang::rules::Rule;
   use graph_generation_lang::parser::{Pattern, NodeDeclaration};
   use std::collections::HashMap;

   // Create a simple rule
   let rule = Rule {
       name: "add_metadata".to_string(),
       lhs: Pattern {
           nodes: vec![NodeDeclaration {
               id: "N".to_string(),
               node_type: None,
               attributes: HashMap::new(),
           }],
           edges: vec![],
       },
       rhs: Pattern {
           nodes: vec![NodeDeclaration {
               id: "N".to_string(),
               node_type: None,
               attributes: {
                   let mut attrs = HashMap::new();
                   attrs.insert("processed".to_string(), MetadataValue::Boolean(true));
                   attrs
               },
           }],
           edges: vec![],
       },
   };

   // Apply to a graph
   let mut graph = Graph::new();
   graph.add_node(Node::new("test".to_string()));

   rule.apply(&mut graph, 1).unwrap();

Error Handling
--------------

All API functions that can fail return ``Result<T, String>`` where the error string contains a human-readable description of what went wrong.

Common error types:

* **Parse Errors**: Invalid GGL syntax
* **Generator Errors**: Invalid parameters or constraints
* **Rule Application Errors**: Pattern matching failures or invalid transformations
* **Graph Errors**: Invalid node/edge references

JSON Output Format
------------------

The engine outputs graphs in the following JSON format:

.. code-block:: json

   {
       "nodes": {
           "node_id": {
               "id": "node_id",
               "type": "node_type",
               "metadata": {
                   "key": "value"
               },
               "x": 0.0,
               "y": 0.0
           }
       },
       "edges": {
           "edge_id": {
               "id": "edge_id",
               "source": "source_node_id",
               "target": "target_node_id",
               "type": "edge_type",
               "metadata": {
                   "key": "value"
               }
           }
       }
   }

Thread Safety
-------------

The GGL library is designed to be thread-safe for read operations. However, concurrent modifications to the same graph instance are not supported. If you need to modify graphs from multiple threads, use appropriate synchronization mechanisms.

Performance Considerations
--------------------------

* **Graph Size**: Performance degrades with very large graphs (>10,000 nodes)
* **Rule Complexity**: Complex patterns with many nodes/edges are slower to match
* **Generator Parameters**: Some generators (like complete graphs) can create many edges
* **Memory Usage**: Graphs with rich metadata consume more memory

Best Practices
--------------

1. **Error Handling**: Always handle ``Result`` types properly
2. **Resource Management**: Large graphs should be dropped when no longer needed
3. **Parameter Validation**: Validate generator parameters before use
4. **Rule Testing**: Test rules on small graphs before applying to large ones
5. **JSON Parsing**: Use proper JSON libraries to parse output

Integration Examples
--------------------

Web Service Integration
~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use graph_generation_lang::GGLEngine;
   use serde_json::Value;

   fn process_ggl_request(ggl_code: &str) -> Result<Value, String> {
       let mut engine = GGLEngine::new();
       let json_str = engine.generate_from_ggl_native(ggl_code)?;
       serde_json::from_str(&json_str)
           .map_err(|e| format!("JSON parse error: {}", e))
   }

File Processing
~~~~~~~~~~~~~~~

.. code-block:: rust

   use std::fs;
   use graph_generation_lang::GGLEngine;

   fn process_ggl_file(filename: &str) -> Result<String, String> {
       let ggl_code = fs::read_to_string(filename)
           .map_err(|e| format!("File read error: {}", e))?;

       let mut engine = GGLEngine::new();
       engine.generate_from_ggl_native(&ggl_code)
   }

Batch Processing
~~~~~~~~~~~~~~~~

.. code-block:: rust

   use graph_generation_lang::GGLEngine;

   fn process_multiple_graphs(ggl_programs: Vec<&str>) -> Vec<Result<String, String>> {
       let mut engine = GGLEngine::new();

       ggl_programs.into_iter()
           .map(|code| engine.generate_from_ggl_native(code))
           .collect()
   }
