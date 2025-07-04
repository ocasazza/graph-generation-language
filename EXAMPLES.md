# Graph Generation Language (GGL) - Examples & Usage Guide

This document provides examples and usage patterns for the Graph Generation Language (GGL), a domain-specific language for creating and manipulating graphs through declarative syntax.

## Table of Contents

1. [Basic Syntax](#basic-syntax)
2. [Node Declarations](#node-declarations)
3. [Edge Declarations](#edge-declarations)
4. [Graph Generators](#graph-generators)
5. [Graph Transformation Rules](#graph-transformation-rules)
6. [Real-World Examples](#real-world-examples)
7. [Advanced Patterns](#advanced-patterns)
8. [Performance Considerations](#performance-considerations)

## Basic Syntax

Every GGL program defines a graph with optional name:

```ggl
graph my_graph {
    // Graph content goes here
}
```

### Comments

GGL supports both single-line and multi-line comments:

```ggl
graph example {
    // Single-line comment
    node alice;

    /*
     * Multi-line comment
     * for detailed explanations
     */
    node bob;
}
```

## Node Declarations

### Simple Nodes

```ggl
graph simple {
    node alice;
    node bob;
    node charlie;
}
```

### Typed Nodes

Nodes can have types using the `:type` syntax:

```ggl
graph social_network {
    node alice :person;
    node bob :person;
    node company_x :organization;
}
```

### Nodes with Attributes

Attributes are specified in square brackets with key-value pairs:

```ggl
graph detailed {
    node alice :person [
        name="Alice Johnson",
        age=30,
        active=true,
        score=98.5,
        location="New York"
    ];

    node company :organization [
        name="Tech Corp",
        employees=1000,
        public=false,
        revenue=50000000.0
    ];
}
```

### Supported Attribute Types

- **Strings**: `"text value"`
- **Integers**: `42`, `-10`
- **Floats**: `3.14`, `-2.5`
- **Booleans**: `true`, `false`

## Edge Declarations

### Directed Edges

Use `->` for directed edges:

```ggl
graph directed {
    node manager;
    node employee;

    edge reports_to: employee -> manager;
}
```

### Undirected Edges

Use `--` for undirected edges:

```ggl
graph friendship {
    node alice;
    node bob;

    edge friendship: alice -- bob;
}
```

### Edges with Attributes

```ggl
graph weighted {
    node city_a;
    node city_b;

    edge highway: city_a -- city_b [
        distance=150.5,
        toll=true,
        speed_limit=70
    ];
}
```

### Edges without Explicit IDs

The edge ID can be omitted, and one will be generated automatically:

```ggl
graph auto_edges {
    node a;
    node b;
    node c;

    edge: a -> b;
    edge: b -> c;
}
```

## Graph Generators

GGL provides several built-in graph generators for common graph structures.

### Complete Graph

Generates a graph where every node is connected to every other node:

```ggl
graph complete_example {
    generate complete {
        nodes: 5;
        prefix: "vertex";
        directed: false;
    }
}
```

**Parameters:**
- `nodes`: Number of nodes to generate
- `prefix`: Prefix for node names (default: "n")
- `directed`: Whether edges should be directed (default: false)

### Path Graph

Generates a linear chain of connected nodes:

```ggl
graph path_example {
    generate path {
        nodes: 6;
        prefix: "step";
        directed: false;
    }
}
```

**Result:** step0 -- step1 -- step2 -- step3 -- step4 -- step5

### Cycle Graph

Generates a circular chain of nodes:

```ggl
graph cycle_example {
    generate cycle {
        nodes: 5;
        prefix: "node";
    }
}
```

**Result:** A pentagon where each node connects to its neighbors and the last connects back to the first.

### Grid Graph

Generates a 2D grid of nodes:

```ggl
graph grid_example {
    generate grid {
        rows: 3;
        cols: 4;
        prefix: "cell";
        periodic: false;
    }
}
```

**Parameters:**
- `rows`: Number of rows
- `cols`: Number of columns
- `prefix`: Prefix for node names (default: "n")
- `periodic`: Whether to wrap edges around (torus topology)

**Result:** Nodes named like `cell0_0`, `cell0_1`, etc., connected in a grid pattern.

### Star Graph

Generates a star topology with one central node connected to all others:

```ggl
graph star_example {
    generate star {
        nodes: 6;
        prefix: "node";
        directed: false;
    }
}
```

**Result:** `node0` (center) connected to `node1`, `node2`, ..., `node5`.

### Tree Graph

Generates a tree with specified branching factor and depth:

```ggl
graph tree_example {
    generate tree {
        branching: 3;
        depth: 3;
        prefix: "node";
    }
}
```

**Parameters:**
- `branching`: Number of children per node
- `depth`: Maximum depth of the tree
- `prefix`: Prefix for node names

### Barabási-Albert Graph

Generates a scale-free network using preferential attachment:

```ggl
graph scale_free {
    generate barabasi_albert {
        nodes: 20;
        edges_per_node: 3;
        prefix: "node";
    }
}
```

**Parameters:**
- `nodes`: Total number of nodes
- `edges_per_node`: Number of edges each new node creates (must be < nodes)
- `prefix`: Prefix for node names

## Graph Transformation Rules

Rules allow you to transform graphs by matching patterns and replacing them.

### Basic Rule Structure

```ggl
rule rule_name {
    lhs { /* pattern to match */ }
    rhs { /* replacement pattern */ }
}
```

### Node Transformation Rules

#### Simple Node Type Change

```ggl
graph promotion {
    node alice :employee;
    node bob :employee;
    node charlie :manager;

    rule promote_employee {
        lhs { node N :employee; }
        rhs { node N :manager; }
    }

    apply promote_employee 5 times;
}
```

#### Adding Attributes

```ggl
graph metadata_addition {
    node user1;
    node user2;

    rule add_metadata {
        lhs { node U; }
        rhs { node U [active=true, created="2024"]; }
    }

    apply add_metadata 10 times;
}
```

### Node Creation Rules

```ggl
graph expansion {
    node root;

    rule add_child {
        lhs { node P; }
        rhs {
            node P;
            node C :child;
            edge: P -> C [type="parent_child"];
        }
    }

    apply add_child 3 times;
}
```

### Node Deletion Rules

```ggl
graph cleanup {
    node isolated1;
    node isolated2;
    node connected1;
    node connected2;
    edge: connected1 -- connected2;

    rule delete_isolated {
        lhs { node N; }  // Matches isolated nodes only
        rhs { }          // Empty RHS = deletion
    }

    apply delete_isolated 5 times;
}
```

### Edge Pattern Rules

#### Triangle Closure

```ggl
graph triangle_closure {
    node a;
    node b;
    node c;
    edge: a -- b;
    edge: b -- c;

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
            edge: A -- C;  // New edge to close triangle
        }
    }

    apply close_triangle 1 times;
}
```

#### Edge Transformation

```ggl
graph edge_transformation {
    node server1;
    node server2;
    edge temp_connection: server1 -> server2;

    rule make_permanent {
        lhs {
            node A;
            node B;
            edge E: A -> B;
        }
        rhs {
            node A;
            node B;
            edge permanent: A -- B [status="permanent"];
        }
    }

    apply make_permanent 1 times;
}
```

## Real-World Examples

### Social Network

```ggl
graph social_network {
    // Core users
    node alice :person [
        name="Alice Johnson",
        age=30,
        location="NYC",
        interests="technology,music"
    ];

    node bob :person [
        name="Bob Smith",
        age=25,
        location="SF",
        interests="sports,travel"
    ];

    node charlie :person [
        name="Charlie Brown",
        age=35,
        location="LA",
        interests="art,food"
    ];

    // Relationships
    edge friendship1: alice -- bob [
        type="friendship",
        strength=0.8,
        since="2020-01-15"
    ];

    edge friendship2: bob -- charlie [
        type="friendship",
        strength=0.6,
        since="2021-03-22"
    ];

    // Generate additional users
    generate complete {
        nodes: 10;
        prefix: "user";
    }

    // Add metadata to all users
    rule add_user_metadata {
        lhs { node U; }
        rhs { node U [active=true, joined="2024", verified=false]; }
    }

    apply add_user_metadata 20 times;

    // Create friend recommendations (mutual friends)
    rule recommend_friends {
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
            edge recommendation: A -- C [type="recommendation", confidence=0.7];
        }
    }

    apply recommend_friends 5 times;
}
```

### Corporate Hierarchy

```ggl
graph organization {
    // Executive level
    node ceo :executive [
        title="Chief Executive Officer",
        level=1,
        department="executive"
    ];

    node cto :executive [
        title="Chief Technology Officer",
        level=2,
        department="technology"
    ];

    node cfo :executive [
        title="Chief Financial Officer",
        level=2,
        department="finance"
    ];

    // Reporting structure
    edge: cto -> ceo [type="reports_to"];
    edge: cfo -> ceo [type="reports_to"];

    // Generate team structure
    generate tree {
        branching: 4;
        depth: 4;
        prefix: "emp";
    }

    // Assign roles based on hierarchy
    rule assign_manager_role {
        lhs {
            node M;
            node S;
            edge: M -> S;
        }
        rhs {
            node M :manager [has_reports=true];
            node S :employee [manager_assigned=true];
            edge: M -> S [type="manages"];
        }
    }

    apply assign_manager_role 20 times;

    // Add department assignments
    rule assign_departments {
        lhs { node E :employee; }
        rhs { node E :employee [department="engineering", team="backend"]; }
    }

    apply assign_departments 15 times;
}
```

### Infrastructure Network

```ggl
graph infrastructure {
    // Data centers
    node primary_dc :datacenter [
        location="us-east-1",
        capacity=1000,
        status="active"
    ];

    node backup_dc :datacenter [
        location="us-west-2",
        capacity=500,
        status="standby"
    ];

    node edge_dc :datacenter [
        location="eu-west-1",
        capacity=200,
        status="active"
    ];

    // Connectivity between data centers
    edge: primary_dc -- backup_dc [
        type="backbone",
        bandwidth="10Gbps",
        latency=50
    ];

    edge: primary_dc -- edge_dc [
        type="international",
        bandwidth="1Gbps",
        latency=120
    ];

    // Generate server infrastructure
    generate grid {
        rows: 5;
        cols: 8;
        prefix: "server";
    }

    // Assign servers to data centers
    rule assign_to_datacenter {
        lhs { node S; }
        rhs {
            node S :server [
                status="active",
                cpu_cores=16,
                memory="64GB",
                storage="1TB"
            ];
            node primary_dc :datacenter;
            edge: S -> primary_dc [type="hosted_in"];
        }
    }

    apply assign_to_datacenter 40 times;

    // Create load balancer connections
    rule add_load_balancing {
        lhs {
            node S1 :server;
            node S2 :server;
        }
        rhs {
            node S1 :server;
            node S2 :server;
            edge: S1 -- S2 [type="load_balanced", weight=0.5];
        }
    }

    apply add_load_balancing 10 times;
}
```

### Transportation Network

```ggl
graph transportation {
    // Major cities
    node nyc :city [name="New York", population=8000000];
    node la :city [name="Los Angeles", population=4000000];
    node chicago :city [name="Chicago", population=2700000];
    node houston :city [name="Houston", population=2300000];

    // Highways
    edge i80: nyc -- chicago [
        type="interstate",
        distance=790,
        speed_limit=70
    ];

    edge i10: la -- houston [
        type="interstate",
        distance=1540,
        speed_limit=75
    ];

    // Generate regional network
    generate complete {
        nodes: 15;
        prefix: "town";
    }

    // Connect towns to cities
    rule connect_to_city {
        lhs { node T; }
        rhs {
            node T :town [population=50000];
            node nyc :city;
            edge: T -- nyc [type="highway", distance=100];
        }
    }

    apply connect_to_city 15 times;

    // Add traffic flow
    rule add_traffic_data {
        lhs {
            node A;
            node B;
            edge R: A -- B;
        }
        rhs {
            node A;
            node B;
            edge R: A -- B [
                traffic_volume=5000,
                peak_hours="7-9,17-19",
                toll_cost=5.50
            ];
        }
    }

    apply add_traffic_data 20 times;
}
```

## Advanced Patterns

### Combining Multiple Generators

```ggl
graph hybrid_network {
    // Core backbone (complete graph)
    generate complete {
        nodes: 5;
        prefix: "core";
    }

    // Regional clusters (star networks)
    generate star {
        nodes: 8;
        prefix: "region1";
    }

    generate star {
        nodes: 6;
        prefix: "region2";
    }

    // Connect regions to core
    rule connect_regions {
        lhs {
            node R;
            node C;
        }
        rhs {
            node R :regional;
            node C :core;
            edge: R -- C [type="uplink", capacity="1Gbps"];
        }
    }

    apply connect_regions 3 times;
}
```

### Conditional Transformations

```ggl
graph conditional_growth {
    node seed :active;

    // Only active nodes can spawn children
    rule grow_active {
        lhs { node P :active; }
        rhs {
            node P :active [children=1];
            node C :inactive;
            edge: P -> C [type="spawned"];
        }
    }

    apply grow_active 5 times;

    // Activate some inactive nodes
    rule activate_nodes {
        lhs { node N :inactive; }
        rhs { node N :active [activated=true]; }
    }

    apply activate_nodes 3 times;
}
```

### Multi-Stage Transformations

```ggl
graph evolution {
    generate path {
        nodes: 10;
        prefix: "node";
    }

    // Stage 1: Add initial metadata
    rule stage1_init {
        lhs { node N; }
        rhs { node N [stage=1, energy=100]; }
    }

    apply stage1_init 10 times;

    // Stage 2: Create connections
    rule stage2_connect {
        lhs {
            node A [stage=1];
            node B [stage=1];
        }
        rhs {
            node A [stage=2];
            node B [stage=2];
            edge: A -- B [type="evolved", strength=0.8];
        }
    }

    apply stage2_connect 5 times;

    // Stage 3: Optimize structure
    rule stage3_optimize {
        lhs {
            node N [stage=2];
            edge E: N -- ;
        }
        rhs {
            node N [stage=3, optimized=true];
            edge E: N -- [weight=1.0];
        }
    }

    apply stage3_optimize 10 times;
}
```

## Performance Considerations

### Efficient Rule Application

1. **Limit iterations**: Always specify reasonable iteration limits
2. **Order rules carefully**: Apply more specific rules before general ones
3. **Use type constraints**: Typed nodes match faster than untyped ones

```ggl
graph efficient {
    generate complete {
        nodes: 100;
        prefix: "node";
    }

    // Good: Specific type constraint
    rule process_servers {
        lhs { node N :server; }
        rhs { node N :server [processed=true]; }
    }

    apply process_servers 100 times;  // Reasonable limit
}
```

### Memory Optimization

For large graphs, consider:

1. **Batch processing**: Apply rules in smaller batches
2. **Selective generation**: Generate only what you need
3. **Attribute minimization**: Use only necessary attributes

```ggl
graph optimized {
    // Generate in reasonable chunks
    generate grid {
        rows: 20;
        cols: 20;  // 400 nodes - manageable
        prefix: "cell";
    }

    // Minimal attributes
    rule add_essential_data {
        lhs { node N; }
        rhs { node N [id=1]; }  // Only essential data
    }

    apply add_essential_data 400 times;
}
```

### Best Practices

1. **Start small**: Test with small graphs first
2. **Use meaningful names**: Clear node and edge IDs improve debugging
3. **Comment complex rules**: Explain the purpose of transformation rules
4. **Validate results**: Check that rules produce expected outcomes

```ggl
graph best_practices {
    // Clear, meaningful names
    node web_server :server [role="frontend"];
    node database :server [role="backend"];

    // Well-documented rule
    rule establish_connection {
        /*
         * Creates a connection between web servers and databases
         * for load balancing and redundancy
         */
        lhs {
            node WS :server [role="frontend"];
            node DB :server [role="backend"];
        }
        rhs {
            node WS :server [role="frontend"];
            node DB :server [role="backend"];
            edge: WS -> DB [
                type="database_connection",
                pool_size=10,
                timeout=30
            ];
        }
    }

    apply establish_connection 1 times;
}
```
