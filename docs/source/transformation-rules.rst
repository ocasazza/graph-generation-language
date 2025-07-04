Transformation Rules
====================

Transformation rules are the heart of GGL's graph manipulation capabilities. They allow you to define patterns to match in your graph and specify how to transform those patterns.

Overview
--------

A rule consists of two parts:

* **LHS (Left-Hand Side)**: The pattern to match in the graph
* **RHS (Right-Hand Side)**: The replacement pattern

.. code-block:: ggl

   rule rule_name {
       lhs { pattern_to_match }
       rhs { replacement_pattern }
   }

Rule Application
----------------

Rules are applied using the ``apply`` statement:

.. code-block:: ggl

   apply rule_name number times;

Basic Rule Types
----------------

Node Transformation Rules
~~~~~~~~~~~~~~~~~~~~~~~~~~

These rules modify node properties without changing graph structure.

Type Changes
^^^^^^^^^^^^

.. code-block:: ggl

   rule promote_employee {
       lhs { node N :employee; }
       rhs { node N :manager; }
   }

This rule changes all nodes of type "employee" to type "manager".

Attribute Addition
^^^^^^^^^^^^^^^^^^

.. code-block:: ggl

   rule add_timestamp {
       lhs { node N; }
       rhs { node N [created="2024", active=true]; }
   }

This rule adds attributes to all nodes.

Conditional Transformation
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: ggl

   rule activate_users {
       lhs { node U :user [status="pending"]; }
       rhs { node U :user [status="active", activated="2024"]; }
   }

This rule only affects users with status "pending".

Node Creation Rules
~~~~~~~~~~~~~~~~~~~

These rules create new nodes and edges.

Simple Node Creation
^^^^^^^^^^^^^^^^^^^^

.. code-block:: ggl

   rule add_child {
       lhs { node P :parent; }
       rhs {
           node P :parent;
           node C :child;
           edge: P -> C [type="parent_child"];
       }
   }

This rule adds a child node to every parent node.

Conditional Creation
^^^^^^^^^^^^^^^^^^^^

.. code-block:: ggl

   rule expand_servers {
       lhs { node S :server [load="high"]; }
       rhs {
           node S :server [load="high"];
           node B :server [load="low", type="backup"];
           edge: S -- B [type="backup_link"];
       }
   }

This rule creates backup servers for high-load servers.

Node Deletion Rules
~~~~~~~~~~~~~~~~~~~

These rules remove nodes from the graph.

Simple Deletion
^^^^^^^^^^^^^^^

.. code-block:: ggl

   rule remove_inactive {
       lhs { node N [active=false]; }
       rhs { }
   }

This rule deletes all inactive nodes. The empty RHS means deletion.

Conditional Deletion
^^^^^^^^^^^^^^^^^^^^

.. code-block:: ggl

   rule cleanup_old_sessions {
       lhs { node S :session [age=30]; }
       rhs { }
   }

This rule removes old session nodes.

Edge Pattern Rules
~~~~~~~~~~~~~~~~~~

These rules work with edge patterns and connectivity.

Triangle Closure
^^^^^^^^^^^^^^^^

.. code-block:: ggl

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
           edge: A -- C [type="inferred"];
       }
   }

This rule adds edges to close triangles in the graph.

Edge Transformation
^^^^^^^^^^^^^^^^^^^

.. code-block:: ggl

   rule strengthen_connections {
       lhs {
           node A;
           node B;
           edge E: A -- B [weight=0.5];
       }
       rhs {
           node A;
           node B;
           edge E: A -- B [weight=1.0, strengthened=true];
       }
   }

This rule modifies edge attributes.

Path Extension
^^^^^^^^^^^^^^

.. code-block:: ggl

   rule extend_path {
       lhs {
           node A;
           node B;
           edge: A -> B [type="path"];
       }
       rhs {
           node A;
           node B;
           node C :endpoint;
           edge: A -> B [type="path"];
           edge: B -> C [type="path"];
       }
   }

This rule extends paths by adding new endpoints.

Advanced Rule Patterns
----------------------

Multi-Node Patterns
~~~~~~~~~~~~~~~~~~~~

Rules can match complex subgraph patterns:

.. code-block:: ggl

   rule create_management_layer {
       lhs {
           node E1 :employee;
           node E2 :employee;
           node E3 :employee;
       }
       rhs {
           node E1 :employee;
           node E2 :employee;
           node E3 :employee;
           node M :manager;
           edge: E1 -> M [type="reports_to"];
           edge: E2 -> M [type="reports_to"];
           edge: E3 -> M [type="reports_to"];
       }
   }

Attribute-Based Matching
~~~~~~~~~~~~~~~~~~~~~~~~

Rules can match based on specific attribute values:

.. code-block:: ggl

   rule upgrade_servers {
       lhs {
           node S :server [cpu=4, memory=8];
       }
       rhs {
           node S :server [cpu=8, memory=16, upgraded=true];
       }
   }

Type-Specific Rules
~~~~~~~~~~~~~~~~~~~

Rules can be restricted to specific node types:

.. code-block:: ggl

   rule process_orders {
       lhs {
           node O :order [status="pending"];
           node C :customer;
           edge: O -> C [type="belongs_to"];
       }
       rhs {
           node O :order [status="processing", started="2024"];
           node C :customer;
           edge: O -> C [type="belongs_to"];
       }
   }

Rule Application Semantics
---------------------------

Iteration Behavior
~~~~~~~~~~~~~~~~~~

Rules are applied iteratively:

1. Find all matches for the LHS pattern
2. Apply transformations (one match per iteration for creation rules)
3. Repeat until no more matches or iteration limit reached

