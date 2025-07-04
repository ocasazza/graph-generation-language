Graph Generation Language Documentation
=======================================

Welcome to the Graph Generation Language (GGL) documentation. GGL is a domain-specific language for creating and manipulating graphs through declarative syntax.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   getting-started
   language-reference
   generators
   transformation-rules
   examples
   api-reference
   contributing

Overview
--------

GGL allows you to:

* Define graph structures using intuitive node and edge declarations
* Generate common graph topologies with built-in generators
* Apply transformation rules to modify graph structure
* Export graphs in standard JSON format

Quick Example
-------------

.. code-block:: ggl

   graph social_network {
       // Define nodes with types and attributes
       node alice :person [name="Alice", age=30];
       node bob :person [name="Bob", age=25];

       // Create relationships
       edge friendship: alice -- bob [strength=0.8];

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

Features
--------

* **Declarative Syntax**: Define graphs using intuitive node and edge declarations
* **Built-in Generators**: Create common graph structures (complete, path, cycle, grid, star, tree, scale-free)
* **Transformation Rules**: Apply pattern-based rules to modify graph structure
* **Rich Attributes**: Support for typed nodes and edges with metadata
* **JSON Output**: Export graphs in standard JSON format

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
