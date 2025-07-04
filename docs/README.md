# Graph Generation Language Documentation

This directory contains the Sphinx documentation for the Graph Generation Language (GGL) project.

## Building the Documentation

### Prerequisites

- Python 3.7 or later
- pip

### Setup

1. Install the required dependencies:

```bash
cd docs
pip install -r requirements.txt
```

### Building

To build the HTML documentation:

```bash
make html
```

The generated documentation will be in `build/html/`. Open `build/html/index.html` in your browser to view it.

### Development

For development, you can use these convenient commands:

```bash
# Build and serve the documentation locally
make serve

# This will:
# 1. Clean previous builds
# 2. Build the HTML documentation
# 3. Start a local server at http://localhost:8000
```

### Available Make Targets

- `make html` - Build HTML documentation
- `make clean` - Remove build directory
- `make html-dev` - Clean build and generate HTML
- `make serve` - Build and serve documentation locally
- `make install` - Install Python dependencies
- `make help` - Show all available targets

### Documentation Structure

```
docs/
├── source/
│   ├── index.rst              # Main documentation index
│   ├── getting-started.rst    # Getting started guide
│   ├── language-reference.rst # Complete language reference
│   ├── generators.rst         # Graph generators documentation
│   ├── transformation-rules.rst # Rules system documentation
│   ├── examples.rst           # Practical examples
│   ├── api-reference.rst      # API documentation
│   ├── contributing.rst       # Contributing guide
│   ├── conf.py               # Sphinx configuration
│   ├── _static/              # Static files (CSS, images)
│   └── _templates/           # Custom templates
├── requirements.txt          # Python dependencies
├── Makefile                 # Build automation
└── README.md               # This file
```

### Writing Documentation

The documentation uses reStructuredText (RST) format. Here are some quick tips:

#### Code Blocks

```rst
.. code-block:: ggl

   graph example {
       node alice :person;
       node bob :person;
       edge: alice -- bob;
   }
```

#### Cross-References

```rst
See :doc:`getting-started` for installation instructions.
```

#### Admonitions

```rst
.. note::
   This is a note.

.. warning::
   This is a warning.

.. tip::
   This is a tip.
```

### Syntax Highlighting

The documentation includes custom syntax highlighting for GGL code blocks. Use the `ggl` language identifier in code blocks to enable it:

```rst
.. code-block:: ggl

   // Your GGL code here
```

### Themes and Styling

The documentation uses the Read the Docs theme with custom CSS for improved styling. The custom styles are defined in `source/_static/custom.css`.

### Publishing

The documentation can be published to various platforms:

1. **GitHub Pages**: Build the HTML and push to a `gh-pages` branch
2. **Read the Docs**: Connect your repository to Read the Docs for automatic building
3. **Netlify**: Deploy the `build/html` directory

### Troubleshooting

**Build Errors**

If you encounter build errors:

1. Check that all dependencies are installed: `pip install -r requirements.txt`
2. Clean the build directory: `make clean`
3. Try building again: `make html`

**Missing Extensions**

If Sphinx complains about missing extensions, make sure all dependencies in `requirements.txt` are installed.

**Syntax Errors**

Check your RST syntax. Common issues:
- Inconsistent indentation
- Missing blank lines around code blocks
- Incorrect directive syntax

### Contributing to Documentation

When contributing to the documentation:

1. Follow the existing structure and style
2. Test your changes by building locally
3. Use clear, concise language
4. Include code examples where appropriate
5. Update the table of contents if adding new sections

For more details on contributing, see the [Contributing Guide](source/contributing.rst).
