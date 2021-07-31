use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyObjectProtocol};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tokens {
    EOF,

    Function,
    Class,
    Struct,
    TypeName,
    Operator,

    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,

    Dot,
    Comma,

    Assignment,
    Semicolon,
    Colon,
    Tag,
    Reference,
    Question,
    At,
    Percent,
    Bang,
    BackSlash,

    Space,
    Tab,
    Newline,

    SingleQuote,
    DoubleQuote,
    Identifier,
    NumericLiteral,
}

impl Tokens {
    fn from_i32(value: i32) -> Tokens {
        match value {
            0 => Tokens::EOF,

            1 => Tokens::Function,
            2 => Tokens::Class,
            3 => Tokens::Struct,
            4 => Tokens::TypeName,
            5 => Tokens::Operator,

            6 => Tokens::LeftBrace,
            7 => Tokens::RightBrace,
            8 => Tokens::LeftBracket,
            9 => Tokens::RightBracket,
            10 => Tokens::LeftParen,
            11 => Tokens::RightParen,

            12 => Tokens::Dot,
            13 => Tokens::Comma,

            14 => Tokens::Assignment,
            15 => Tokens::Semicolon,
            16 => Tokens::Colon,
            17 => Tokens::Tag,
            18 => Tokens::Reference,
            19 => Tokens::Question,
            20 => Tokens::At,
            21 => Tokens::Percent,
            22 => Tokens::Bang,
            23 => Tokens::BackSlash,

            24 => Tokens::Space,
            25 => Tokens::Tab,
            26 => Tokens::Newline,

            27 => Tokens::SingleQuote,
            28 => Tokens::DoubleQuote,
            29 => Tokens::Identifier,
            30 => Tokens::NumericLiteral,

            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[pyfunction]
fn token_num_to_name(num: i32) -> String {
    format!("{:?}", Tokens::from_i32(num))
}

#[pyclass]
#[derive(PartialEq, Debug)]
struct Token {
    part: String,
    token: Tokens,
}

#[pyproto]
impl PyObjectProtocol for Token {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "Token(\"{}\", {:?}: {})",
            self.part, self.token, self.token as i32
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Token(\"{}\", {:?}: {})",
            self.part, self.token, self.token as i32
        ))
    }
}

#[pymethods]
impl Token {
    #[new]
    fn new(part: String, token: i32) -> Self {
        Token {
            part,
            token: Tokens::from_i32(token),
        }
    }
    #[getter]
    fn part(&self) -> PyResult<String> {
        Ok(self.part.clone())
    }
    #[getter]
    fn token(&self) -> PyResult<i32> {
        Ok(self.token as i32)
    }

    #[setter]
    fn set_part(&mut self, value: String) -> PyResult<()> {
        self.part = value;
        Ok(())
    }

