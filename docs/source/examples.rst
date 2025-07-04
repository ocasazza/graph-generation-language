Examples
========

This section provides practical examples of using GGL for various graph modeling scenarios.

Social Network Examples
-----------------------

Basic Social Network
~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph social_network {
       // Core users
       node alice :person [name="Alice", age=30, location="NYC"];
       node bob :person [name="Bob", age=25, location="SF"];
       node charlie :person [name="Charlie", age=35, location="LA"];

       // Friendships
       edge: alice -- bob [type="friendship", strength=0.8];
       edge: bob -- charlie [type="friendship", strength=0.6];

       // Generate additional users
       generate complete {
           nodes: 5;
           prefix: "user";
       }

       // Add metadata to all users
       rule add_user_metadata {
           lhs { node U; }
           rhs { node U [active=true, joined="2024"]; }
       }

       apply add_user_metadata 10 times;
   }

Friend Recommendation System
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph recommendation_system {
       // Initial users and connections
       node alice :user;
       node bob :user;
       node charlie :user;
       node diana :user;

       edge: alice -- bob [type="friend"];
       edge: bob -- charlie [type="friend"];
       edge: charlie -- diana [type="friend"];

       // Rule: Recommend friends of friends
       rule recommend_friends {
           lhs {
               node A :user;
               node B :user;
               node C :user;
               edge: A -- B [type="friend"];
               edge: B -- C [type="friend"];
           }
           rhs {
               node A :user;
               node B :user;
               node C :user;
               edge: A -- B [type="friend"];
               edge: B -- C [type="friend"];
               edge: A -- C [type="recommendation", confidence=0.7];
           }
       }

       apply recommend_friends 5 times;
   }

Infrastructure Examples
-----------------------

Data Center Network
~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph datacenter {
       // Core infrastructure
       node primary_dc :datacenter [location="us-east-1", capacity=1000];
       node backup_dc :datacenter [location="us-west-2", capacity=500];

       // Backbone connection
       edge: primary_dc -- backup_dc [
           type="backbone",
           bandwidth="10Gbps",
           latency=50
       ];

       // Generate server grid
       generate grid {
           rows: 5;
           cols: 8;
           prefix: "server";
       }

       // Assign servers to primary datacenter
       rule assign_to_datacenter {
           lhs { node S; }
           rhs {
               node S :server [status="active", cpu=16, memory="64GB"];
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

Microservices Architecture
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph microservices {
       // API Gateway
       node gateway :service [type="api_gateway", port=80];

       // Core services
       node auth :service [type="authentication", port=8001];
       node user :service [type="user_management", port=8002];
       node order :service [type="order_processing", port=8003];
       node payment :service [type="payment", port=8004];

       // Databases
       node user_db :database [type="postgresql"];
       node order_db :database [type="mongodb"];
       node cache :database [type="redis"];

       // Service connections
       edge: gateway -> auth [type="authenticates"];
       edge: gateway -> user [type="routes"];
       edge: gateway -> order [type="routes"];
       edge: order -> payment [type="processes_payment"];

       // Database connections
       edge: user -> user_db [type="stores_data"];
       edge: order -> order_db [type="stores_data"];
       edge: auth -> cache [type="caches_sessions"];

       // Rule: Add monitoring to all services
       rule add_monitoring {
           lhs { node S :service; }
           rhs {
               node S :service [monitored=true];
               node M :monitor;
               edge: S -> M [type="monitored_by"];
           }
       }

       apply add_monitoring 5 times;
   }

Organizational Examples
-----------------------

Corporate Hierarchy
~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph organization {
       // Executive level
       node ceo :executive [title="CEO", level=1];
       node cto :executive [title="CTO", level=2];
       node cfo :executive [title="CFO", level=2];

       // Reporting structure
       edge: cto -> ceo [type="reports_to"];
       edge: cfo -> ceo [type="reports_to"];

       // Generate team structure
       generate tree {
           branching: 4;
           depth: 3;
           prefix: "emp";
       }

       // Assign management roles
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

       apply assign_manager_role 15 times;

       // Add department assignments
       rule assign_departments {
           lhs { node E :employee; }
           rhs { node E :employee [department="engineering"]; }
       }

       apply assign_departments 10 times;
   }

Project Team Structure
~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph project_teams {
       // Project leads
       node frontend_lead :lead [project="frontend", experience=5];
       node backend_lead :lead [project="backend", experience=7];
       node devops_lead :lead [project="devops", experience=6];

       // Generate team members
       generate star {
           nodes: 6;
           prefix: "frontend_dev";
       }

       generate star {
           nodes: 8;
           prefix: "backend_dev";
       }

       generate star {
           nodes: 4;
           prefix: "devops_eng";
       }

       // Assign team members to leads
       rule assign_to_frontend {
           lhs { node D; }
           rhs {
               node D :developer [team="frontend", skills="react"];
               node frontend_lead :lead;
               edge: D -> frontend_lead [type="reports_to"];
           }
       }

       apply assign_to_frontend 6 times;
   }

Transportation Examples
-----------------------

City Transportation Network
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph transportation {
       // Major cities
       node nyc :city [name="New York", population=8000000];
       node la :city [name="Los Angeles", population=4000000];
       node chicago :city [name="Chicago", population=2700000];

       // Interstate highways
       edge: nyc -- chicago [
           type="interstate",
           route="I-80",
           distance=790,
           speed_limit=70
       ];

       // Generate regional towns
       generate complete {
           nodes: 10;
           prefix: "town";
       }

       // Connect towns to major cities
       rule connect_to_city {
           lhs { node T; }
           rhs {
               node T :town [population=50000];
               node nyc :city;
               edge: T -- nyc [type="highway", distance=100];
           }
       }

       apply connect_to_city 10 times;

       // Add traffic data
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
                   peak_hours="7-9,17-19"
               ];
           }
       }

       apply add_traffic_data 15 times;
   }

