Graph Generators
================

GGL provides several built-in graph generators for creating common graph structures. This document provides detailed information about each generator, including parameters, examples, and use cases.

Overview
--------

Generators are invoked using the ``generate`` statement:

.. code-block:: ggl

   generate generator_name {
       parameter: value;
       parameter: value;
   }

All generators support the ``prefix`` parameter to customize node naming.

Complete Graph Generator
------------------------

Creates a graph where every node is connected to every other node.

Parameters
~~~~~~~~~~

* ``nodes`` (required): Number of nodes to generate
* ``prefix`` (optional): Node name prefix (default: "n")
* ``directed`` (optional): Whether edges should be directed (default: false)

Examples
~~~~~~~~

Basic complete graph:

.. code-block:: ggl

   generate complete {
       nodes: 4;
   }

Creates nodes: n0, n1, n2, n3 with 6 undirected edges.

Custom prefix and directed edges:

.. code-block:: ggl

   generate complete {
       nodes: 3;
       prefix: "vertex";
       directed: true;
   }

Creates nodes: vertex0, vertex1, vertex2 with 6 directed edges.

Properties
~~~~~~~~~~

* **Nodes**: n
* **Edges**: n(n-1)/2 for undirected, n(n-1) for directed
* **Connectivity**: Every node connected to every other node
* **Use cases**: Fully connected networks, cliques, reference topologies

Path Graph Generator
--------------------

Creates a linear chain of connected nodes.

Parameters
~~~~~~~~~~

* ``nodes`` (required): Number of nodes to generate
* ``prefix`` (optional): Node name prefix (default: "n")
* ``directed`` (optional): Whether edges should be directed (default: false)

Examples
~~~~~~~~

Basic path:

.. code-block:: ggl

   generate path {
       nodes: 5;
   }

Creates: n0 -- n1 -- n2 -- n3 -- n4

Directed path with custom prefix:

.. code-block:: ggl

   generate path {
       nodes: 4;
       prefix: "step";
       directed: true;
   }

Creates: step0 -> step1 -> step2 -> step3

Properties
~~~~~~~~~~

* **Nodes**: n
* **Edges**: n-1
* **Connectivity**: Linear chain
* **Use cases**: Sequences, pipelines, linear processes

Cycle Graph Generator
---------------------

Creates a circular chain of nodes where the last node connects back to the first.

Parameters
~~~~~~~~~~

* ``nodes`` (required): Number of nodes to generate
* ``prefix`` (optional): Node name prefix (default: "n")

Examples
~~~~~~~~

Basic cycle:

.. code-block:: ggl

   generate cycle {
       nodes: 5;
   }

Creates a pentagon: n0 -- n1 -- n2 -- n3 -- n4 -- n0

Custom prefix:

.. code-block:: ggl

   generate cycle {
       nodes: 3;
       prefix: "vertex";
   }

Creates a triangle: vertex0 -- vertex1 -- vertex2 -- vertex0

Properties
~~~~~~~~~~

* **Nodes**: n
* **Edges**: n
* **Connectivity**: Circular chain
* **Use cases**: Rings, circular processes, closed loops

Grid Graph Generator
--------------------

Creates a 2D grid of nodes with optional periodic boundary conditions.

Parameters
~~~~~~~~~~

* ``rows`` (required): Number of rows
* ``cols`` (required): Number of columns
* ``prefix`` (optional): Node name prefix (default: "n")
* ``periodic`` (optional): Whether to wrap edges around (torus topology) (default: false)

Examples
~~~~~~~~

Basic 3x3 grid:

.. code-block:: ggl

   generate grid {
       rows: 3;
       cols: 3;
   }

Creates nodes: n0_0, n0_1, n0_2, n1_0, n1_1, n1_2, n2_0, n2_1, n2_2

Periodic grid (torus):

.. code-block:: ggl

   generate grid {
       rows: 4;
       cols: 4;
       prefix: "cell";
       periodic: true;
   }

Creates a 4x4 torus with wraparound edges.

Properties
~~~~~~~~~~

* **Nodes**: rows × cols
* **Edges**:
  * Regular: (rows-1)×cols + rows×(cols-1)
  * Periodic: 2×rows×cols
* **Connectivity**: 2D lattice structure
* **Use cases**: Spatial networks, cellular automata, mesh topologies

Star Graph Generator
--------------------

Creates a star topology with one central node connected to all others.

Parameters
~~~~~~~~~~

* ``nodes`` (required): Total number of nodes (including center)
* ``prefix`` (optional): Node name prefix (default: "n")
* ``directed`` (optional): Whether edges should be directed (default: false)

Examples
~~~~~~~~

Basic star:

.. code-block:: ggl

   generate star {
       nodes: 6;
   }

Creates: n0 (center) connected to n1, n2, n3, n4, n5

Directed star:

