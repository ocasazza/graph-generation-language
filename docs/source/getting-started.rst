Getting Started
===============

This guide will help you get up and running with the Graph Generation Language (GGL).

Installation
------------

Prerequisites
~~~~~~~~~~~~~

* Rust 1.70 or later
* Cargo (comes with Rust)

Building from Source
~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   git clone https://github.com/ocasazza/graph-generation-language.git
   cd graph-generation-language
   cargo build --release

Running Tests
~~~~~~~~~~~~~

.. code-block:: bash

   cargo test

Your First Graph
----------------

Let's create a simple graph to get familiar with GGL syntax.

Basic Graph
~~~~~~~~~~~

Create a file called ``hello.ggl``:

.. code-block:: ggl

   graph hello_world {
       node alice;
       node bob;
       edge friendship: alice -- bob;
   }

This creates a simple graph with two nodes connected by an undirected edge.

Adding Attributes
~~~~~~~~~~~~~~~~~

Let's make it more interesting by adding types and attributes:

.. code-block:: ggl

   graph social_network {
       node alice :person [
           name="Alice Johnson",
           age=30,
           city="New York"
       ];

       node bob :person [
           name="Bob Smith",
           age=25,
           city="San Francisco"
       ];

       edge friendship: alice -- bob [
           strength=0.8,
           since="2020-01-15"
       ];
   }

Using Generators
~~~~~~~~~~~~~~~~

Instead of manually creating nodes, you can use built-in generators:

.. code-block:: ggl

   graph generated_network {
       // Generate a complete graph with 5 nodes
       generate complete {
           nodes: 5;
           prefix: "user";
       }

       // Add a central hub
       node hub :server;

       // Connect hub to all users (this would be done with rules)
   }

Applying Rules
~~~~~~~~~~~~~~

Rules allow you to transform your graph:

.. code-block:: ggl

   graph evolving_network {
       // Start with some nodes
       node alice :person;
       node bob :person;
       node charlie :person;

       // Add connections
       edge: alice -- bob;
       edge: bob -- charlie;

       // Rule to add metadata to all people
       rule add_status {
           lhs { node N :person; }
           rhs { node N :person [active=true, joined="2024"]; }
       }

       // Apply the rule
       apply add_status 5 times;

       // Rule to close triangles
       rule close_triangle {
           lhs {
               node A;
               node B;
               node C;
               edge: A -- B;
               edge: B -- C;
           }
           rhs {
               node A;
               node B;
               node C;
               edge: A -- B;
               edge: B -- C;
               edge: A -- C;
           }
       }

       apply close_triangle 1 times;
   }

Running Your Program
--------------------

Once you have a GGL file, you can process it:

.. code-block:: bash

   cargo run -- your_file.ggl

This will parse your GGL program and output the resulting graph in JSON format.

Understanding the Output
------------------------

GGL outputs graphs in JSON format with the following structure:

.. code-block:: json

   {
       "nodes": {
           "alice": {
               "id": "alice",
               "type": "person",
               "metadata": {
                   "name": "Alice Johnson",
                   "age": 30,
                   "city": "New York"
               },
               "x": 0.0,
               "y": 0.0
           }
       },
       "edges": {
           "friendship": {
               "id": "friendship",
               "source": "alice",
               "target": "bob",
               "type": "",
               "metadata": {
                   "strength": 0.8,
                   "since": "2020-01-15"
               }
           }
       }
   }

Next Steps
----------

Now that you understand the basics, you can:

* Explore the :doc:`language-reference` for complete syntax details
* Learn about :doc:`generators` for creating common graph structures
* Study :doc:`transformation-rules` for advanced graph manipulation
* Check out :doc:`examples` for real-world use cases

Common Patterns
---------------

Here are some common patterns you'll use frequently:

Creating Typed Hierarchies
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph hierarchy {
       node root :manager;

       generate tree {
           branching: 3;
           depth: 3;
           prefix: "emp";
       }

       rule assign_roles {
           lhs {
               node M;
               node S;
               edge: M -> S;
           }
           rhs {
               node M :manager;
               node S :employee;
               edge: M -> S [type="reports_to"];
           }
       }

       apply assign_roles 10 times;
   }

Building Networks with Hubs
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph hub_network {
       node central_hub :server;

       generate star {
           nodes: 10;
           prefix: "client";
       }

       rule connect_to_hub {
           lhs { node C; }
           rhs {
               node C :client;
               node central_hub :server;
               edge: C -> central_hub [type="connection"];
           }
       }

       apply connect_to_hub 10 times;
   }

Troubleshooting
---------------

Common Issues
~~~~~~~~~~~~~

**Syntax Errors**
   Make sure all statements end with semicolons and brackets are properly matched.

**Rule Not Applying**
   Check that your pattern in the ``lhs`` section actually matches nodes/edges in your graph.

**Missing Nodes in Rules**
   When referencing nodes in rules, make sure they exist in your graph or are created by generators.

**Type Mismatches**
   Node types in patterns must exactly match the types in your graph.

Getting Help
~~~~~~~~~~~~

* Check the :doc:`language-reference` for syntax details
* Look at :doc:`examples` for working code samples
* Review the test files in the ``tests/`` directory for more examples
