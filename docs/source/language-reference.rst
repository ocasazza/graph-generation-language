Language Reference
==================

This document provides a complete reference for the Graph Generation Language (GGL) syntax and semantics.

Program Structure
-----------------

Every GGL program consists of a single graph definition:

.. code-block:: ggl

   graph [graph_name] {
       statement*
   }

The graph name is optional. If omitted, the graph will be unnamed.

Statements
----------

A GGL program consists of several types of statements:

* Node declarations
* Edge declarations
* Generator statements
* Rule definitions
* Rule applications

Comments
--------

GGL supports both single-line and multi-line comments:

.. code-block:: ggl

   // Single-line comment

   /*
    * Multi-line comment
    * can span multiple lines
    */

Node Declarations
-----------------

Syntax
~~~~~~

.. code-block:: ggl

   node identifier [: type] [attributes] ;

Examples
~~~~~~~~

.. code-block:: ggl

   node alice;                              // Simple node
   node bob :person;                        // Typed node
   node server :machine [cpu=8, ram=16];   // Node with attributes

Node Types
~~~~~~~~~~

Node types are optional identifiers that categorize nodes:

.. code-block:: ggl

   node web_server :server;
   node database :server;
   node user :person;

Attributes
~~~~~~~~~~

Attributes are key-value pairs enclosed in square brackets:

.. code-block:: ggl

   node alice :person [
       name="Alice Johnson",
       age=30,
       active=true,
       score=98.5
   ];

Supported attribute types:

* **String**: ``"text value"``
* **Integer**: ``42``, ``-10``
* **Float**: ``3.14``, ``-2.5``
* **Boolean**: ``true``, ``false``

Edge Declarations
-----------------

Syntax
~~~~~~

.. code-block:: ggl

   edge [identifier :] source_node edge_operator target_node [attributes] ;

Edge Operators
~~~~~~~~~~~~~~

* ``->`` : Directed edge
* ``--`` : Undirected edge

Examples
~~~~~~~~

.. code-block:: ggl

   edge friendship: alice -- bob;                    // Named undirected edge
   edge: employee -> manager;                        // Anonymous directed edge
   edge connection: server1 -- server2 [weight=0.8]; // Edge with attributes

Edge IDs
~~~~~~~~

Edge IDs are optional. If omitted, a unique ID will be generated automatically:

.. code-block:: ggl

   edge explicit_id: alice -> bob;    // Explicit ID
   edge: alice -> charlie;            // Auto-generated ID

Generator Statements
--------------------

Syntax
~~~~~~

.. code-block:: ggl

   generate generator_name {
       parameter: value;
       parameter: value;
       ...
   }

Available Generators
~~~~~~~~~~~~~~~~~~~~

Complete Graph
^^^^^^^^^^^^^^

Generates a graph where every node is connected to every other node.

.. code-block:: ggl

   generate complete {
       nodes: 5;                    // Required: number of nodes
       prefix: "vertex";            // Optional: node name prefix (default: "n")
       directed: false;             // Optional: edge direction (default: false)
   }

Path Graph
^^^^^^^^^^

Generates a linear chain of connected nodes.

.. code-block:: ggl

   generate path {
       nodes: 6;                    // Required: number of nodes
       prefix: "step";              // Optional: node name prefix (default: "n")
       directed: false;             // Optional: edge direction (default: false)
   }

Cycle Graph
^^^^^^^^^^^

Generates a circular chain of nodes.

.. code-block:: ggl

   generate cycle {
       nodes: 5;                    // Required: number of nodes
       prefix: "node";              // Optional: node name prefix (default: "n")
   }

Grid Graph
^^^^^^^^^^

Generates a 2D grid of nodes.

.. code-block:: ggl

   generate grid {
       rows: 3;                     // Required: number of rows
       cols: 4;                     // Required: number of columns
       prefix: "cell";              // Optional: node name prefix (default: "n")
       periodic: false;             // Optional: wrap edges (torus) (default: false)
   }

Star Graph
^^^^^^^^^^

Generates a star topology with one central node.

.. code-block:: ggl

   generate star {
       nodes: 6;                    // Required: total number of nodes
       prefix: "node";              // Optional: node name prefix (default: "n")
       directed: false;             // Optional: edge direction (default: false)
   }

Tree Graph
^^^^^^^^^^

Generates a tree with specified branching factor and depth.

.. code-block:: ggl

   generate tree {
       branching: 3;                // Required: children per node
       depth: 3;                    // Required: maximum depth
       prefix: "node";              // Optional: node name prefix (default: "n")
   }

Barabási-Albert Graph
^^^^^^^^^^^^^^^^^^^^^

Generates a scale-free network using preferential attachment.

.. code-block:: ggl

   generate barabasi_albert {
       nodes: 20;                   // Required: total number of nodes
       edges_per_node: 3;           // Required: edges per new node (< nodes)
       prefix: "node";              // Optional: node name prefix (default: "n")
   }

Rule Definitions
----------------

Syntax
~~~~~~

.. code-block:: ggl

   rule rule_name {
       lhs { pattern }
       rhs { pattern }
   }

Patterns
~~~~~~~~

Patterns describe subgraphs to match (LHS) or create (RHS):

.. code-block:: ggl

   rule example_rule {
       lhs {
           node A :employee;
           node B :manager;
           edge: A -> B [type="reports_to"];
       }
       rhs {
           node A :employee [promoted=true];
           node B :manager;
           edge: A -> B [type="reports_to"];
           edge: B -> A [type="mentors"];
       }
   }

Pattern Elements
~~~~~~~~~~~~~~~~

Node Patterns
^^^^^^^^^^^^^

.. code-block:: ggl

   node identifier [: type] [attributes] ;

