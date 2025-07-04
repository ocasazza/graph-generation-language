# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'Graph Generation Language'
copyright = '2024, GGL Contributors'
author = 'GGL Contributors'
release = '0.1.0'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.viewcode',
    'sphinx.ext.napoleon',
    'sphinx.ext.intersphinx',
    'sphinx.ext.todo',
    'sphinx.ext.coverage',
    'sphinx.ext.mathjax',
    'sphinx.ext.ifconfig',
    'sphinx.ext.githubpages',
    'myst_parser',
]

# Add support for Markdown files
source_suffix = {
    '.rst': None,
    '.md': 'myst_parser',
}

templates_path = ['_templates']
exclude_patterns = []

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# Theme options
html_theme_options = {
    'canonical_url': '',
    'analytics_id': '',
    'logo_only': False,
    'display_version': True,
    'prev_next_buttons_location': 'bottom',
    'style_external_links': False,
    'vcs_pageview_mode': '',
    'style_nav_header_background': '#2980B9',
    # Toc options
    'collapse_navigation': True,
    'sticky_navigation': True,
    'navigation_depth': 4,
    'includehidden': True,
    'titles_only': False
}

# Custom CSS
html_css_files = [
    'custom.css',
]

# -- Extension configuration -------------------------------------------------

# Napoleon settings
napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = False
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = False
napoleon_use_admonition_for_notes = False
napoleon_use_admonition_for_references = False
napoleon_use_ivar = False
napoleon_use_param = True
napoleon_use_rtype = True
napoleon_preprocess_types = False
napoleon_type_aliases = None
napoleon_attr_annotations = True

# Intersphinx mapping
intersphinx_mapping = {
    'python': ('https://docs.python.org/3/', None),
    'rust': ('https://doc.rust-lang.org/', None),
}

# MyST parser configuration
myst_enable_extensions = [
    "colon_fence",
    "deflist",
    "dollarmath",
    "fieldlist",
    "html_admonition",
    "html_image",
    "linkify",
    "replacements",
    "smartquotes",
    "strikethrough",
    "substitution",
    "tasklist",
]

# Code highlighting
pygments_style = 'sphinx'
highlight_language = 'rust'

# Add custom lexer for GGL syntax
def setup(app):
    from pygments.lexers import get_lexer_by_name
    from pygments.token import Keyword, Name, String, Number, Comment, Operator, Punctuation
    from pygments.lexer import RegexLexer

    class GGLLexer(RegexLexer):
        name = 'GGL'
        aliases = ['ggl']
        filenames = ['*.ggl']

        tokens = {
            'root': [
                (r'//.*?$', Comment.Single),
                (r'/\*.*?\*/', Comment.Multiline),
                (r'\b(graph|node|edge|rule|lhs|rhs|apply|times|generate)\b', Keyword),
                (r'\b(true|false)\b', Keyword.Constant),
                (r'"[^"]*"', String),
                (r'\b\d+(\.\d+)?\b', Number),
                (r'[a-zA-Z_][a-zA-Z0-9_]*', Name),
                (r'(->|--)', Operator),
                (r'[{}()\[\];:=,]', Punctuation),
                (r'\s+', Name),
            ]
        }

    app.add_lexer('ggl', GGLLexer)