.. code-block:: ggl

   apply expand_network 5 times;

This applies the rule up to 5 times, stopping early if no matches are found.

Match Selection
~~~~~~~~~~~~~~~

For rules that create new elements:
- Only one match is applied per iteration to avoid ID conflicts
- The first match found is typically selected

For rules that only modify existing elements:
- All matches may be applied simultaneously

Conflict Resolution
~~~~~~~~~~~~~~~~~~~

When multiple rules could apply to the same elements:
- Rules are applied in the order they appear in the graph
- Each rule application is atomic
- No partial applications occur

Pattern Matching Details
------------------------

Node Matching
~~~~~~~~~~~~~~

Nodes match if:

1. **Type constraint**: If specified in pattern, node type must match exactly
2. **Attribute constraints**: All specified attributes must match exactly
3. **Uniqueness**: Each graph node can only match one pattern node per rule application

Edge Matching
~~~~~~~~~~~~~

Edges match if:

1. **Endpoint matching**: Source and target nodes match the pattern
2. **Direction**: Directed (``->``) vs undirected (``--``) must match
3. **Attribute constraints**: All specified attributes must match exactly

Isolation Requirements
~~~~~~~~~~~~~~~~~~~~~~

For deletion rules (empty RHS), nodes must be isolated (no edges) to match single-node patterns.

Common Rule Patterns
--------------------

Network Growth
~~~~~~~~~~~~~~

.. code-block:: ggl

   rule grow_network {
       lhs { node N :active; }
       rhs {
           node N :active [connections=1];
           node M :new;
           edge: N -- M [type="growth"];
       }
   }

Hierarchy Creation
~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   rule create_hierarchy {
       lhs {
           node W1 :worker;
           node W2 :worker;
       }
       rhs {
           node W1 :worker;
           node W2 :worker;
           node S :supervisor;
           edge: W1 -> S [type="reports_to"];
           edge: W2 -> S [type="reports_to"];
       }
   }

Load Balancing
~~~~~~~~~~~~~~

.. code-block:: ggl

   rule balance_load {
       lhs {
           node S :server [load="high"];
           node T :server [load="low"];
       }
       rhs {
           node S :server [load="medium"];
           node T :server [load="medium"];
           edge: S -- T [type="load_share"];
       }
   }

Cleanup Operations
~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   rule remove_duplicates {
       lhs {
           node A;
           node B;
           edge E1: A -- B;
           edge E2: A -- B;
       }
       rhs {
           node A;
           node B;
           edge E1: A -- B;
       }
   }

Best Practices
--------------

Rule Design
~~~~~~~~~~~

1. **Start simple**: Begin with basic transformations before complex patterns
2. **Use types**: Type constraints make rules more specific and efficient
3. **Limit iterations**: Always specify reasonable iteration limits
4. **Test incrementally**: Apply rules step by step to verify behavior

Pattern Specificity
~~~~~~~~~~~~~~~~~~~~

1. **Be specific**: More specific patterns match fewer cases but are more predictable
2. **Use attributes**: Attribute constraints help target specific nodes/edges
3. **Consider order**: Rule application order can affect results

Performance
~~~~~~~~~~~

1. **Minimize pattern size**: Smaller patterns match faster
2. **Use type constraints**: Typed matching is more efficient
3. **Limit creation rules**: Rules that create elements should have reasonable iteration limits

Debugging Rules
~~~~~~~~~~~~~~~

1. **Start with small graphs**: Test rules on simple examples first
2. **Use meaningful names**: Clear rule and variable names aid debugging
3. **Apply incrementally**: Use small iteration counts during development
4. **Check intermediate results**: Verify graph state between rule applications

Error Handling
--------------

Common Issues
~~~~~~~~~~~~~

**Rule Not Applying**
   - Check that LHS pattern actually exists in your graph
   - Verify type and attribute constraints are correct
   - Ensure nodes referenced in edges exist in the pattern

**Infinite Loops**
   - Rules that create patterns they can match again
   - Solution: Add attributes to prevent re-matching

**ID Conflicts**
   - Multiple matches creating nodes/edges with same IDs
   - Solution: Use unique ID generation or limit iterations

**Type Mismatches**
   - Pattern types don't match graph node types
   - Solution: Verify type names are exactly correct

Example: Complete Workflow
--------------------------

Here's a complete example showing rule-based graph evolution:

.. code-block:: ggl

   graph evolving_network {
       // Start with basic nodes
       node alice :person;
       node bob :person;
       node charlie :person;

       // Initial connections
       edge: alice -- bob;
       edge: bob -- charlie;

       // Rule 1: Add metadata to people
       rule initialize_people {
           lhs { node P :person; }
           rhs { node P :person [active=true, connections=0]; }
       }

       apply initialize_people 5 times;

       // Rule 2: Close triangles
       rule close_triangles {
           lhs {
               node A :person;
               node B :person;
               node C :person;
               edge: A -- B;
               edge: B -- C;
           }
           rhs {
               node A :person;
               node B :person;
               node C :person;
               edge: A -- B;
               edge: B -- C;
               edge: A -- C [type="friend_of_friend"];
           }
       }

       apply close_triangles 3 times;

       // Rule 3: Add influencers
       rule add_influencer {
           lhs {
               node P1 :person;
               node P2 :person;
               edge: P1 -- P2;
           }
           rhs {
               node P1 :person;
               node P2 :person;
               node I :influencer;
               edge: P1 -- P2;
               edge: P1 -- I [type="follows"];
               edge: P2 -- I [type="follows"];
           }
       }

       apply add_influencer 2 times;
   }

This example shows how rules can be chained to create complex graph transformations through simple, incremental steps.