Public Transit System
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph transit_system {
       // Generate subway line (path)
       generate path {
           nodes: 12;
           prefix: "station";
       }

       // Add central hub
       node central_hub :station [type="major_hub"];

       // Connect hub to line
       rule connect_to_hub {
           lhs { node S; }
           rhs {
               node S :station [type="regular"];
               node central_hub :station;
               edge: S -- central_hub [type="express_line"];
           }
       }

       apply connect_to_hub 3 times;

       // Add bus connections
       rule add_bus_service {
           lhs { node S :station; }
           rhs {
               node S :station;
               node B :bus_stop;
               edge: S -- B [type="bus_connection"];
           }
       }

       apply add_bus_service 8 times;
   }

Biological Network Examples
---------------------------

Protein Interaction Network
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph protein_network {
       // Key proteins
       node p53 :protein [function="tumor_suppressor"];
       node mdm2 :protein [function="ubiquitin_ligase"];
       node rb :protein [function="cell_cycle"];

       // Known interactions
       edge: p53 -- mdm2 [type="inhibits", strength=0.9];
       edge: p53 -- rb [type="activates", strength=0.7];

       // Generate additional proteins
       generate barabasi_albert {
           nodes: 50;
           edges_per_node: 3;
           prefix: "protein";
       }

       // Add functional annotations
       rule annotate_proteins {
           lhs { node P; }
           rhs { node P :protein [annotated=true, pathway="unknown"]; }
       }

       apply annotate_proteins 50 times;

       // Infer interactions based on co-expression
       rule infer_interactions {
           lhs {
               node P1 :protein;
               node P2 :protein;
               node P3 :protein;
               edge: P1 -- P3;
               edge: P2 -- P3;
           }
           rhs {
               node P1 :protein;
               node P2 :protein;
               node P3 :protein;
               edge: P1 -- P3;
               edge: P2 -- P3;
               edge: P1 -- P2 [type="inferred", confidence=0.6];
           }
       }

       apply infer_interactions 10 times;
   }

Game Development Examples
-------------------------

