Contributing
============

We welcome contributions to the Graph Generation Language project! This guide will help you get started with contributing code, documentation, or other improvements.

Getting Started
---------------

Development Environment
~~~~~~~~~~~~~~~~~~~~~~~

1. **Install Rust**: Make sure you have Rust 1.70 or later installed
2. **Clone the repository**:

   .. code-block:: bash

      git clone https://github.com/ocasazza/graph-generation-language.git
      cd graph-generation-language

3. **Build the project**:

   .. code-block:: bash

      cargo build

4. **Run tests**:

   .. code-block:: bash

      cargo test

Project Structure
~~~~~~~~~~~~~~~~~

.. code-block:: text

   ├── src/
   │   ├── lib.rs          # Main library interface
   │   ├── parser.rs       # GGL language parser
   │   ├── types.rs        # Core graph data structures
   │   ├── generators.rs   # Built-in graph generators
   │   ├── rules.rs        # Rule application engine
   │   └── ggl.pest        # Grammar definition
   ├── tests/              # Test suites
   │   ├── parser_tests.rs
   │   ├── rule_tests.rs
   │   ├── generator_tests.rs
   │   └── integration_tests.rs
   ├── docs/               # Sphinx documentation
   └── examples/           # Example GGL programs

Types of Contributions
----------------------

Code Contributions
~~~~~~~~~~~~~~~~~~

**Bug Fixes**
   - Fix parsing errors
   - Resolve rule application issues
   - Improve generator correctness

**New Features**
   - Add new graph generators
   - Implement additional rule patterns
   - Enhance language syntax

**Performance Improvements**
   - Optimize pattern matching
   - Improve memory usage
   - Speed up graph generation

Documentation
~~~~~~~~~~~~~

**API Documentation**
   - Add missing docstrings
   - Improve code examples
   - Update type signatures

**User Guides**
   - Write tutorials
   - Create example programs
   - Improve getting started guide

**Reference Documentation**
   - Update language reference
   - Document new features
   - Fix typos and errors

Testing
~~~~~~~

**Unit Tests**
   - Add tests for new features
   - Improve test coverage
   - Fix failing tests

**Integration Tests**
   - Test complete workflows
   - Verify example programs
   - Test error handling

**Performance Tests**
   - Benchmark generators
   - Test large graph handling
   - Memory usage tests

Development Workflow
--------------------

1. **Fork the Repository**

   Create a fork of the repository on GitHub.

2. **Create a Feature Branch**

   .. code-block:: bash

      git checkout -b feature/your-feature-name

3. **Make Your Changes**

   Follow the coding standards and write tests for new functionality.

4. **Run Tests**

   .. code-block:: bash

      cargo test
      cargo clippy
      cargo fmt

5. **Commit Your Changes**

   .. code-block:: bash

      git add .
      git commit -m "Add your descriptive commit message"

6. **Push to Your Fork**

   .. code-block:: bash

      git push origin feature/your-feature-name

7. **Create a Pull Request**

   Open a pull request on GitHub with a clear description of your changes.

Coding Standards
----------------

Rust Style
~~~~~~~~~~~

- Follow standard Rust formatting (use ``cargo fmt``)
- Use ``cargo clippy`` to catch common issues
- Write idiomatic Rust code
- Add documentation comments for public APIs

Code Organization
~~~~~~~~~~~~~~~~~

- Keep functions focused and small
- Use meaningful variable and function names
- Organize code into logical modules
- Separate concerns appropriately

Error Handling
~~~~~~~~~~~~~~

- Use ``Result<T, String>`` for fallible operations
- Provide descriptive error messages
- Handle edge cases gracefully
- Document error conditions

Testing Guidelines
------------------

Test Structure
~~~~~~~~~~~~~~

- Place unit tests in the same file as the code they test
- Use integration tests for end-to-end functionality
- Group related tests in modules

Test Naming
~~~~~~~~~~~

- Use descriptive test names that explain what is being tested
- Follow the pattern ``test_<functionality>_<scenario>``
- Example: ``test_complete_graph_generation``

Test Coverage
~~~~~~~~~~~~~

- Aim for high test coverage of new code
- Test both success and failure cases
- Include edge cases and boundary conditions

Example Test
~~~~~~~~~~~~