.. code-block:: ggl

   generate star {
       nodes: 4;
       prefix: "node";
       directed: true;
   }

Creates: node0 -> node1, node0 -> node2, node0 -> node3

Properties
~~~~~~~~~~

* **Nodes**: n
* **Edges**: n-1
* **Connectivity**: Central hub topology
* **Use cases**: Hub networks, centralized systems, broadcast topologies

Tree Graph Generator
--------------------

Creates a tree with specified branching factor and depth.

Parameters
~~~~~~~~~~

* ``branching`` (required): Number of children per node
* ``depth`` (required): Maximum depth of the tree
* ``prefix`` (optional): Node name prefix (default: "n")

Examples
~~~~~~~~

Binary tree:

.. code-block:: ggl

   generate tree {
       branching: 2;
       depth: 3;
   }

Creates a binary tree with depth 3: 1 + 2 + 4 = 7 nodes

Ternary tree:

.. code-block:: ggl

   generate tree {
       branching: 3;
       depth: 2;
       prefix: "node";
   }

Creates: node0 (root) with 3 children, each having 3 children

Properties
~~~~~~~~~~

* **Nodes**: (b^d - 1) / (b - 1) where b=branching, d=depth
* **Edges**: nodes - 1
* **Connectivity**: Hierarchical tree structure
* **Use cases**: Hierarchies, decision trees, organizational structures

Barabási-Albert Graph Generator
-------------------------------

Creates a scale-free network using preferential attachment.

Parameters
~~~~~~~~~~

* ``nodes`` (required): Total number of nodes
* ``edges_per_node`` (required): Number of edges each new node creates (must be < nodes)
* ``prefix`` (optional): Node name prefix (default: "n")

Examples
~~~~~~~~

Basic scale-free network:

.. code-block:: ggl

   generate barabasi_albert {
       nodes: 20;
       edges_per_node: 3;
   }

Creates a 20-node scale-free network where each new node connects to 3 existing nodes.

Small scale-free network:

.. code-block:: ggl

   generate barabasi_albert {
       nodes: 10;
       edges_per_node: 2;
       prefix: "vertex";
   }

Properties
~~~~~~~~~~

* **Nodes**: n
* **Edges**: Approximately n × edges_per_node
* **Connectivity**: Scale-free degree distribution
* **Use cases**: Social networks, web graphs, biological networks

Algorithm Details
~~~~~~~~~~~~~~~~~

1. Start with a complete graph of ``edges_per_node + 1`` nodes
2. For each new node:
   - Calculate degree-based probabilities for existing nodes
   - Select ``edges_per_node`` distinct nodes using preferential attachment
   - Connect the new node to selected nodes

Combining Generators
--------------------

You can use multiple generators in the same graph:

.. code-block:: ggl

   graph hybrid_network {
       // Core backbone
       generate complete {
           nodes: 5;
           prefix: "core";
       }

       // Regional clusters
       generate star {
           nodes: 8;
           prefix: "region1";
       }

       generate star {
           nodes: 6;
           prefix: "region2";
       }
   }

Generator Best Practices
------------------------

Naming Conventions
~~~~~~~~~~~~~~~~~~

Use descriptive prefixes to distinguish different parts of your graph:

.. code-block:: ggl

   generate complete {
       nodes: 5;
       prefix: "backbone";
   }

   generate star {
       nodes: 10;
       prefix: "cluster";
   }

Parameter Validation
~~~~~~~~~~~~~~~~~~~~

Generators validate their parameters:

* ``nodes`` must be positive
* ``edges_per_node`` must be less than ``nodes`` for Barabási-Albert
* ``rows`` and ``cols`` must be positive for grid
* ``branching`` and ``depth`` must be positive for tree

Performance Considerations
~~~~~~~~~~~~~~~~~~~~~~~~~~

* Large complete graphs (nodes > 1000) generate many edges
* Grid graphs with periodic boundaries double the edge count
* Barabási-Albert generation time increases with network size
* Tree depth grows exponentially with branching factor

Common Use Cases
----------------

Network Modeling
~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph internet_topology {
       // Core routers (complete graph)
       generate complete {
           nodes: 10;
           prefix: "core_router";
       }

       // Regional networks (stars)
       generate star {
           nodes: 20;
           prefix: "region_a";
       }
   }

Social Networks
~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph social_platform {
       // Influencers (highly connected)
       generate barabasi_albert {
           nodes: 100;
           edges_per_node: 5;
           prefix: "user";
       }
   }

Infrastructure
~~~~~~~~~~~~~~

.. code-block:: ggl

   graph data_center {
       // Server rack (grid)
       generate grid {
           rows: 8;
           cols: 10;
           prefix: "server";
       }
   }

Organizational Structure
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph company {
       // Management hierarchy
       generate tree {
           branching: 4;
           depth: 4;
           prefix: "employee";
       }
   }