* **identifier**: Variable name for the node in the pattern
* **type**: Optional type constraint
* **attributes**: Optional attribute constraints

Edge Patterns
^^^^^^^^^^^^^

.. code-block:: ggl

   edge [identifier :] source_node edge_operator target_node [attributes] ;

* **identifier**: Optional variable name for the edge
* **source_node/target_node**: Must reference node identifiers in the pattern
* **edge_operator**: ``->`` or ``--``
* **attributes**: Optional attribute constraints

Rule Semantics
~~~~~~~~~~~~~~

Node Matching
^^^^^^^^^^^^^

Nodes in the LHS pattern match graph nodes if:

1. Types match (if specified in pattern)
2. All specified attributes match exactly
3. The node hasn't been matched by another part of the pattern

Edge Matching
^^^^^^^^^^^^^

Edges in the LHS pattern match graph edges if:

1. Source and target nodes match the pattern
2. Direction matches (directed vs undirected)
3. All specified attributes match exactly
4. The edge hasn't been matched by another part of the pattern

Transformation
^^^^^^^^^^^^^^

When a rule is applied:

1. **Preserved Elements**: Nodes/edges that appear in both LHS and RHS with the same identifier are preserved but may be modified
2. **Deleted Elements**: Nodes/edges that appear in LHS but not RHS are deleted
3. **Created Elements**: Nodes/edges that appear in RHS but not LHS are created
4. **Modified Elements**: Preserved elements may have their attributes updated based on RHS specification

Rule Applications
-----------------

Syntax
~~~~~~

.. code-block:: ggl

   apply rule_name number times ;

Examples
~~~~~~~~

.. code-block:: ggl

   apply promote_employee 5 times;
   apply add_metadata 10 times;
   apply close_triangle 1 times;

Application Semantics
~~~~~~~~~~~~~~~~~~~~~

* Rules are applied iteratively up to the specified number of times
* In each iteration, the rule system finds all possible matches
* For rules that create new elements, only one match is applied per iteration to avoid ID conflicts
* For rules that only modify existing elements, all matches may be applied simultaneously
* Rule application stops early if no matches are found

Identifiers
-----------

Identifiers are used for node names, edge names, rule names, and pattern variables.

Syntax
~~~~~~

.. code-block:: text

   identifier = (letter | "_") (letter | digit | "_")*

Examples
~~~~~~~~

.. code-block:: ggl

   alice
   web_server_1
   _private_node
   DatabaseConnection

Literals
--------

String Literals
~~~~~~~~~~~~~~~

.. code-block:: ggl

   "Hello, World!"
   "Alice Johnson"
   ""

Numeric Literals
~~~~~~~~~~~~~~~~

.. code-block:: ggl

   42          // Integer
   -10         // Negative integer
   3.14        // Float
   -2.5        // Negative float

Boolean Literals
~~~~~~~~~~~~~~~~

.. code-block:: ggl

   true
   false

Grammar Summary
---------------

.. code-block:: pest

   // Program structure
   program = { SOI ~ graph ~ EOI }
   graph = { "graph" ~ ident? ~ "{" ~ statement* ~ "}" }

   // Statements
   statement = { node_decl | edge_decl | generate_stmt | rule_def | apply_rule }

   // Node declarations
   node_decl = { "node" ~ ident ~ node_type? ~ attributes? ~ ";" }
   node_type = { ":" ~ ident }

   // Edge declarations
   edge_decl = { "edge" ~ ident? ~ ":" ~ ident ~ edge_op ~ ident ~ attributes? ~ ";" }
   edge_op = { "->" | "--" }

   // Attributes
   attributes = { "[" ~ attribute_list ~ "]" }
   attribute_list = { (attribute ~ ("," ~ attribute)*)? }
   attribute = { ident ~ "=" ~ value }

   // Values
   value = { string | number | boolean | ident }
   string = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
   number = @{ ("+" | "-")? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT*)? }
   boolean = @{ "true" | "false" }

   // Generators
   generate_stmt = { "generate" ~ ident ~ "{" ~ param_list ~ "}" }
   param_list = { (param ~ ";")* }
   param = { ident ~ ":" ~ value }

   // Rules
   rule_def = { "rule" ~ ident ~ "{" ~ "lhs" ~ pattern ~ "rhs" ~ pattern ~ "}" }
   apply_rule = { "apply" ~ ident ~ number ~ "times" ~ ";" }

   // Patterns
   pattern = { "{" ~ (node_pattern | edge_pattern)* ~ "}" }
   node_pattern = { "node" ~ ident ~ node_type? ~ attributes? ~ ";" }
   edge_pattern = { ("edge" ~ ident? ~ ":")? ~ ident ~ edge_op ~ ident ~ attributes? ~ ";" }

   // Identifiers
   ident = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }

   // Whitespace and comments
   WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
   COMMENT = _{ "//" ~ (!"\n" ~ ANY)* | "/*" ~ (!"*/" ~ ANY)* ~ "*/" }

Error Handling
--------------

Common Syntax Errors
~~~~~~~~~~~~~~~~~~~~

**Missing Semicolons**
   All statements must end with semicolons.

**Unmatched Brackets**
   Ensure all ``{}``, ``[]``, and ``()`` are properly matched.

**Invalid Identifiers**
   Identifiers must start with a letter or underscore.

**Invalid Attribute Values**
   Attribute values must be valid strings, numbers, or booleans.

Common Semantic Errors
~~~~~~~~~~~~~~~~~~~~~~

**Undefined Node References**
   Edge declarations must reference existing nodes.

**Unknown Rule Applications**
   Applied rules must be defined in the same graph.

**Invalid Generator Parameters**
   Generator parameters must match expected types and constraints.

**Pattern Mismatches**
   Rule patterns must be valid and reachable in the graph structure.