.. code-block:: rust

   #[test]
   fn test_node_creation_with_attributes() {
       let node = Node::new("test".to_string())
           .with_type("person".to_string())
           .with_metadata("age".to_string(), MetadataValue::Integer(30));

       assert_eq!(node.id, "test");
       assert_eq!(node.r#type, "person");
       assert_eq!(node.metadata.get("age"), Some(&MetadataValue::Integer(30)));
   }

Documentation Standards
-----------------------

Code Documentation
~~~~~~~~~~~~~~~~~~

- Add doc comments for all public functions and types
- Include examples in documentation
- Document parameters and return values
- Explain complex algorithms

Example Documentation
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   /// Creates a complete graph where every node is connected to every other node.
   ///
   /// # Parameters
   ///
   /// * `params` - Generator parameters including:
   ///   - `nodes`: Number of nodes to generate (required)
   ///   - `prefix`: Node name prefix (optional, default: "n")
   ///   - `directed`: Whether edges should be directed (optional, default: false)
   ///
   /// # Returns
   ///
   /// A `Result` containing the generated graph or an error message.
   ///
   /// # Examples
   ///
   /// ```rust
   /// let mut params = HashMap::new();
   /// params.insert("nodes".to_string(), MetadataValue::Integer(4));
   /// let graph = generate_complete(&params)?;
   /// assert_eq!(graph.node_count(), 4);
   /// assert_eq!(graph.edge_count(), 6); // n*(n-1)/2 for undirected
   /// ```
   pub fn generate_complete(params: &HashMap<String, MetadataValue>) -> Result<Graph, String> {
       // Implementation...
   }

Sphinx Documentation
~~~~~~~~~~~~~~~~~~~~

- Use reStructuredText format
- Include code examples
- Cross-reference related sections
- Keep documentation up to date

Adding New Features
-------------------

Graph Generators
~~~~~~~~~~~~~~~~

To add a new graph generator:

1. **Implement the generator function** in ``src/generators.rs``
2. **Add it to the registry** in the ``get_generator`` function
3. **Write comprehensive tests** in ``tests/generator_tests.rs``
4. **Document the generator** in ``docs/source/generators.rst``

Example generator implementation:

.. code-block:: rust

   pub fn generate_wheel(params: &HashMap<String, MetadataValue>) -> Result<Graph, String> {
       let n = get_param_int(params, "nodes")?;
       let prefix = get_param_string(params, "prefix", "n");

       if n < 4 {
           return Err("Wheel graph requires at least 4 nodes".to_string());
       }

       let mut graph = Graph::new();

       // Add center node
       let center = format!("{}0", prefix);
       graph.add_node(Node::new(center.clone()));

       // Add rim nodes and connect to center
       for i in 1..n {
           let node_id = format!("{}{}", prefix, i);
           graph.add_node(Node::new(node_id.clone()));

           // Connect to center
           let edge_id = format!("e0_{}", i);
           graph.add_edge(Edge::new(edge_id, center.clone(), node_id));
       }

       // Connect rim nodes in a cycle
       for i in 1..n {
           let source = format!("{}{}", prefix, i);
           let target = format!("{}{}", prefix, if i == n - 1 { 1 } else { i + 1 });
           let edge_id = format!("e{}_{}", i, if i == n - 1 { 1 } else { i + 1 });
           graph.add_edge(Edge::new(edge_id, source, target));
       }

       Ok(graph)
   }

Language Extensions
~~~~~~~~~~~~~~~~~~~

To extend the GGL language:

1. **Update the grammar** in ``src/ggl.pest``
2. **Modify the parser** in ``src/parser.rs``
3. **Update the AST types** if needed
4. **Add processing logic** in the appropriate module
5. **Write tests** for the new syntax
6. **Update documentation**

Rule System Enhancements
~~~~~~~~~~~~~~~~~~~~~~~~~

To improve the rule system:

1. **Identify the enhancement** (new pattern types, optimization, etc.)
2. **Modify** ``src/rules.rs`` as needed
3. **Add comprehensive tests** in ``tests/rule_tests.rs``
4. **Update documentation** in ``docs/source/transformation-rules.rst``

Submitting Pull Requests
-------------------------

Pull Request Guidelines
~~~~~~~~~~~~~~~~~~~~~~~

- **Clear Description**: Explain what your PR does and why
- **Small, Focused Changes**: Keep PRs focused on a single feature or fix
- **Tests Included**: Add tests for new functionality
- **Documentation Updated**: Update docs for user-facing changes
- **Clean History**: Use meaningful commit messages

PR Template
~~~~~~~~~~~

When creating a pull request, include:

.. code-block:: text

   ## Description
   Brief description of the changes

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Documentation update
   - [ ] Performance improvement
   - [ ] Other (please describe)

   ## Testing
   - [ ] Tests pass locally
   - [ ] New tests added for new functionality
   - [ ] Manual testing performed

   ## Documentation
   - [ ] Documentation updated
   - [ ] Examples added/updated
   - [ ] API docs updated

   ## Checklist
   - [ ] Code follows project style guidelines
   - [ ] Self-review completed
   - [ ] Comments added for complex code
   - [ ] No breaking changes (or clearly documented)

Review Process
~~~~~~~~~~~~~~

1. **Automated Checks**: CI will run tests and linting
2. **Code Review**: Maintainers will review your code
3. **Feedback**: Address any requested changes
4. **Approval**: Once approved, your PR will be merged

Getting Help
------------

Communication Channels
~~~~~~~~~~~~~~~~~~~~~~

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Pull Request Comments**: For code-specific questions

Reporting Issues
~~~~~~~~~~~~~~~~

When reporting bugs:

1. **Search existing issues** first
2. **Provide a clear title** and description
3. **Include steps to reproduce** the issue
4. **Add relevant code samples** or error messages
5. **Specify your environment** (OS, Rust version, etc.)

Issue Template
~~~~~~~~~~~~~~

.. code-block:: text

   ## Bug Description
   Clear description of the bug

   ## Steps to Reproduce
   1. Step one
   2. Step two
   3. Step three

   ## Expected Behavior
   What you expected to happen

   ## Actual Behavior
   What actually happened

   ## Environment
   - OS: [e.g., macOS 12.0]
   - Rust version: [e.g., 1.70.0]
   - GGL version: [e.g., 0.1.0]

   ## Additional Context
   Any other relevant information

Recognition
-----------

Contributors will be recognized in:

- **CONTRIBUTORS.md** file
- **Release notes** for significant contributions
- **Documentation credits** for documentation improvements

Thank you for contributing to the Graph Generation Language project! Your contributions help make GGL better for everyone.
