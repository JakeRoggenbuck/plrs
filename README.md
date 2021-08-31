# plrs <a href="https://pypi.org/project/plrs/">![Version](https://img.shields.io/pypi/v/plrs)</a>
The multi-tool of lexical analysis and tokenization.

# Install
```
pip install plrs
```

# Build
```
maturin build
```
# Examples
- [zipfs-law](https://github.com/JakeRoggenbuck/zipfs-law)

# API
#### Global variables
```
EOF_TOKEN
```

#### Classes
```
Tokens
Settings
Token
  - part
  - token
  - set_part
  - set_token
  - __str__
  - __repr__

Lexer
  - new
  - char_forward
  - skip_over_char_set
  - next
```

#### Functions
```
is_char_symbol
is_char_operator
is_char_whitespace
is_char_numeric
is_single_quote
is_double_quote
ends_token
is_part_numeric
tokenize
```