    #[setter]
    fn set_token(&mut self, value: i32) -> PyResult<()> {
        self.token = Tokens::from_i32(value);
        Ok(())
    }
}

#[pyfunction]
fn is_char_symbol(ch: char) -> bool {
    match ch {
        '[' | ']' | '{' | '}' | '(' | ')' | '.' | ',' | ':' | ';' | '=' | '\'' | '\"' => true,
        _ => false,
    }
}

#[pyfunction]
fn is_char_operator(ch: char) -> bool {
    match ch {
        '+' | '-' | '*' | '/' | '^' | '>' | '<' => true,
        _ => false,
    }
}

#[pyfunction]
fn is_char_whitespace(ch: char) -> bool {
    match ch {
        '\t' | ' ' | '\n' => true,
        _ => false,
    }
}

#[pyfunction]
fn is_char_numeric(ch: char) -> bool {
    return ch.is_digit(10);
}

#[pyfunction]
fn is_single_quote(ch: char) -> bool {
    return ch == '\'';
}

#[pyfunction]
fn is_double_quote(ch: char) -> bool {
    return ch == '\"';
}

#[pyfunction]
fn ends_token(cur: char, next: char) -> bool {
    if is_char_whitespace(next) {
        return true;
    }
    if is_char_symbol(cur) {
        return true;
    }
    if is_char_symbol(next) {
        return true;
    }
    if is_char_operator(cur) {
        return true;
    }
    if is_char_operator(next) {
        return true;
    }
    if is_char_whitespace(cur) {
        return false;
    }
    return false;
}

#[pyfunction]
fn is_part_numeric(part: &str) -> bool {
    for c in part.chars() {
        if is_char_numeric(c) {
            return true;
        }
    }
    return false;
}

#[pyfunction]
fn tokenize(part: &str) -> Token {
    let mut token = match part {
        "fn" | "fun" | "func" | "function" => Tokens::Function,
        "class" | "cls" => Tokens::Class,
        "struct" => Tokens::Struct,
        "int" | "float" | "bool" | "double" | "long" | "str" | "string" => Tokens::TypeName,
        "+" | "-" | "*" | "/" | "^" | ">" | "<" => Tokens::Operator,

        "{" => Tokens::LeftBrace,
        "}" => Tokens::RightBrace,
        "[" => Tokens::LeftBracket,
        "]" => Tokens::RightBracket,
        "(" => Tokens::LeftParen,
        ")" => Tokens::RightParen,

        "." => Tokens::Dot,
        "," => Tokens::Comma,

        "=" => Tokens::Assignment,
        ";" => Tokens::Semicolon,
        ":" => Tokens::Colon,
        "#" => Tokens::Tag,
        "&" => Tokens::Reference,
        "?" => Tokens::Question,
        "@" => Tokens::At,
        "%" => Tokens::Percent,
        "!" => Tokens::Bang,
        "\\" => Tokens::BackSlash,

        " " => Tokens::Space,
        "\t" => Tokens::Tab,
        "\n" => Tokens::Newline,

        "\'" => Tokens::SingleQuote,
        "\"" => Tokens::DoubleQuote,
        _ => Tokens::Identifier,
    };

    if token == Tokens::Identifier {
        if is_part_numeric(part) {
            token = Tokens::NumericLiteral;
        }
    }

    let part = String::from(part);
    return Token { part, token };
}

#[pyclass]
struct Lexer {
    index: usize,
    length: usize,
    chars: Vec<char>,
    eof: bool,
}

#[pymethods]
impl Lexer {
    /// The EOF code for python to access
    #[classattr]
    const EOF: i32 = Tokens::EOF as i32;

    #[new]
    fn new(chars: String) -> Self {
        let length = chars.clone().len();
        let chars: Vec<char> = chars.chars().collect();
        Lexer {
            index: 0,
            chars,
            length,
            eof: false,
        }
    }

    fn next(&mut self) -> Option<Token> {
        let mut buffer = String::new();
        loop {
            if self.eof {
                return Some(Token {
                    part: "".to_string(),
                    token: Tokens::EOF,
                });
            }
            if self.index + 1 == self.length {
                self.eof = true;
                buffer.push(self.chars[self.index]);
                return Some(tokenize(&buffer));
            }

            let current: char = self.chars[self.index];
            let next: char = self.chars[self.index + 1];

            if !is_char_whitespace(current) {
                buffer.push(current);
                if ends_token(current, next) {
                    self.index += 1;
                    return Some(tokenize(&buffer));
                }
            }

            self.index += 1;
        }
    }
}

#[pymodule]
fn plrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_char_symbol, m)?)?;
    m.add_function(wrap_pyfunction!(is_char_operator, m)?)?;
    m.add_function(wrap_pyfunction!(is_char_whitespace, m)?)?;
    m.add_function(wrap_pyfunction!(is_char_numeric, m)?)?;
    m.add_function(wrap_pyfunction!(is_single_quote, m)?)?;
    m.add_function(wrap_pyfunction!(is_double_quote, m)?)?;
    m.add_function(wrap_pyfunction!(ends_token, m)?)?;
    m.add_function(wrap_pyfunction!(is_part_numeric, m)?)?;

    m.add_function(wrap_pyfunction!(token_num_to_name, m)?)?;

    m.add_class::<Lexer>()?;
    m.add_class::<Token>()?;

    Ok(())
}
