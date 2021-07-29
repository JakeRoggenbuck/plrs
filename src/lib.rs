use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyObjectProtocol};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tokens {
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

    Plus,
    Minus,
    Star,
    Slash,
    Carrot,
    Greater,
    Less,

    Space,
    Tab,
    Newline,

    Comment,
    SingleQuote,
    DoubleQuote,
    Identifier,
    NumericLiteral,
}

impl Tokens {
    fn from_i32(value: i32) -> Tokens {
        match value {
            1 => Tokens::LeftBrace,
            2 => Tokens::RightBrace,
            3 => Tokens::LeftBracket,
            4 => Tokens::RightBracket,
            5 => Tokens::LeftParen,
            6 => Tokens::RightParen,
            7 => Tokens::Dot,
            8 => Tokens::Comma,
            9 => Tokens::Assignment,
            10 => Tokens::Semicolon,

            11 => Tokens::Colon,
            12 => Tokens::Tag,
            13 => Tokens::Reference,
            14 => Tokens::Question,

            15 => Tokens::Plus,
            16 => Tokens::Minus,
            17 => Tokens::Star,
            18 => Tokens::Slash,
            19 => Tokens::Carrot,
            20 => Tokens::Greater,
            21 => Tokens::Less,

            22 => Tokens::Space,
            23 => Tokens::Tab,
            24 => Tokens::Newline,

            25 => Tokens::Comment,
            26 => Tokens::SingleQuote,
            27 => Tokens::DoubleQuote,
            28 => Tokens::Identifier,
            29 => Tokens::NumericLiteral,
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
        Ok(format!("Token(\"{}\", {:?})", self.part, self.token))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Token(\"{}\", {:?})", self.part, self.token))
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

        "+" => Tokens::Plus,
        "-" => Tokens::Minus,
        "*" => Tokens::Star,
        "/" => Tokens::Slash,
        "^" => Tokens::Carrot,
        ">" => Tokens::Greater,
        "<" => Tokens::Less,

        " " => Tokens::Space,
        "\t" => Tokens::Tab,
        "\n" => Tokens::Newline,

        "~" => Tokens::Comment,
        "\'" => Tokens::SingleQuote,
        "\"" => Tokens::DoubleQuote,
        _ => Tokens::Identifier,
    };

    if token == Tokens::Identifier {
        if is_part_numeric(part) {
            token = Tokens::Identifier;
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

    #[staticmethod]
    fn lexer() -> bool {
        return true;
    }

    fn next(&mut self) -> Option<Token> {
        let mut buffer = String::new();
        loop {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_char_symbol_test() {
        for i in ['[', ']', ')', '(', '.', ';'].iter() {
            assert!(is_char_symbol(*i));
        }
        for i in ['a', 'b', '7', '8'].iter() {
            assert!(!is_char_symbol(*i));
        }
    }

    #[test]
    fn is_char_operator_test() {
        for i in ['+', '-', '*', '^'].iter() {
            assert!(is_char_operator(*i));
        }

        for i in ['a', '(', '7', ']'].iter() {
            assert!(!is_char_operator(*i));
        }
    }

    #[test]
    fn is_char_whitespace_test() {
        for i in [' ', '\t', '\n'].iter() {
            assert!(is_char_whitespace(*i));
        }

        for i in ['a', '(', '7', ']'].iter() {
            assert!(!is_char_whitespace(*i));
        }
    }

    #[test]
    fn is_char_numeric_test() {
        for i in ['1', '3', '5', '9'].iter() {
            assert!(is_char_numeric(*i));
        }

        for i in ['a', '(', ']', '+', 'n'].iter() {
            assert!(!is_char_numeric(*i));
        }
    }

    #[test]
    fn is_double_quote_test() {
        assert!(is_double_quote('\'') == false);
        assert!(is_double_quote('\"') == true);
    }

    #[test]
    fn is_single_quote_test() {
        assert!(is_single_quote('\'') == true);
        assert!(is_single_quote('\"') == false);
    }
}