Game World Map
~~~~~~~~~~~~~~

.. code-block:: ggl

   graph game_world {
       // Starting area
       node village :location [type="safe_zone", level=1];

       // Generate world map as grid
       generate grid {
           rows: 8;
           cols: 8;
           prefix: "area";
       }

       // Add special locations
       node dungeon :location [type="dungeon", level=5, boss=true];
       node castle :location [type="castle", level=10];

       // Connect special locations
       edge: village -- dungeon [type="path", danger=3];
       edge: dungeon -- castle [type="bridge", danger=7];

       // Add difficulty progression
       rule add_difficulty {
           lhs { node L; }
           rhs { node L :location [difficulty=1, explored=false]; }
       }

       apply add_difficulty 64 times;

       // Create quest chains
       rule create_quest_chain {
           lhs {
               node L1 :location;
               node L2 :location;
               edge: L1 -- L2;
           }
           rhs {
               node L1 :location;
               node L2 :location;
               edge: L1 -- L2;
               edge: L1 -> L2 [type="quest_leads_to"];
           }
       }

       apply create_quest_chain 20 times;
   }

Financial Network Examples
--------------------------

Trading Network
~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph trading_network {
       // Major exchanges
       node nyse :exchange [location="New York"];
       node nasdaq :exchange [location="New York"];
       node lse :exchange [location="London"];

       // Generate trading firms
       generate star {
           nodes: 20;
           prefix: "firm";
       }

       // Connect firms to exchanges
       rule connect_to_exchange {
           lhs { node F; }
           rhs {
               node F :trading_firm [capital=1000000];
               node nyse :exchange;
               edge: F -> nyse [type="trades_on"];
           }
       }

       apply connect_to_exchange 20 times;

       // Create trading relationships
       rule create_trading_pairs {
           lhs {
               node F1 :trading_firm;
               node F2 :trading_firm;
           }
           rhs {
               node F1 :trading_firm;
               node F2 :trading_firm;
               edge: F1 -- F2 [type="trading_partner", volume=50000];
           }
       }

       apply create_trading_pairs 30 times;
   }

Complex Multi-Domain Example
----------------------------

Smart City Infrastructure
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: ggl

   graph smart_city {
       // Transportation layer
       generate grid {
           rows: 6;
           cols: 6;
           prefix: "intersection";
       }

       // Energy grid
       node power_plant :facility [type="power", capacity=1000];
       node substation1 :facility [type="substation"];
       node substation2 :facility [type="substation"];

       edge: power_plant -> substation1 [type="power_line"];
       edge: power_plant -> substation2 [type="power_line"];

       // Communication network
       generate star {
           nodes: 10;
           prefix: "cell_tower";
       }

       // IoT sensors
       rule add_sensors {
           lhs { node I; }
           rhs {
               node I :intersection [traffic_light=true];
               node S :sensor [type="traffic"];
               edge: I -> S [type="monitors"];
           }
       }

       apply add_sensors 36 times;

       // Connect sensors to communication network
       rule connect_sensors {
           lhs {
               node S :sensor;
               node T;
           }
           rhs {
               node S :sensor [connected=true];
               node T :cell_tower;
               edge: S -> T [type="transmits_data"];
           }
       }

       apply connect_sensors 36 times;

       // Add smart buildings
       rule add_buildings {
           lhs { node I :intersection; }
           rhs {
               node I :intersection;
               node B :building [type="smart", floors=10];
               edge: I -- B [type="located_at"];
           }
       }

       apply add_buildings 20 times;

       // Connect buildings to power grid
       rule connect_to_power {
           lhs {
               node B :building;
               node S :facility [type="substation"];
           }
           rhs {
               node B :building [powered=true];
               node S :facility;
               edge: B -> S [type="power_consumer"];
           }
       }

       apply connect_to_power 20 times;
   }

This comprehensive example shows how GGL can model complex, multi-layered systems by combining different graph structures and using rules to establish relationships between different domains (transportation, energy, communication, and buildings).
